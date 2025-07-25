use serde_json as json;
use std::{
    env::{self, args},
    fs::{self, create_dir},
    io::{self, Read, stdin},
    mem,
    path::{self, PathBuf},
    process::ExitCode,
};
use thiserror::Error;

type Result<T = (), E = Error> = std::result::Result<T, E>;

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        mem::discriminant(self) == mem::discriminant(other)
    }
}

/// Typed errors used for tests. PartialEq is implemented for tests.
#[derive(Debug, Error)]
enum Error {
    #[error("the JSON is not an object")]
    JsonIsNotObject,

    #[error("couldn't convert stdin to JSON")]
    CouldNotParseJson,

    #[error("the key {name:#?} under {context:#?} must have exactly one path component")]
    MultiplePathComponents { name: String, context: PathBuf },

    #[error("the key {name:#?} under {context:#?} is a non-regular path component")]
    NotRegularComponent { name: String, context: PathBuf },

    #[error("couldn't create a {kind} at path {context:#?}: {e:#?}")]
    Create {
        e: io::Error,
        context: PathBuf,
        kind: &'static str,
    },

    #[error("couldn't set the current dir to the newly created path {context:#?}: {e:#?}")]
    ChangeDir { context: PathBuf, e: io::Error },

    #[error("couldn't set the current dir {context:#?} to the dir above it: {e:#?}")]
    ChangeDirUp { context: PathBuf, e: io::Error },

    #[cfg(not(unix))]
    #[error("found a JSON value which isn't an object or a string at path {context:#?}")]
    InvalidJsonPart { context: PathBuf },

    #[cfg(unix)]
    #[error("found a JSON value which isn't an object, an array, or a string at path {context:#?}")]
    InvalidJsonPart { context: PathBuf },

    #[cfg(unix)]
    #[error("found a JSON array that doesn't contain a single string at path {context:#?}")]
    NotSingleStringArray { context: PathBuf },
}

// TODO:
// Consider adding an option not to overwrite old files?
// To be clear, directory <-> file conversion throw an error, but file <-> file don't.
//
/// Converts a JSON object to a directory, recursively.
fn json_object_to_dir_rec(context: &mut PathBuf, object: json::Map<String, json::Value>) -> Result {
    for (name, value) in object {
        let path_component: PathBuf = name.clone().into();

        // Check the path.
        {
            let components: Vec<path::Component> = path_component.components().collect();

            let [component] = components[..] else {
                return Err(Error::MultiplePathComponents {
                    name,
                    context: context.clone(),
                });
            };

            let path::Component::Normal(_) = component else {
                return Err(Error::NotRegularComponent {
                    name,
                    context: context.clone(),
                });
            };
        }

        context.push(path_component.clone());

        match value {
            // Create a regular file.
            json::Value::String(content) => {
                if let Err(e) = fs::write(&path_component, content) {
                    return Err(Error::Create {
                        e,
                        context: context.clone(),
                        kind: "regular file",
                    });
                };
            }

            // Create a directory.
            json::Value::Object(subdir) => {
                if let Err(e) = create_dir(&path_component) {
                    match e.kind() {
                        // For regular files, this is fine.
                        // For directories, this is fine.
                        // For symlinks, this is fine, chdir will fail.
                        std::io::ErrorKind::AlreadyExists => {}
                        _ => {
                            return Err(Error::Create {
                                e,
                                context: context.clone(),
                                kind: "directory",
                            });
                        }
                    }
                }

                if let Err(e) = env::set_current_dir(&path_component) {
                    return Err(Error::ChangeDir {
                        context: context.clone(),
                        e,
                    });
                };

                // TODO: consider using a queue instead of recursion to avoid stack overflows?
                json_object_to_dir_rec(context, subdir)?;

                if let Err(e) = env::set_current_dir(path::Component::ParentDir) {
                    return Err(Error::ChangeDirUp {
                        context: context.clone(),
                        e,
                    });
                };
            }

            // Create a symlink.
            // FIXME: add documentation for this feature.
            #[cfg(unix)]
            json::Value::Array(vec) => match &vec[..] {
                [json::Value::String(target)] => {
                    use std::os::unix;

                    if let Err(e) = unix::fs::symlink(target, &path_component) {
                        return Err(Error::Create {
                            e,
                            context: context.clone(),
                            kind: "symlink",
                        });
                    };
                }
                _ => {
                    return Err(Error::NotSingleStringArray {
                        context: context.clone(),
                    });
                }
            },

            _ => {
                return Err(Error::InvalidJsonPart {
                    context: context.clone(),
                });
            }
        }

        context.pop();
    }

    Ok(())
}

fn current_dir() -> PathBuf {
    path::Component::CurDir.as_os_str().into()
}

fn json_object_to_dir(object: json::Map<String, json::Value>) -> Result {
    json_object_to_dir_rec(&mut current_dir(), object)
}

fn run(string: &str) -> Result {
    let Ok(json) = json::from_str(string) else {
        return Err(Error::CouldNotParseJson);
    };

    let json::Value::Object(object) = json else {
        return Err(Error::JsonIsNotObject);
    };

    json_object_to_dir(object)
}

