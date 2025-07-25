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
            self.overlays.default
            (import rust-overlay)
          ];
        };
      in
      {
        packages.default = pkgs.json-to-dir;

        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            python3
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

        toDir =
          name: expr:
          pkgs.runCommand name
            {
              passAsFile = [ "dir" ];
              dir = builtins.toJSON expr;
            }
            ''
              mkdir $out
              cd $out
              ${pkgs.json-to-dir}/bin/json-to-dir < $dirPath
            '';
      }
    )
    // {
      overlays.default = final: prev: {
        json-to-dir = final.callPackage ./json-to-dir.nix { };
      };
    };
}
