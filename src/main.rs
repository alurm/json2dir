/// TODO: figure out if std::fs can be used in a way that would protect against TOCTOU reliably.
use serde_json as json;
use std::{
    env::{self, args},
    fs,
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

// TODO: figure out a way to properly create dummy payloads.
/// Tagged errors, useful for tests. PartialEq just checks the discriminant and not the payload.
#[derive(Debug, Error)]
enum Error {
    #[error("expected provided JSON to be an object")]
    InvalidTopJson,

    #[error("couldn't convert stdin to JSON")]
    CouldNotParseJson,

    #[error("the key {name:#?} under {context:#?} must have exactly one path component")]
    MultiplePathComponents { name: String, context: PathBuf },

    #[error("the key {name:#?} under {context:#?} is a non-regular path component")]
    NotRegularComponent { name: String, context: PathBuf },

    #[error("couldn't create a {kind} at {context:#?}: {e:#?}")]
    Create {
        e: io::Error,
        context: PathBuf,
        kind: &'static str,
    },

    #[error("couldn't set the current dir to the newly created path {context:#?}: {e:#?}")]
    ChangeDir { context: PathBuf, e: io::Error },

    #[error("couldn't set the current dir {context:#?} to the dir above it: {e:#?}")]
    ChangeDirUp { context: PathBuf, e: io::Error },

    #[error("couldn't make the script executable at {context:#?}: {e:#?}")]
    CouldNotMakeFileExecutable { context: PathBuf, e: io::Error },

    #[cfg(not(unix))]
    #[error("expected a JSON value to be an object or a string while at {context:#?}")]
    InvalidJsonPart { context: PathBuf },

    #[cfg(unix)]
    #[error("expected a JSON value to be an object, an array, or a string while at {context:#?}")]
    InvalidJsonPart { context: PathBuf },

    #[cfg(unix)]
    #[error("expected a JSON array to be of the form [type, payload] while at {context:#?}")]
    InvalidJsonArray { context: PathBuf },

    #[cfg(unix)]
    #[error(
        "expected a JSON array's first element to be either \"link\" or \"script\" while at {context:#?}"
    )]
    InvalidArrayKind { context: PathBuf },
}

#[derive(Default)]
/// Cfg provides a way for tests to have a representation at runtime if #[cfg(test)] is true.
struct Cfg {
    #[cfg(test)]
    test: Test,
}

#[cfg(test)]
#[derive(Default, Debug)]
/// Runtime representation of a test. Useful for causing some hard-to-cause errors.
enum Test {
    #[default]
    Default,
    #[cfg(unix)]
    CauseChangeDirUpError,
    #[cfg(unix)]
    RemoveScriptAfterCreation,
    #[cfg(unix)]
    RemoveScriptAfterGettingMode,
}

fn json_object_to_dir_rec(
    context: &mut PathBuf,
    object: json::Map<String, json::Value>,
    _cfg: &Cfg,
) -> Result {
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

        // Ignore errors here.
        // We don't care what was here before, but we want to remove symlinks.
        let _ = fs::remove_file(&path_component);

        match value {
            // Create a directory.
            json::Value::Object(subdir) => {
                if let Err(e) = fs::create_dir(&path_component) {
                    match e.kind() {
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

                #[cfg(all(test, unix))]
                if let Test::CauseChangeDirUpError = _cfg.test {
                    let here = path::absolute(std::env::current_dir().unwrap()).unwrap();
                    fs::create_dir(here.join("bar")).unwrap();
                    env::set_current_dir("bar").unwrap();

                    use std::{fs::File, os::unix::fs::PermissionsExt};

                    let meta = here.metadata().unwrap();
                    let mut perms = meta.permissions();
                    perms.set_mode(0o600);
                    let file = File::open(here).unwrap();
                    file.set_permissions(perms).unwrap();
                }

                // TODO: use a queue instead of recursing to be safe against stack overflows?
                json_object_to_dir_rec(context, subdir, _cfg)?;

                if let Err(e) = env::set_current_dir(path::Component::ParentDir) {
                    return Err(Error::ChangeDirUp {
                        context: context.clone(),
                        e,
                    });
                };
            }

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

            // Create a symlink or an executable file.
            #[cfg(unix)]
            json::Value::Array(vec) => match &vec[..] {
                [json::Value::String(kind), json::Value::String(payload)] => match kind.as_str() {
                    "script" => {
                        if let Err(e) = fs::write(&path_component, payload) {
                            return Err(Error::Create {
                                e,
                                context: context.clone(),
                                kind: "script",
                            });
                        };

                        #[cfg(test)]
                        if let Test::RemoveScriptAfterCreation = _cfg.test {
                            fs::remove_file(&path_component).unwrap();
                        }

                        use std::{
                            fs::{metadata, set_permissions},
                            os::unix::fs::{MetadataExt, PermissionsExt},
                        };

                        let mode = match metadata(&path_component) {
                            Ok(meta) => meta.mode(),
                            Err(e) => {
                                return Err(Error::CouldNotMakeFileExecutable {
                                    context: context.clone(),
                                    e,
                                });
                            }
                        };

                        #[cfg(test)]
                        if let Test::RemoveScriptAfterGettingMode = _cfg.test {
                            fs::remove_file(&path_component).unwrap();
                        }

                        if let Err(e) =
                            set_permissions(path_component, PermissionsExt::from_mode(mode | 0o111))
                        {
                            return Err(Error::CouldNotMakeFileExecutable {
                                context: context.clone(),
                                e,
                            });
                        }
                    }

                    "link" => {
                        use std::os::unix;

                        if let Err(e) = unix::fs::symlink(payload, &path_component) {
                            return Err(Error::Create {
                                e,
                                context: context.clone(),
                                kind: "symlink",
                            });
                        }
                    }
                    _ => {
                        return Err(Error::InvalidArrayKind {
                            context: context.clone(),
                        });
                    }
                },
                _ => {
                    return Err(Error::InvalidJsonArray {
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

// TODO: add an option not to overwrite old files?
/// Converts a JSON object to a directory, recursively.
/// Operating system's defaults are generally followed without adding logic on top.
/// For example, by default, writing to symlinks will write to their target, the symlink itself will not be replaced.
fn json_object_to_dir(object: json::Map<String, json::Value>, cfg: &Cfg) -> Result {
    json_object_to_dir_rec(&mut current_dir(), object, cfg)
}

fn parse_and_run(string: &str, cfg: &Cfg) -> Result {
    let Ok(json) = json::from_str(string) else {
        return Err(Error::CouldNotParseJson);
    };

    let json::Value::Object(object) = json else {
        return Err(Error::InvalidTopJson);
    };

    json_object_to_dir(object, cfg)
}

fn main() -> ExitCode {
    if args().len() != 1 {
        eprintln!("Usage: json2dir < file.json");
        return ExitCode::FAILURE;
    }

    let mut string = String::new();

    if let Err(e) = stdin().read_to_string(&mut string) {
        eprintln!("Error: couldn't read stdin to an internal representation: {e:#?}.");
        return ExitCode::FAILURE;
    }

    if let Err(e) = parse_and_run(&string, &Cfg::default()) {
        eprintln!("Error: {e}.");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

#[cfg(test)]
mod tests;
