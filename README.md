# srslte-sys
Unsafe Rust bindings for [srsLTE](https://github.com/srsLTE/srslte) on linux generated via [bindgen](https://github.com/rust-lang-nursery/rust-bindgen).  
These are bindings for the components written in C (i.e. [srslte/src/lib*](https://github.com/srsLTE/srsLTE/tree/master/lib), but may be extended to cover the high-level C++ components at some point.

## How it works
1. Clone [srsLTE](https://github.com/srsLTE/srsLTE) from GitHub
2. Build srsLTE using the [cmake](https://crates.io/crates/cmake) crate
3. Use rust-bindgen to generate the bindings

## Dependencies
Native  
* git, gcc
* [bindgen dependencies](https://rust-lang-nursery.github.io/rust-bindgen/requirements.html)
* [srsLTE dependencies](https://github.com/srsLTE/srsLTE#build-instructions)

Rust  
* [bindgen](https://crates.io/crates/bindgen)
* [cmake](https://crates.io/crates/cmake)


## Customizing the Build
The build can be customized via environment variables  
* ``` SRSLTE_SYS_REPO ``` : Set the repo URL to use (defaults to https://github.com/srsLTE/srslte)
* ``` SRSLTE_SYS_BRANCH ``` : Set the branch (defaults to master)
* ``` SRSLTE_SYS_COMMIT ``` : Set the commit to check out (defaults to HEAD)
* ``` SRSLTE_SYS_VOLK ``` : Set whether to link [VOLK](http://libvolk.org/) for downstream build processes (defaults to true)
  
Other
* ```cargo clean``` also removes the sources (they are cloned to target/srslte_sources)

## TODO
* Build a safe wrapper in another project
* De-clutter build-script
* Switch to bindgen 0.31.* once it is stable enough to build the bindings without failing; in build.rs replace ```hide_type()``` with ```blacklisted_type()``` and remove ```constified_enums()``` (since it is the default in 0.31.*)
* Generate bindings for srsENB and srsUE CPP headers/components
