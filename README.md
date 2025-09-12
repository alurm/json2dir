# `json2dir`: a JSON-to-directory converter, a fast alternative to home-manager for managing dotfiles

![Build status](https://github.com/alurm/json2dir/actions/workflows/build.yaml/badge.svg)
![100% region coverage](https://github.com/alurm/json2dir/actions/workflows/check-for-full-region-coverage.yaml/badge.svg)

`json2dir` it a tool suitable for converting JSON objects into directory trees using a special conversion scheme specified below.

## Table of contents

- [TL;DR](#tldr)
- [Use cases](#use-cases)
  - [Managing dotfiles with Nix](#managing-dotfiles-with-nix)
- [Conversion scheme](#conversion-scheme)
- [Caveats](#caveats)
- [Installing](#installing)
  - [With Cargo](#with-cargo)
  - [With Nix flakes](#with-nix-flakes)
- [Development](#development)

## TL;DR

Let's start with an example.

Assume the we have the file named `example-tree.json` in the current directory with the following contents:

```json
{
  "greeting": "Hello, world!",
  "dir": {
    "subfile": "Content.\n",
    "subdir": {}
  },
  "symlink": ["link", "target path"],
  "script": ["script", "#!/bin/sh\necho Howdy!"]
}
```

And then, after installing `json2dir`, we run this command:

```sh
cat example-tree.json | json2dir
```

The following files will be created:

- `greeting`: a regular file containing the text `Hello, world!`.
- `dir`: a directory with two entries in it (`subfile` and `subdir`).
- `symlink`: a symbolic link pointing to `target path`,
- `script`: an executable shell script that prints `Howdy!` when run.

Learn the [conversion scheme](#conversion-scheme) for more details.

## Use cases

Since `json2dir` accepts any JSON, it has a variety of use cases with other tools that can generate JSON.

### Managing dotfiles with Nix

`json2dir` can be used together with `nix profile` as a `home-manager` replacement for managing dotfiles. [Explanation](./docs/home.md).

## Conversion scheme

### Objects

Objects represent directories. Keys of objects represent names of files in directories. The JSON document as a whole must be an object.

#### Examples

An empty directory: `{}`.

A directory with an empty directory named `foo`: `{"foo": {}}`.

### Strings

String values represent contents of files.

#### Examples

A directory with a file named `hello` with the text `Hello, world`: `{"hello": "Hello, world"}`.

### Arrays

Arrays represent symlinks and files with an executable bit set (executable files, scripts). Such files are not supported on Windows.

The first element of the array must be a string.

If the string is `"link"`, the second array element represents the target of the symlink.

If the string is `"script"`, the second array element represent the contents of the executable file.

#### Examples

A symbolic link pointing to the root directory: `["link", "/"]`.

An executable file printing `Hello` when ran: `["script", "#!/bin/sh\necho Hello"]`.

## Caveats

Regular JSON constraints apply. In particular, the input must be UTF-8. Currently, there's no way to represent files containing non-UTF-8 content.

When using this utility to create files for other users, care must be taken in order to prevent TOCTOU (time of check, time of use) attacks (e.g. with symlinks).

Further design considerations are described in [this document](docs/json2dir-design.md).

Potential solution are described in [this document](docs/addressing-limitations.md).

## Installing

`json2dir` can be installed in multiple ways.

### With Cargo

Run `cargo install json2dir`.

### With Nix flakes

[flake.nix](flake.nix) contains a Nix package for `json2dir`. Add it to your flake inputs. The path `json2dir.packages.${system}.default` contains the package.

## Development

To build the project, run `cargo build` or `nix build`. If you're using `rustup`, `rust-toolchain.toml` is provided.

Useful scripts may be found in the `scripts` folder.

A Nix cache is available at <https://json2dir.cachix.org>.

Feel free to fork/open issues/submit PRs/etc.

<!-- ## Further reading
- [README of dir2json](src/bin/dir2json/README.md) -->
