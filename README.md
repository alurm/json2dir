# json-to-dir: convert JSON with objects and strings into a directory

![Build](https://github.com/alurm/json-to-dir/actions/workflows/build.yaml/badge.svg)
![100% region coverage](https://github.com/alurm/json-to-dir/actions/workflows/coverage.yaml/badge.svg)

An example:

```sh
printf '%s' '{
  "file": "Hello, world!\n",
  "dir": {
    "subfile": "Content.\n",
    "subdir": {}
  }
}' | json-to-dir
```

## Building

Run `cargo build` or `nix build`.
