{
  check-coverage,
  report ? false,
}:
let
  impl =
    args@{
      scdoc,
      installShellFiles,
      lib,
      ...
    }:
    (
      if check-coverage then
        let
          toolchain = args.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        in
        args.makeRustPlatform {
          rustc = toolchain;
          cargo = toolchain;
        }
      else
        args.rustPlatform
    ).buildRustPackage
      {
        pname = "json2dir";
        version = "0.1.0";
        src = ./.;

        cargoHash = "sha256-v+pbMlrcz1Cx3j8swYRkjEwSY2wv0047DScb/f8YxgI=";

        nativeBuildInputs = [
          scdoc
          installShellFiles
        ];

        # TODO: update the man page.
        postInstall = ''
          scdoc < ./json2dir.1.scd > json2dir.1
          installManPage json2dir.1
        '';

        meta = {
          homepage = "https://github.com/alurm/json2dir";
          license = lib.licenses.mit;
          description = "Program to convert JSON expressions to directory trees";
        };

        nativeCheckInputs = (
          if check-coverage then
            with args;
            [
              jq
              cargo-llvm-cov
            ]
          else
            [ ]
        );

        checkPhase = ''
          runHook preCheck

          ${
            if check-coverage then
              (
                if report then
                  "./scripts/test-and-collect-coverage --html --output-dir $out"
                else
                  "./scripts/helpers/test-and-check-for-full-coverage"
              )
            else
              "./scripts/test"
          }

          runHook postCheck
        '';
      };
in
if
  check-coverage

# TODO: remove duplication.
then
  args@{
    scdoc,
    installShellFiles,
    lib,

    rust-bin,
    jq,
    cargo-llvm-cov,
    makeRustPlatform,
  }:
  impl args
else
  args@{
    scdoc,
    installShellFiles,
    lib,

    rustPlatform,
  }:
  impl args
