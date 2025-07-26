{ runCommand, json-to-dir, ... }:
name: expr:
runCommand name
  {
    passAsFile = [ "dirAsJson" ];
    dirAsJson = if builtins.typeOf expr == "string" then expr else builtins.toJSON expr;
    nativeBuildInputs = [ json-to-dir ];
  }
  ''
    mkdir $out
    cd $out
    json-to-dir < $dirAsJsonPath
  ''
