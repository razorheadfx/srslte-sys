extern crate bindgen;
extern crate pkg_config;

use std::env;
use std::path::PathBuf;

fn main() {
    let srslte_dir = {
        let dir = env::var("SRSLTE_DIR").expect(
            "Please set environment variable SRSLTE_DIR to point at the built version of srslte",
        );
        let p = PathBuf::from(dir);
        assert!(
            p.is_dir() && p.exists(),
            "The given SRSLTE_DIR is not a valid directory"
        );
        p
    };

    let bindings_out = PathBuf::from(env::var("OUT_DIR").unwrap()).join("srslte_bindings.rs");

    let libdir = srslte_dir.join("lib/");
    let include_dir = srslte_dir.join("include/");

    let main_header = include_dir.join("srslte/srslte.h");
    let rf_header = include_dir.join("srslte/phy/rf/rf.h");

    //if the header does not exist assume we need to rebuild
    let bindings = bindgen::Builder::default()
        .header(format!("{}", main_header.display()))
        .header(format!("{}", rf_header.display()))
        .clang_arg(format!("-I{}", include_dir.display()))
        .blacklist_type("FP_NORMAL")
        .blacklist_type("FP_NAN")
        .blacklist_type("FP_INFINITE")
        .blacklist_type("FP_ZERO")
        .blacklist_type("FP_SUBNORMAL")
        .constified_enum_module("*")
        .generate()
        .expect("Unable to generate bindings");

    //spit the bindings into a file

    bindings
        .write_to_file(bindings_out)
        .expect("Couldn't write bindings!");

    // add the bindings dir to the linker path
    println!("cargo:rustc-link-search=native={}", libdir.display());

    // modify the linker paths and link library components
    pkg_config::probe_library("fftw3").expect("Failed to find fftw3; is it installed?");

    // link volk of installed (assumes srslte is built with it)
    if pkg_config::probe_library("volk").is_ok() {
        println!("cargo:rustc-flags=-l dylib=volk");
    }

    // binding linking doesnt work, must do it this way
    println!("cargo:rustc-flags=-l dylib=srslte_phy");
    println!("cargo:rustc-flags=-l dylib=srslte_common");
    // we're not linking srslte_rf since that loads libuhd, which will be loaded by soapysdr (and probably use a different version)
}
