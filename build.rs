extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main () {
    println!("cargo:rustc-link-lib=mat91lib");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .generate()
        .expect("Could not generate bindings!");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Could not write bindings");
}