fn main() -> ExitCode {
    if args().len() != 1 {
        eprintln!("Usage: json-to-dir < file.json");
        return ExitCode::FAILURE;
    }

    let mut string = String::new();

    if stdin().read_to_string(&mut string).is_err() {
        eprintln!("Error: couldn't read stdin.");
        return ExitCode::FAILURE;
    }

    match run(&string) {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Error: {e}.");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use std::error;

    use super::*;

    fn make_dummy_io_error() -> io::Error {
        io::Error::new(
            // Dummy value.
            io::ErrorKind::NotFound,
            "",
        )
    }

    #[cfg(unix)]
    #[test]
    fn system_test() -> Result<(), Box<dyn error::Error>> {
        #[derive(Debug)]
        enum Environment {
            Default,
            MakeFooToBarSymlink,
        }

        #[derive(Debug)]
        struct Test<'a> {
            input: &'a str,
            result: Result,
            environment: Environment,
        }

        let tests = [
            Test {
                input: r#"{"/": ""}"#,
                result: Err(Error::NotRegularComponent {
                    name: "".into(),
                    context: "".into(),
                }),
                environment: Environment::Default,
            },
            Test {
                input: r#"{".": ""}"#,
                result: Err(Error::NotRegularComponent {
                    name: "".into(),
                    context: "".into(),
                }),
                environment: Environment::Default,
            },
            Test {
                input: r#"{"..": ""}"#,
                result: Err(Error::NotRegularComponent {
                    name: "".into(),
                    context: "".into(),
                }),
                environment: Environment::Default,
            },
            Test {
                input: r#"{"foo": [1, 2, 3]}"#,
                result: Err(Error::NotSingleStringArray { context: "".into() }),
                environment: Environment::Default,
            },
            Test {
                input: r#"{"foo": "kek"}"#,
                result: Ok(()),
                environment: Environment::MakeFooToBarSymlink,
            },
            Test {
                input: r#"{"foo": {}}"#,
                result: Err(Error::ChangeDir {
                    e: make_dummy_io_error(),
                    context: "".into(),
                }),
                environment: Environment::MakeFooToBarSymlink,
            },
            Test {
                input: r#"{"foo": ["bar"]}"#,
                result: Ok(()),
                environment: Environment::Default,
            },
            Test {
                input: r#"{"foo": [""]}"#,
                result: Err(Error::Create {
                    e: make_dummy_io_error(),
                    context: "".into(),
                    kind: "",
                }),
                environment: Environment::Default,
            },
        ];

        // For some reason, using a tempdir per test makes them fail sometimes.
        let dir = tempfile::tempdir()?;
        env::set_current_dir(&dir)?;

        for (i, test) in tests.iter().enumerate() {
            use std::os::unix;

            println!("\n# Subtest number {i}\n{test:#?}");

            let subdir = i.to_string();

            create_dir(&subdir)?;
            env::set_current_dir(&subdir)?;

            match test.environment {
                Environment::Default => {}
                // Environment::MakeFooFile => fs::write("foo", "")?,
                // Environment::MakeFooDir => create_dir("foo")?,
                Environment::MakeFooToBarSymlink => unix::fs::symlink("bar", "foo")?,
            }

            let result = run(test.input);

            assert_eq!(result, test.result);

            env::set_current_dir(&dir)?;
        }

        Ok(())
    }

    #[test]
    fn test() -> Result<(), Box<dyn error::Error>> {
        #[derive(Debug)]
        enum Environment {
            Default,
            MakeFooFile,
            MakeFooDir,
            MakeUnwritableDir,
        }

        #[derive(Debug)]
        struct Test<'a> {
            input: &'a str,
            result: Result,
            environment: Environment,
        }

        let tests = [
            Test {
                input: r"3",
                result: Err(Error::JsonIsNotObject),
                environment: Environment::Default,
            },
            Test {
                input: "3 4",
                result: Err(Error::CouldNotParseJson),
                environment: Environment::Default,
            },
            Test {
                input: r#"{"foo/bar": ""}"#,
                result: Err(Error::MultiplePathComponents {
                    name: "".into(),
                    context: "".into(),
                }),
                environment: Environment::Default,
            },
            Test {
                input: r#"{"file": "Hello!"}"#,
                result: Ok(()),
                environment: Environment::Default,
            },
            Test {
                input: r"{}",
                result: Ok(()),
                environment: Environment::Default,
            },
            Test {
                input: r#"{"file": 3}"#,
                result: Err(Error::InvalidJsonPart { context: "".into() }),
                environment: Environment::Default,
            },
            Test {
                input: r#"{"foo": {}}"#,
                result: Err(Error::ChangeDir {
                    context: "".into(),
                    e: make_dummy_io_error(),
                }),
                environment: Environment::MakeFooFile,
            },
            Test {
                input: r#"{"foo": {}}"#,
                result: Ok(()),
                environment: Environment::MakeFooDir,
            },
            Test {
                input: r#"{"foo": "Hello"}"#,
                result: Err(Error::Create {
                    e: make_dummy_io_error(),
                    kind: "",
                    context: "".into(),
                }),
                environment: Environment::MakeFooDir,
            },
            Test {
                input: r#"{"foo": "Hello"}"#,
                result: Ok(()),
                environment: Environment::MakeFooFile,
            },
            Test {
                input: r#"{"foo": {"bar": "baz"}}"#,
                result: Err(Error::Create {
                    e: make_dummy_io_error(),
                    kind: "",
                    context: "".into(),
                }),
                environment: Environment::MakeUnwritableDir,
            },
        ];

        for test in tests {
            println!("{test:#?}");

            let tmp = tempfile::tempdir()?;
            env::set_current_dir(&tmp)?;

            match test.environment {
                Environment::Default => {}
                Environment::MakeFooFile => fs::write("foo", "")?,
                Environment::MakeFooDir => create_dir("foo")?,
                Environment::MakeUnwritableDir => {
                    let mut permissions = fs::metadata(current_dir())?.permissions();
                    permissions.set_readonly(true);
                    fs::set_permissions(current_dir(), permissions)?;
                }
            }

            let result = run(test.input);

            assert_eq!(result, test.result);
        }

        Ok(())
    }
}
