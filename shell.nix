{
  sources ? import ./nix/sources.nix,
  nixpkgs ? sources.nixpkgs,
  rust-overlay ? (import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz")),
  pkgs ? import nixpkgs {
    overlays = [ rust-overlay ];
  }
}:
pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    cargo
    cargo-watch
    nodejs
    wasm-pack
    (rust-bin.stable.latest.default.override {
      extensions = [ "rust-src" ];
      targets = [ "wasm32-unknown-unknown" ];
    })
  ];
}
