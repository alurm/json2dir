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
        packages = pkgs.json2dir;

        devShells.default = pkgs.mkShell {
          inputsFrom = [ pkgs.json2dir.check-for-full-coverage ];
        };
      }
    )
    // {
      overlays = {
        default = final: prev: {
          json2dir = final.callPackage (import ./. { check-coverage = false; }) { };
        };
        dev = final: prev: {
          json2dir = {
            default = final.callPackage (import ./. { check-coverage = false; }) { };
            check-for-full-coverage = final.callPackage (import ./. { check-coverage = true; }) { };
            report-coverage = final.callPackage (import ./. { check-coverage = true; report = true; }) { };
          };
        };
      };
    };
}
