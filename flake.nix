{
  inputs.rust-overlay.url = "github:oxalica/rust-overlay";

  outputs = { nixpkgs, rust-overlay, flake-utils, ... }: flake-utils.lib.eachDefaultSystem (system: let
    pkgs = (import nixpkgs) {
      inherit system;
      overlays = [ (import rust-overlay) ];
    };
  in {
    packages.default = pkgs.rustPlatform.buildRustPackage {
      pname = "json2dir";
      version = "0.1.0";
      src = ./.;
      cargoHash = "sha256-Q1dBrRFNBMUOEKJh7QHAsAg+nXwwVZlu0L87VHd6RNI=";
      meta = {
        homepage = "https://github.com/alurm/json2dir";
        maintainers = [];
        license = pkgs.lib.licenses.mit;
        description = "json2dir: convert JSON with objects and strings into a directory";
      };
    };
    devShells.default = pkgs.mkShell {
      packages = [
        (pkgs.rust-bin.nightly.latest.default.override {
          extensions = [ "rust-src" "rust-docs" "miri" "rust-analyzer" ];
        })
      ];
    };
  });
}
