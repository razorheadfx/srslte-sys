# srslte-sys
Unsafe Rust bindings for [srsLTE](https://github.com/srsLTE/srslte) on linux generated via [bindgen](https://github.com/rust-lang-nursery/rust-bindgen).  
These are bindings for the components written in C (i.e. [srslte/src/lib*](https://github.com/srsLTE/srsLTE/tree/master/lib), but may be extended to cover the high-level C++ components at some point (once bindgen can handle C++).  

## How to use it
1. Clone [srsLTE](https://github.com/srsLTE/srsLTE) from GitHub
2. Build srsLTE and install them somewhere (i.e. adding ```-DCMAKE_INSTALL_PREFIX=<install dir>``` to the cmake command)
3. export the install directory ```export SRSLTE_DIR=<install dir>``` and/or add it to your .bashrc/.zshrc.
4. ```cargo build``` to make the bindings and run ```cargo test``` to run bindgen's automatically generated layout tests

## Dependencies
Native  
* [bindgen dependencies](https://rust-lang-nursery.github.io/rust-bindgen/requirements.html)
* [srsLTE dependencies](https://github.com/srsLTE/srsLTE#build-instructions)

## Usage
Cargo.toml
```
[dependencies]
srslte-sys = {git = "https://github.com/razorheadfx/srslte-sys"}
```
lib.rs
```
extern crate srslte_sys as srslte;
```

## Gotchas
* Generates bindings for srslte_rf components but does not link them, this is to prevent linking errors when using various versions of hackrf or libuhd. If you want the build script to link them, enable the ```srslte_rf``` feature in your Cargo.toml.



## TODO
* Build a safe wrapper in another project
* Publish on crates.io
