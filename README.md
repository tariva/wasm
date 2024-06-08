# Rust WebAssembly Keylogger Demo

This project demonstrates a basic keylogger implemented in Rust, compiled to WebAssembly (Wasm), and served using Webpack. The purpose of this demo is to explore the capabilities of Rust and WebAssembly in intercepting and logging keystrokes within a web environment. This project is intended for educational purposes only, highlighting the integration of Rust, WebAssembly, and web technologies.

## Features

- Rust-based Logic: Core keylogging functionality implemented securely in Rust.
- WebAssembly Compilation: Rust code is compiled into WebAssembly, enabling high-performance, low-level browser operations.
- Webpack Integration: Utilizes Webpack to serve the Wasm module and facilitate easy development workflows.
- Project Structure
- src/: Rust source files implementing the keylogging logic.
- pkg/: built WebAssembly module.
- wasmdemo/src: rust sourcecode
- webpack.config.js: Configuration file for Webpack.
- index.html: Entry point of the web application demonstrating the keylogger.

## Prerequisites

To build and run this project, you will need:

- Rust and Cargo (Rust's package manager)
- wasm-pack for building Rust code into WebAssembly
- Node.js and npm for managing JavaScript dependencies and running Webpack

## usage

``cd wasmdemo && cargo build -r``
``wasm-pack pack``
