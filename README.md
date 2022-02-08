
# Install
* https://www.rust-lang.org/learn/get-started --> Getting started
* https://visualstudio.microsoft.com/visual-cpp-build-tools/



# Embedded

* https://doc.rust-lang.org/stable/embedded-book/c-tips/index.html
* https://github.com/rust-embedded/book


# Most important commands

## Build
* build your project with `cargo build`
* run your project with `cargo run`
* test your project with `cargo test`
* build documentation for your project with `cargo doc`
* publish a library to crates.io with `cargo publish`


`cargo --version`


### Training

#### hello-rust
Directory: hello-rust
Description: https://www.rust-lang.org/learn/get-started

* cargo new hello-rust


https://doc.rust-lang.org/stable/rust-by-example/primitives.html


## Rust on Raspberry Pi Pico
* https://github.com/rp-rs/rp-hal
* https://www.youtube.com/watch?v=Yi0WRF5WPFw

# Test
* git clone https://github.com/rp-rs/rp-hal
* Need for building (Raspberry Pico HAL / driver)
  * rustup target add thumbv6m-none-eabi
* Need for downloading the binary to the target
  * cargo install elf2uf2-rs
* Execute build-rp-hal-blinky.sh

