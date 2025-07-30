{
  rustPlatform,
  scdoc,
  installShellFiles,
  lib,
  es,

  custom,

  # TODO: ideally, should only be needed if testing is required.
  # Not sure if that is the case at the moment.
  jq,
  cargo-llvm-cov,
  clang-tools,
  rust-bin,
  makeRustPlatform,
}:
# TODO: this should be cleaned up later.
(if custom.coverage
  then
    let
      toolchain = rust-bin.nightly.latest.default.override {
        extensions = [
          "llvm-tools-preview"
        ];
      };
    in
    makeRustPlatform {
      rustc = toolchain;
      cargo = toolchain;
    }
  else rustPlatform
).buildRustPackage {
  pname = "json-to-dir";
  version = "0.1.0";
  src = ./.;
  cargoHash = "sha256-CEUu8cFnpKGLrukPJaRgpfdFpRyBlrPbGl9lW73S1l4=";

  nativeBuildInputs = [
    scdoc
    installShellFiles
  ];

  postInstall = ''
    scdoc < ./json-to-dir.1.scd > json-to-dir.1
    installManPage json-to-dir.1
  '';

  meta = {
    homepage = "https://github.com/alurm/json-to-dir";
    maintainers = [ ];
    license = lib.licenses.mit;
    description = "json-to-dir: convert JSON with objects and strings into a directory";
  };

  nativeCheckInputs = [
    es
  ]
  ++ (
    if custom.coverage then
      [
        jq
        cargo-llvm-cov
      ]
    else
      [ ]
  );

  doCheck = true;
  checkPhase = ''
    runHook preCheck

    patchShebangs .

    ./do ${
      if custom.coverage then "run-all-tests-with-coverage-with-percent-output" else "run-all-tests"
    }
    ./do run-all-tests

    runHook postCheck
  '';
}
