![Build and Deploy](https://github.com/jpnykw/befunge-interpreter/workflows/Build%20and%20Deploy/badge.svg) ![Rust](https://github.com/jpnykw/befunge-interpreter/workflows/Rust/badge.svg)

# what's this

This is an interpreter for the language befunge-93. See Wikipedia(https://en.wikipedia.org/wiki/Befunge) for the language specification. This interpreter provides basic functionality. Special implementations will be done in the future, but don't expect much as this project is being developed as a hobby XD

# how to use

You need to have Node.js and Rust installed to run it.  At a minimum, you will need `wasm-pack` and `live-server` installed. When you are ready, run the following command.

build: `wasm-pack build --target web`
run: `live-server --port=3000`
