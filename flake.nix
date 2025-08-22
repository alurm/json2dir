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
        packages = pkgs.json2dir
        # # Not sure how useful this is.
        # // {
        #   cross = pkgs.pkgsCross;
        # };
        ;

        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            jq
            scdoc
            (rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)
          ] ++ (
            if system != "aarch64-darwin" then [ cargo-llvm-cov ] else [ ]
          );
        };

        # This can be brought back once cargo-llvm-cov works on MacOS.
        # devShells.default = pkgs.mkShell {
        #   inputsFrom = [
        #     (
        #       # pkgs.json2dir.check-for-full-coverage
        #       if system != "aarch64-darwin" then pkgs.json2dir.check-for-full-coverage else pkgs.json2dir.default
        #     )
        #   ];
        # };
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
            report-coverage = final.callPackage (import ./. {
              check-coverage = true;
              report = true;
            }) { };
            static = final.pkgsStatic.callPackage (import ./. { check-coverage = false; }) { };
          };
        };
      };

      nixConfig = {
        extra-substituters = [ "https://json2dir.cachix.org" ];
        extra-trusted-public-keys = [
          "json2dir.cachix.org-1:kGit6Ar45Bu+1ivmgMpNSaBBNm8TRQ14WXE0gSIGpHo="
        ];
      };
    };
}
