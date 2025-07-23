use serde_json as json;
use std::{
    env::{self, args},
    fs::{self, create_dir},
    io::{self, Read, stdin},
    path::{self, PathBuf},
    process::ExitCode,
};
use thiserror::Error;

type Result<T = (), E = Error> = std::result::Result<T, E>;

#[derive(Debug, Error)]
enum Error {
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

    #[error("found a JSON array that doesn't contain a single string at path {context:#?}")]
    NotSingleStringArray { context: PathBuf },

    #[cfg(not(unix))]
    #[error("found a JSON value which isn't an object or a string at path {context:#?}")]
    InvalidJson { context: PathBuf },

    #[cfg(unix)]
    #[error("found a JSON value which isn't an object, an array, or a string at path {context:#?}")]
    InvalidJsonPart { context: PathBuf },

    #[error("couldn't convert stdin to JSON")]
    CouldNotReadJson,

    #[error("the JSON is not an object")]
    JsonIsNotObject,

    // Usage.
    #[error("Usage: json-to-dir < file.json")]
    InvalidArgs,
}

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
                        // Existing directies are fine.
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

fn json_object_to_dir(object: json::Map<String, json::Value>) -> Result {
    json_object_to_dir_rec(&mut path::Component::CurDir.as_os_str().into(), object)
}

fn run() -> Result {
    if args().len() != 1 {
        return Err(Error::InvalidArgs);
    }

    let mut string = String::new();

    if let Err(_) = stdin().read_to_string(&mut string) {
        return Err(Error::CouldNotReadJson);
    }

    let Ok(json) = json::from_str(&string) else {
        return Err(Error::CouldNotReadJson);
    };

    let json::Value::Object(object) = json else {
        return Err(Error::JsonIsNotObject);
    };

    json_object_to_dir(object)
}

fn main() -> ExitCode {
    match run() {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            match e {
                // Usage.
                Error::InvalidArgs => eprintln!("{e}"),
                // A regular error.
                _ => eprintln!("Error: {e}."),
            }
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use std::env::set_current_dir;

    use super::*;

    fn object(string: &str) -> json::Map<String, json::Value> {
        let json = json::from_str(string).unwrap();
        let json::Value::Object(object) = json else {
            panic!();
        };
        object
    }

    #[test]
    fn test() -> Result<(), Box<dyn std::error::Error>> {
        struct Test {
            json: json::Map<String, json::Value>,
            result: Result,
        }

        let tests = [
            Test {
                json: object(r#"{}"#),
                result: Ok(()),
            },
            Test {
                json: object(r#"3"#),
                result: Err(Error::JsonIsNotObject),
            },
        ];

        for test in tests {
            let dir = tempfile::tempdir()?;
            set_current_dir(dir)?;
            assert_eq!(1, 0);
            // assert_eq!(test.result, json_object_to_dir(test.json));
        }

        Ok(())
    }
}
