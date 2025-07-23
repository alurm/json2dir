# json-to-dir: convert JSON with objects and strings into a directory

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
