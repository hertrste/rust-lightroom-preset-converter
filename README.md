# Rust Lightroom Preset Converter

## Build Instructions

The Nix package manager is used to build the Rust webassembly part of the
project. Do the following:
```sh
$ nix-build nix/release.nix -A build
```

To build and serve the web application that uses the webassembly library do the
following:
```sh
cd webapp/
npm install
npm run start
```

This should start a web-server that is accessible at `http://localhost:8080`
per default (port may vary).


## Known Issues

* The `webapp` has a hardcoded path to `result/` in the projects top-level.
  Therefore, `nix-build` must be invoked to build the Rust part of the project
  first.
