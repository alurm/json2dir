# Using `json2dir` as a [`home-manager`](https://github.com/nix-community/home-manager) alternative for managing dotfiles

You can use `json2dir` to generate your dotfiles and use `nix profile` to manage your packages.

The idea here is to use `nix eval --json` to generate the JSON representing your dotfiles (or, in general, parts of your home directory) and pass it to `json2dir`. Since the Nix builder is not involved, this is fast.

> You may wish to use [Nixsa](https://github.com/noamraph/nixsa) or a similar solution to use Nix without root.

> You don't have to use this tool in conjunction with Nix! You can also use JSON directly, generate it from YAML or Cue, or do something different.

## Getting started

Here's a sample single-file `flake.nix` to get you started:

```nix
{
  inputs.json2dir.url = "github:alurm/json2dir";
  
  outputs =
    { nixpkgs, flake-utils, json2dir, ... }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        packages.default = pkgs.symlinkJoin {
          name = "my-profile";
          paths = with pkgs; [
            helix

            git
            jujutsu

            fish
            direnv

            # ...

            # Replace $path_to_this_flake with the path to the directory of the flake.
            (writeShellScriptBin "nix2home" ''
              cd ~ || exit 1
              nix eval $path_to_this_flake#home --json | ${json2dir.packages.${system}.default}/bin/json2dir || exit 1
            '')
          ];
        };
      }
    )
    // {
      home.".config" =
        let
          full-name = "John Doe";
          email = "john@example.com";
        in
        {
          git.config = ''
            [init]
            defaultBranch = main

            [user]
            name = ${full-name}
            email = ${email}
          '';

          jj.config = ''
            "$schema" = "https://jj-vcs.github.io/jj/latest/config-schema.json";

            [user]
            name = "${full-name}"
            email = "${email}"

            [ui]
            diff-editor = ":builtin"
            default-command = ["log", "-r", "all()"]
          '';

          direnv.direnvrc = ''
            source "$NIX_STATE_HOME/profile/share/nix-direnv/direnvrc";
          '';

          # ...
        };
    };
}
```

Note that if you want to try out this example, you need to update `$path_to_this_flake` to point to your flake.

To apply this configuration, you need to install the profile:

```
nix profile add .
```

Later, this profile can be upgraded with `nix profile upgrade <name>`, where `<name>` is the name of the profile you just added (you can discover it by running `nix profile list`).

After you have installed this profile (and added it to your `PATH`), now you can use the `nix2home` helper (defined via `writeShellScriptBin` above) to quickly update your dotfiles. Here's an example run:

```sh
$ time nix2home
real	0m0.069s
user	0m0.060s
sys	0m0.009s
```

You might find [`nixpkgs.lib.generators`](https://nixos.org/manual/nixpkgs/stable/#sec-generators) useful for generating configuration files of a specific format.

## More elaborate configs

- <https://github.com/alurm/infra/tree/main/mac/flake.nix>

Note that you don't have to put everything inside of one big Nix file, of course. For config files requiring no templating, `builtins.readFile` (or some kind of wrapper around it) may be helpful.

## Caveats

Old files will not be deleted automatically.

Files created by `json2dir` are mutable. This allows for avoiding Nix builds, therefore making updating files fast, and allows for quickly changing files temporarily, but is not declarative.

This solution doesn't replace all of functionality of `home-manager`, only the dotfiles management part.
