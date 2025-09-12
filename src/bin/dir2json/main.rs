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

type Subset = HashMap<Vec<u8>, Pathspec>;
type Pathspec = Option<Subset>;
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

// }

fn main() {
    // fs::File::open(".").unwrap();
}
