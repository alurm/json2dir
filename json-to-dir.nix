{
  rustPlatform,
  scdoc,
  installShellFiles,
  lib,
}:
rustPlatform.buildRustPackage {
  pname = "json-to-dir";
  version = "0.1.0";
  src = ./.;
  cargoHash = "sha256-vKC2FYMng8bjCj3LjlME5/zvzr6Zl6DWmv7EmcW3Euk=";

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
}
