# `json2dir`: a tool that converts JSON objects to directory trees

![Build](https://github.com/alurm/json2dir/actions/workflows/build.yaml/badge.svg)
![100% region coverage](https://github.com/alurm/json2dir/actions/workflows/check-for-full-region-coverage.yaml/badge.svg)

TL;DR:

```sh
printf '%s' '{
  "file": "Hello, world!",
  "dir": {
    "subfile": "Content.\n",
    "subdir": {}
  },
  "symlink": ["link", "target path"],
  "script": ["script", "#!/bin/sh\necho Howdy!"]
}' | json2dir
```

Here, four files will be added to the current directory: `file`, `dir`, `symlink`, and `script`.

# Input schema

- Objects represent directories.
- Strings represent contents of files.
- Arrays are used to represent symlinks and executable files.
- Arrays of the form `["link", target]` represent symlinks, second element representing the target of the symlink.
- Arrays of the form `["script", content]` represent executable files, second representing the content of the script.

# Caveats

Regular JSON constraints apply. In particular, the input must be UTF-8. Currently, there's no way to represent files containing non-UTF-8 content.

When using this utility to create files for other users, care must be taken in order to prevent TOCTOU (time of check, time of use) attacks (e.g. with symlinks).

# Development

To build the project, run `cargo build` or `nix build`. If you're using `rustup`, `rust-toolchain.toml` is provided.

Useful scripts may be found in the `scripts` folder.

Feel free to fork/open issues/submit PRs/etc.

A Nix cache is available at https://json2dir.cachix.org.

# Users

- [nix2dir](https://github.com/alurm/nix2dir) uses this project in order to convert Nix expressions to directory trees (not released yet).

