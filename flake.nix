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
            rust-overlay.overlays.default
          ];
        };
      in
      {
        packages = {
          default = pkgs.json2dir;
          check-coverage = pkgs.json2dir-check-coverage;
        };

        devShells.default = pkgs.mkShell {
          inputsFrom = [ pkgs.json2dir-check-coverage ];
        };
      }
    )
    // {
      overlays = {
        default = final: prev: {
          json2dir = final.callPackage (import ./. { check-coverage = false; }) { };
        };
        dev = final: prev: self.overlays.default final prev // {
          json2dir-check-coverage = final.callPackage (import ./. { check-coverage = true; }) { };
        };
      };
    };
}
