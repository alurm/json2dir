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
*/

// dir2json . foo main.go **/*.go

use std::collections::HashMap;

type Dir = HashMap<Vec<u8>, File>;

enum File {
    Dir(Dir),
    Regular,
}

fn parse() -> Dir {
    todo!()
}

fn main() {}
