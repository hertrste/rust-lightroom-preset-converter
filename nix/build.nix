{ naersk
, nix-gitignore
}:
let
  wasmTarget = "wasm32-unknown-unknown";
in
naersk.buildPackage {
  root = nix-gitignore.gitignoreSource [ "nix/" ] ../.;

  copyLibs = true;
  CARGO_BUILD_TARGET = wasmTarget;
}
