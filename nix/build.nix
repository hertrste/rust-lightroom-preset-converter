{ naersk
, nix-gitignore
, stdenv
, wasm-bindgen-cli
}:
let
  wasmTarget = "wasm32-unknown-unknown";
  rustBuild = naersk.buildPackage {
    root = nix-gitignore.gitignoreSource [ "nix/" ] ../.;

    copyLibs = true;
    copyTarget = true;
    CARGO_BUILD_TARGET = wasmTarget;
  };
in
stdenv.mkDerivation {
    name = "preset-converter";

    src = rustBuild;

    nativeBuildInputs = [ wasm-bindgen-cli ];

    buildCommand = ''
      mkdir -p $out

      for module in $src/lib/*.wasm; do
        RUST_BACKTRACE=1 wasm-bindgen --target web --out-dir $out $module
      done
    '';
  }
