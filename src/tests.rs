use std::error;

use super::*;

fn make_dummy_io_error() -> io::Error {
    io::Error::new(
        // A dummy value.
        io::ErrorKind::NotFound,
        "",
    )
}

#[cfg(unix)]
#[test]
fn test_unix() -> Result<(), Box<dyn error::Error>> {
    #[derive(Debug)]
    enum Environment {
        Default,
        MakeFooToBarSymlink,
        MakeUnexecutableDir,
        MakeDir,
    }

    #[derive(Debug)]
    struct Test<'a> {
        input: &'a str,
        result: Result,
        environment: Environment,
        action: crate::Test,
    }

    impl<'a> Test<'a> {
        fn new(input: &'a str) -> Self {
            Self {
                input,
                result: Ok(()),
                environment: Environment::Default,
                action: crate::Test::default(),
            }
        }
    }

    let tests = [
        Test {
            result: Err(Error::NotRegularComponent {
                name: "".into(),
                context: "".into(),
            }),
            ..Test::new(r#"{"/": ""}"#)
        },
        Test {
            result: Err(Error::NotRegularComponent {
                name: "".into(),
                context: "".into(),
            }),
            ..Test::new(r#"{".": ""}"#)
        },
        Test {
            result: Err(Error::NotRegularComponent {
                name: "".into(),
                context: "".into(),
            }),
            ..Test::new(r#"{"..": ""}"#)
        },
        Test {
            result: Err(Error::InvalidJsonArray { context: "".into() }),
            ..Test::new(r#"{"foo": [1, 2, 3]}"#)
        },
        Test {
            environment: Environment::MakeFooToBarSymlink,
            ..Test::new(r#"{"foo": "kek"}"#)
        },
        Test {
            result: Err(Error::ChangeDir {
                e: make_dummy_io_error(),
                context: "".into(),
            }),
            environment: Environment::MakeUnexecutableDir,
            ..Test::new(r#"{"foo": {}}"#)
        },
        Test {
            ..Test::new(r#"{"foo": ["link", "bar"]}"#)
        },
        Test {
            result: Err(Error::Create {
                e: make_dummy_io_error(),
                context: "".into(),
                kind: "",
            }),
            environment: Environment::MakeDir,
            ..Test::new(r#"{"foo": ["script", "Hello"]}"#)
        },
        Test {
            ..Test::new(r#"{"foo": ["script", ""]}"#)
        },
        Test {
            result: Err(Error::InvalidArrayKind { context: "".into() }),
            ..Test::new(r#"{"foo": ["linksym", ""]}"#)
        },
        // These have actions.
        Test {
            result: Err(Error::ChangeDirUp {
                e: make_dummy_io_error(),
                context: "".into(),
            }),
            action: crate::Test::CauseChangeDirUpError,
            ..Test::new(r#"{"foo": {}}"#)
        },
        Test {
            result: Err(Error::CouldNotMakeFileExecutable {
                context: "".into(),
                e: make_dummy_io_error(),
            }),
            action: crate::Test::RemoveScriptAfterCreation,
            ..Test::new(r#"{"foo": ["script", ""]}"#)
        },
        Test {
            result: Err(Error::CouldNotMakeFileExecutable {
                context: "".into(),
                e: make_dummy_io_error(),
            }),
            action: crate::Test::RemoveScriptAfterGettingMode,
            ..Test::new(r#"{"foo": ["script", ""]}"#)
        },
        // These are Linux only.
        // On MacOS, creating an symlink that points nowhere is not an error.
        #[cfg(target_os = "linux")]
        Test {
            result: Err(Error::Create {
                e: make_dummy_io_error(),
                context: "".into(),
                kind: "",
            }),
            ..Test::new(r#"{"foo": ["link", ""]}"#)
        },
    ];

    for test in tests {
        use std::os::unix;

        println!("{test:#?}");

        let tmp = tempfile::tempdir().unwrap();
        env::set_current_dir(&tmp).unwrap();

        match test.environment {
            Environment::Default => {}
            Environment::MakeFooToBarSymlink => unix::fs::symlink("bar", "foo").unwrap(),
            Environment::MakeDir => fs::create_dir("foo").unwrap(),
            Environment::MakeUnexecutableDir => {
                use std::{fs::set_permissions, os::unix::fs::PermissionsExt};

                let p = PermissionsExt::from_mode(0o600);

                fs::create_dir("foo").unwrap();

                set_permissions("foo", p).unwrap();
            }
        }

        let result = parse_and_run(test.input, &Cfg { test: test.action });

        assert_eq!(result, test.result);
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

    impl<'a> Test<'a> {
        fn new(input: &'a str) -> Self {
            Self {
                input,
                result: Ok(()),
                environment: Environment::Default,
            }
        }
    }

    let tests = [
        Test {
            result: Err(Error::InvalidTopJson),
            ..Test::new("3")
        },
        Test {
            ..Test::new(r#"{"foo": {}}"#)
        },
        Test {
            result: Err(Error::CouldNotParseJson),
            ..Test::new("3 4")
        },
        Test {
            result: Err(Error::MultiplePathComponents {
                name: "".into(),
                context: "".into(),
            }),
            ..Test::new(r#"{"foo/bar": ""}"#)
        },
        Test {
            ..Test::new(r#"{"file": "Hello!"}"#)
        },
        Test { ..Test::new("{}") },
        Test {
            result: Err(Error::InvalidJsonPart { context: "".into() }),
            ..Test::new(r#"{"file": 3}"#)
        },
        Test {
            environment: Environment::MakeFooDir,
            ..Test::new(r#"{"foo": {}}"#)
        },
        Test {
            result: Err(Error::Create {
                e: make_dummy_io_error(),
                kind: "",
                context: "".into(),
            }),
            environment: Environment::MakeFooDir,
            ..Test::new(r#"{"foo": "Hello"}"#)
        },
        Test {
            environment: Environment::MakeFooFile,
            ..Test::new(r#"{"foo": "Hello"}"#)
        },
        Test {
            result: Err(Error::Create {
                e: make_dummy_io_error(),
                kind: "",
                context: "".into(),
            }),
            environment: Environment::MakeUnwritableDir,
            ..Test::new(r#"{"foo": {"bar": "baz"}}"#)
        },
        Test {
            result: Err(Error::MultiplePathComponents {
                name: "".into(),
                context: "./foo/bar".into(),
            }),
            ..Test::new(r#"{"foo": {"bar": {"": "error"}}}"#)
        },
    ];

    for test in tests {
        println!("{test:#?}");

        let tmp = tempfile::tempdir().unwrap();
        env::set_current_dir(&tmp).unwrap();

        match test.environment {
            Environment::Default => {}
            Environment::MakeFooFile => fs::write("foo", "").unwrap(),
            Environment::MakeFooDir => fs::create_dir("foo").unwrap(),
            Environment::MakeUnwritableDir => {
                let mut permissions = fs::metadata(current_dir()).unwrap().permissions();
                permissions.set_readonly(true);
                fs::set_permissions(current_dir(), permissions).unwrap();
            }
        }

        let result = parse_and_run(test.input, &Cfg::default());

        assert_eq!(result, test.result);
    }

    Ok(())
}
