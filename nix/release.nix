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
{
  build = pkgs.callPackage ./build.nix { inherit naersk; };
}
