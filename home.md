# Using `json2dir` as a [`home-manager`](https://github.com/nix-community/home-manager) alternative

You can use `json2dir` to generate your dotfiles and use `nix profile` to manage your packages.

## Getting started

Here's a sample single-file `flake.nix` to get started (note: you don't have to use flakes):

```nix
{
  outputs =
    { nixpkgs, flake-utils, ... }:
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

            # These scripts help to update the profile and dotfiles.
            # You need to replace $variables, if you want to use them.

            (writeShellScriptBin "apply-my-profile-and-dotfiles" ''
              # Replace $profile_flake_name to have the name of this flake.
              nix profile upgrade $profile_flake_name || exit 1
              apply-my-dotfiles || exit 1
            '')

            # This operation is typically much faster than applying the profile.
            (writeShellScriptBin "apply-my-dotfiles" ''
              cd || exit 1
              # Replace $profile_flake_path to point to your flake.
              nix eval $profile_flake_path#home --json | json2dir || exit 1
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

Note that if you want to try out this example, you need to update `$profile_flake_path` to point to your flake.

To apply this configuration, two actions need to be performed:

```sh
# Install the profile (done only once).
nix profile install $profile_flake_path

# Apply the dotfiles.
nix eval --json .#home | (cd ~ && json2dir)
```

Later, you can use `nix profile upgrade $profile_flake_name` to upgrade your packages (this doesn't update your dotfiles, you need to use a helper script or do that separately).

You mind find [`nixpkgs.lib.generators`](https://nixos.org/manual/nixpkgs/stable/#sec-generators) useful for generating configuration files of a specific format.

## Caveats

Old files will not be deleted automatically.

Files created by `json2dir` are mutable. This allows for avoiding Nix builds, therefore making updating files fast, and allows for quickly changing files temporarily, but is not declarative.
