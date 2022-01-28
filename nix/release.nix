{
  sources ? import ./sources.nix,
  pkgs ? import ./nixpkgs.nix {}
}:
let
  wasmTarget = "wasm32-unknown-unknown";
  rustVersion = "1.54.0";
  rust = pkgs.rust-bin.stable.${rustVersion}.default.override {
    targets = [ wasmTarget ];
  };
  naersk = pkgs.callPackage sources.naersk {
    cargo = rust;
    rustc = rust;
  };
in
rec {
  # Build the rust wasm project resulting in *.wasm + javascript bindings
  rust-wasm-preset-converter = pkgs.callPackage ./build.nix { inherit naersk; };

  # Bundle the rust wasm project with a html project
  preset-converter = pkgs.stdenv.mkDerivation {
    name = "html";

    src = ../.;

    buildCommand = ''
      mkdir -p $out
      ln -s ${rust-wasm-preset-converter} $out/rust-preset-converter
      ln -s $src/index.html $out/index.html
      ln -s $src/app.js $out/app.js
    '';
  };

  # Create a script that starts a webserver serving the preset converter
  http-server = pkgs.writers.writeBash "http-server.sh" ''
    set -uo pipefail
    port="8000"
    ${pkgs.simple-http-server}/bin/simple-http-server --port "$port" -- ${preset-converter} &
    src_pid=$!

    echo "Serving ${preset-converter} with PID $src_pid"

    trap 'kill $src_pid; exit' INT

    sleep 1000
  '';
}
