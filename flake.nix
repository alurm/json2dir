{
  inputs.rust-overlay.url = "github:oxalica/rust-overlay";

  outputs =
    {
      nixpkgs,
      rust-overlay,
      flake-utils,
      self,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
          overlays = [
            self.overlays.dev
            (import rust-overlay)
          ];
        };
      in
      {
        packages = {
          default = pkgs.json-to-dir;
          coverage = pkgs.json-to-dir-coverage;
        };

        devShells.default = pkgs.mkShell {
          inputsFrom = [ pkgs.json-to-dir-coverage ];

          packages = with pkgs; [
            (rust-bin.nightly.latest.default.override {
              extensions = [
                "rust-src"
                "rust-docs"
                "miri"
                "rust-analyzer"
                "llvm-tools-preview"
              ];
            })
          ];
        };

        # Converts a Nix expression to a directory.
        #
        # Two arguments are expected: the derivation name and the Nix expression to convert.
        # 
        # Strings are assumed to be JSON.
        # Other data is first converted to JSON using builtins.toJSON.
        # JSON objects represent directories.
        # JSON strings represent file contents.
        # JSON arrays of the form ["target"] represent symlinks.
        toDir = (import ./to-dir.nix) pkgs;
      }
    )
    // {
      overlays = {
        default = final: prev: {
          json-to-dir = final.callPackage ./package.nix { custom.coverage = false; };
        };
        dev = final: prev: {
          json-to-dir = final.callPackage ./package.nix { custom.coverage = false; };
          json-to-dir-coverage = final.callPackage ./package.nix { custom.coverage = true; };
        };
      };
    };
}
