/*
TODO:

- Create a module for handling command line arguments.
- Create a module for reading the FS.
- Create a module for converting that to JSON.
*/

/*
# Only picked files

dir2json main.go foo/{bar,baz}

# The whole dir

dir2json

# Alternative design

dir2json '{foo, bar, baz: {quux, lol, wut}, joe: {0, 1}}'

dir2json foo bar { quux }

Main problems: tab completion, reliance on brace expansion, verbosity, completeness, special filenames.

dir2json foo bar { quux \\{ foo \\} }

Execline can do the escaping, actually.

# Should json2dir have a syntax to extract only a subset of the archive?

dir2json # Nothing.
dir2json foo # Only the pathname foo.
dir2json foo/ # The pathname foo and all of its children.
dir2json . # Nothing.
dir2json ./ # Everything.
dir2json ./{foo,bar,baz}

dir2json # Nothing.
dir2json foo
dir2json {foo,bar,baz}

dir2json src flake.nix

dir2json src/

dir2json src/json2dir/main.rs

foo -- represents the path foo only
dir2json . is allowed

dir2json --help
Warning: --help is interpreted as a pathname.

dir2json .
dir2json foo bar
dir2json foo/bar

jq -n '{ [base64, "..."] }' | dir2json

type Subset = HashMap<Vec<u8>, Pathspec>;
type Pathspec = Option<Subset>;

# How to do dirj2son?

dir2json needs to receive a selection tree.

```
type SelectionTree = Map<String, SelectionNode>;

enum SelectionNode {
    Leaf: String,
    Directory: SelectionTree
}

{
    a, b, c
}
```

What if a selector is not present?

echo '{ "src" }' | dir2json

Should there be a special syntax to select a directory?

dir2json src/

What should a call to dir2json do when no arguments are passed? Should the selection be empty by default?

`{ <include only these files> }`

The user may want to select based on other filters. E.g. using `find`.

It might be good to have multiple supported selector frontends.

```
{ . }
```

What escape sequences should be used?

dir2json '{ a b c: { d e f } }'

dir2json '{ "main.go" }'

Should extraction commands be supported?

jq | json2dir

jq | dir2json

```
dir2json
None

echo {}
Some $ Map None

echo { }

{ a }
```
*/

// dir2json . foo main.go **/*.go

// use std::collections::HashMap;
// use std::fs;

// type Dir = HashMap<Vec<u8>, File>;

// enum File {
//     Dir(Dir),
//     Regular,
// }

// fn parse() -> Dir {
//     todo!()
// }

// type Pathspec = Option<Entries>;
// struct Entries(HashMap<Vec<u8>, Pathspec>);

// fn _dir2json(_pathspec: Pathspec) {

use serde_json as json;
use std::{
    env::args,
    io::{Read as _, stdin},
    process::ExitCode,
};

use crate::selection::Selection;

mod selection;

// FIXME.
type Result<T = (), E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("couldn't convert stdin to JSON")]
    CouldNotParseJson,
    #[error("expected selection to be either an object or null")]
    InvalidJsonType,
}

fn read_all() {}

fn read_selection(selection: Selection) {
    match selection.0 {
        None => read_all(),
        Some(entries) => {
            for entry in entries {
                // env::set_current_dir(entry)
                todo!();
            }
        }
    }
}

fn parse_and_run(string: &str) -> Result {
    let Ok(json) = json::from_str(string) else {
        return Err(Error::CouldNotParseJson);
    };

    let selection = selection::from_json(json);

    dbg!(selection);

    // dbg!(read_selection(selection));

    todo!()
}

fn main() -> ExitCode {
    if args().len() != 1 {
        eprintln!("Usage: dir2json < selection.json");
        return ExitCode::FAILURE;
    }

    let mut string = String::new();

    if let Err(e) = stdin().read_to_string(&mut string) {
        eprintln!("Error: couldn't read stdin to an internal representation: {e:#?}.");
        return ExitCode::FAILURE;
    }

    if let Err(e) = parse_and_run(&string) {
        eprintln!("Error: {e}.");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
