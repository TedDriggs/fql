fql

Rust implementation of a lexer and parser for CrowdStrike Falcon Query Language (FQL).

# Getting Started

You'll need `rust` and its package manager `cargo`. You can install them [here](https://rustup.rs/).

Once you have those, you will be able to run the tests and the demo CLI.

-   Run the tests with `cargo test`
-   Run the CLI with `cargo run -- --help`

# Web Demo

The `fql-ts` crate exposes bindings so the core `fql` crate can be used from JS/TS in the browser.

## Prerequisites

-   `wasm-pack` is needed to build the `fql-ts` package. You install it by running `cargo install wasm-pack` in your terminal.
-   `node` and `npm` are needed for the demo server. You can download the installer [here](https://nodejs.org/en/)

## Steps

1. Enter the `fql-ts` directory
2. Run `wasm-pack build --target web` to build the NPM package.
3. `cd ../fql-web-demo` to switch to the web demo directory.
4. Run `npm install` to install the dependencies from the `package.json` file
5. Run `npm run start` to start the dev server
