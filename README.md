# `json2dir`: a JSON-to-directory converter, a fast alternative to home-manager for managing dotfiles

![Build](https://github.com/alurm/json2dir/actions/workflows/build.yaml/badge.svg)
![100% region coverage](https://github.com/alurm/json2dir/actions/workflows/check-for-full-region-coverage.yaml/badge.svg)

`json2dir` specifies a subset of JSON suitable for describing directory trees and provides a tool to instantiate such descriptions.

TL;DR:

`example-tree.json`:

```json
{
  "file": "Hello, world!",
  "dir": {
    "subfile": "Content.\n",
    "subdir": {}
  },
  "symlink": ["link", "target path"],
  "script": ["script", "#!/bin/sh\necho Howdy!"]
}
```

```sh
cat example-tree.json | json2dir
```

Here, four files will be added to the current directory:

- `file`: a file with the text `Hello, world!`,
- `dir`: a directory with two entries in it,
- `symlink`: a symbolic link pointing to `target path`,
- `script`: an executable shell script that prints `Howdy!` when run.

## Use cases

- [Using `json2dir` with Nix as a `home-manager` alternative for managing dotfiles](./home.md)

## Input schema

- Objects represent directories.
- Strings represent contents of files.
- Arrays are used to represent symlinks and executable files.
- Arrays of the form `["link", target]` represent symlinks, second element representing the target of the symlink.
- Arrays of the form `["script", content]` represent executable files, second representing the content of the script.

## Caveats

Regular JSON constraints apply. In particular, the input must be UTF-8. Currently, there's no way to represent files containing non-UTF-8 content.

When using this utility to create files for other users, care must be taken in order to prevent TOCTOU (time of check, time of use) attacks (e.g. with symlinks).

## Packaging

[flake.nix](flake.nix) contains a Nix package for `json2dir`.

## Development

To build the project, run `cargo build` or `nix build`. If you're using `rustup`, `rust-toolchain.toml` is provided.

Useful scripts may be found in the `scripts` folder.

A Nix cache is available at <https://json2dir.cachix.org>.

Feel free to fork/open issues/submit PRs/etc.
