extern crate cmake;
extern crate bindgen;

use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

//clone the repo to...
const SRCPATH: &'static str = "target/srslte_sources";

//defaults, are override by environment variables
//SRSLTE_SYS_{REPO,BRANCH,COMMIT}
const REPO: &'static str = "https://github.com/srsLTE/srslte";
const BRANCH: &'static str = "master";
const COMMIT: &'static str = "HEAD";
const LINK_VOLK: bool = true;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let gen_libpath = out_dir.join("lib");
    let gen_include_path = PathBuf::from(out_dir).join("include");

    let main_header = gen_include_path.join("srslte/srslte.h");
    let rf_header = gen_include_path.join("srslte/phy/rf/rf.h");

    //use volk, defaults to true
    let link_volk: bool = match option_env!("SRSLTE_SYS_VOLK") {
        Some(b) => std::str::FromStr::from_str(b).unwrap_or_else(|_| LINK_VOLK),
        None => LINK_VOLK,
    };


    //if the header does not exist assume we need to rebuild
    if !main_header.exists() || !rf_header.exists() {
        build_srslte();
    }

    let bindings = bindgen::Builder::default()
        .header(format!("{}", main_header.display()))
        .header(format!("{}", rf_header.display()))
        .clang_arg(format!("-I{}", gen_include_path.display()))
        .hide_type("FP_NORMAL")
        .hide_type("FP_NAN")
        .hide_type("FP_INFINITE")
        .hide_type("FP_ZERO")
        .hide_type("FP_SUBNORMAL")
        .constified_enum("*")
        .link_static("srslte_common")
        .link_static("srslte_phy")
        .link_static("srslte_radio")
        .link_static("srslte_upper")
        .link_static("srslte_asn1")
        .link("srslte_rf")
        .generate()
        .expect("Unable to generate bindings");

    //spit the bindings into a file
    let gen_bindings = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");

    bindings.write_to_file(gen_bindings).expect(
        "Couldn't write bindings!",
    );

    println!("cargo:rustc-link-search=native={}", gen_libpath.display());


    if link_volk {
        println!("cargo:rustc-flags=-l dylib=volk");
    }
}

fn build_srslte() {
    let srces = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join(SRCPATH);

    //and checkout
    let repo: &str = option_env!("SRSLTE_SYS_REPO").unwrap_or_else(|| REPO);
    let branch: &str = option_env!("SRSLTE_SYS_BRANCH").unwrap_or_else(|| BRANCH);
    let commit: &str = option_env!("SRSLTE_SYS_COMMIT").unwrap_or_else(|| COMMIT);


    //if it isnt there clone it
    if !Path::new(&srces.join(".git")).exists() {

        Command::new("git")
            .arg("clone")
            .arg(format!("--branch={}", branch))
            .arg(repo)
            .arg(&srces)
            .output()
            .expect(&format!(
                "Cloning {} of {} to {} failed",
                branch,
                repo,
                srces.display()
            ));

        Command::new("git")
            .arg("reset")
            .arg("--hard")
            .arg(commit)
            .output()
            .expect(&format!("Resetting to {} failed", commit));


    }

    cmake::Config::new(format!("{}", srces.display()))
        .define("ENABLE_BLADERF", "OFF")
        .build();

}
