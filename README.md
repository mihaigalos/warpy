# Warpy
[![CI](https://github.com/mihaigalos/warpy/actions/workflows/ci.yaml/badge.svg)](https://github.com/mihaigalos/warpy/actions/workflows/ci.yaml) [![crates.io](https://img.shields.io/crates/d/warpy.svg)](https://crates.io/crates/warpy) [![LoC](https://tokei.rs/b1/github/mihaigalos/warpy)](https://github.com/mihaigalos/warpy)

A thin [warp](https://github.com/seanmonstar/warp) wrapper for serving folders over http. 

![example](screenshots/warpy.png)


## Why?

Single contained lib with minimalistic CSS style to add http serving of a folder to any application.

## Usage

The package is a library, add it to your `Cargo.toml` dependencies as you would any other package.

## Example

To run a simple example serving from the folder it is invoked, try:

```bash
cargo run --example simple
```
