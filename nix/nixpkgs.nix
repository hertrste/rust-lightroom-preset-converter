{
  sources ? import ./sources.nix,
  nixpkgs ? sources.nixpkgs,
  rust-overlay ? (import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz")),
}:
import nixpkgs {
    overlays = [
      rust-overlay
    ];
}
