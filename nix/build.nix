{
  cargo,
  stdenv,
  wasm-pack,
  naersk,
}:
let
  wasmTarget = "wasm32-unknown-unknown";
in
naersk.buildPackage {
  root = ../.;

  copyLibs = true;
  CARGO_BUILD_TARGET = wasmTarget;
}
