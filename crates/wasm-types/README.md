# yozefu-wasm-types

[![Build](https://github.com/MAIF/yozefu/actions/workflows/build.yml/badge.svg)](https://github.com/MAIF/yozefu/actions/workflows/build.yml)
[![](https://img.shields.io/crates/v/yozefu-wasm-types.svg)](https://crates.io/crates/yozefu-wasm-types)

This library provides structures for defining a WebAssembly module for the search engine of [yozefu](https://github.com/MAIF/yozefu). It uses `json` to exchange data from the host to the WebAssembly module.

## Usage

**NOTE:** You probably don't want to use this crate directly. Instead you should run the command `create-filter`:
```bash
yozf create-filter --language rust key-ends-with
```