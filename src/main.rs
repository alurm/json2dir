use std::{
    env::{self, args},
    fs::{self, create_dir},
    io::{Read, stdin},
    path::{self, Path},
    process::exit,
};

macro_rules! die {
    ($($arg:tt)*) => {{
        eprintln!($($arg)*);
        std::process::exit(1);
    }}
}

fn json_object_to_dir(object: serde_json::Map<String, serde_json::Value>) {
    for (name, value) in object {
        let path = Path::new(&name);
        for component in path.components() {
            if let path::Component::Normal(_) = component {
            } else {
                die!("Error: found a non-regular component in a path");
            }
        }
        match value {
            serde_json::Value::String(content) => {
                fs::write(path, content).unwrap_or_else(|err| {
                    die!("Error: couldn't create a file {path:#?}: {err:#?}")
                });
            }
            serde_json::Value::Object(subdir) => {
                create_dir(&path).unwrap_or_else(|err| {
                    die!("Error: couldn't create a directory {path:#?}: {err:#?}");
                });
                env::set_current_dir(path).unwrap_or_else(|err| { die!("Error: couldn't set the current dir to the newly created path {path:#?}: {err:#?}"); });

                // NOTE: consider using a queue instead of recursion to avoid stack overflows.
                json_object_to_dir(subdir);

                env::set_current_dir(path::Component::ParentDir).unwrap_or_else(|err| die!("Error: couldn't set the current dir {path:#?} to the dir above it: {err:#?}"));
            }
            _ => die!("Error: found a JSON value which isn't an object or a string."),
        }
    }
}

fn main() {
    if args().len() != 1 {
        let usage = include_str!("../README.md");
        print!("{}", usage);
        exit(1);
    }

    let mut string = String::new();
    stdin()
        .read_to_string(&mut string)
        .unwrap_or_else(|_| die!("Error: couldn't read stdin to a string."));
    let json: serde_json::Value = serde_json::from_str(&string)
        .unwrap_or_else(|_| die!("Error: couldn't parse stdin to JSON."));
    if let serde_json::Value::Object(object) = json {
        json_object_to_dir(object);
    } else {
        die!("Error: the JSON is not an object.");
    }
}
