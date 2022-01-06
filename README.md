# Rust Lightroom Preset Converter

## Build Instructions

The Nix package manager is used to build the Rust webassembly part of the
project. Do the following:
```sh
$ nix-build nix/release.nix -A rust-wasm-preset-converter
```

This compiles the Rust code to webassembly (.wasm file ending) with additional
javascript and typescript wrappers that can be imported from your corresponding
web project.

To bundle the webassembly together with the website ready to be served via a
webserver do the following:
```sh
$ nix-build nix/release.nix -A preset-converter
```

To build and serve the web application that uses the webassembly library do the
following:
```sh
$ nix-build nix/release.nix -A http-server
```

This creates a script that on invocation starts a web-server that is accessible
at `http://localhost:8000/index.html` per default.
