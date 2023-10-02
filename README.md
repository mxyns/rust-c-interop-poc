# rust-c-interop-poc

This proof-of-concept for C-to-Rust and Rust-to-C code execution features: 
 - a Rust library callable in C & Rust `uwe-rs-lib` [here](/rs/crates/uwe-rs-lib)
 	- bindings are automatically generated based on function definitions using `cargo-c`
 - a C executable binary calling `uwe-rs-lib` and using its struct types
 - a C library callable in C & Rust `uwe-c-lib` [here](/rs/crates/uwe-c-lib) which depends on `uwe-rs-lib` (calls its functions)
 	- bindings are automatically generates based on header definitions using `rust-bindgen`
 - a Rust executable binary calling `uwe-c-lib` (and, through it, `uwe-rs-lib`)

## requirements
 - unix system
 - llvm-dev libclang-dev clang [rust-bingen requirements](https://rust-lang.github.io/rust-bindgen/requirements.html) : `apt install llvm-dev libclang-dev clang`
 - rust stable toolchain
 - cargo-c (`cargo install cargo-c`)
 - cmake
 - something to compile C
 - let's hope i didn't forget something

## building and running

1. build `uwe-rs-lib` : `rs/ >`  `cargo cinstall -p uwe-rs-lib`
2. double check correct installation
	- `ls /usr/local/lib`
	- `ls /usr/local/include/uwe_rs_lib`
3. build the C binary : `c/bin/ >`  `cmake . && make`
4. run the C binary : `c/bin/ >`  `./uwe_c_bin`
5. build the C library : `c/lib/ >`  `cmake . && make install`
6. double check correct installation
	- `ls /usr/local/lib`
	- `ls /usr/local/include/uwe_c_lib`
7. run the Rust binary : `rs/ >`  `cargo run -p uwe-rs-bin`