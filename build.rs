extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    //println!("cargo:root=mat91lib/sam4s");
    let bindings = bindgen::Builder::default()
        .clang_arg("-Imat91lib")
        .clang_arg("-Imat91lib/sam4s")
        .clang_arg("-Imat91lib/sam4s/atmel")
        .header("wrapper.h")
        .generate()
        .expect("Could not generate bindings!");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Could not write bindings");
}
