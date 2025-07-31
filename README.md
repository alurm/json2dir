# TODO: finish/rework this.

# A json2dir converter in Rust and a poor man's home-manager alternative in Nix

![Build](https://github.com/alurm/json2dir/actions/workflows/build.yaml/badge.svg)
![100% region coverage](https://github.com/alurm/json2dir/actions/workflows/full-region-coverage.yaml/badge.svg)

## The json2dir

An example:

```sh
printf '%s' '{
  "file": "Hello, world!\n",
  "dir": {
    "subfile": "Content.\n",
    "subdir": {}
  }
}' | json2dir
```

## The config management part

`json2dir` when combined with `nix profile` can be used as a poor man's home-manager alternative: `nix profile` can manage your packages, while `json2dir` can manage your `~/.config`.

TODO: finish this.

Here's an sample activation script which you can add to your profile:

```nix
(pkgs.writeShellScriptBin "update-config" ''
  set -x
  cd ~/path/to/your/profile/flake || exit 1
  # TODO: finish this.
'')
```

## Building

Run `cargo build` or `nix build`.

## Contributing

Feel free to fork/open issues/submit PRs. I don't guarantee PRs will be accepted though.
