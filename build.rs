extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::fs::File;
use std::io::{Read,Write};
use std::vec::Vec;

fn main() {
    //println!("cargo:root=mat91lib/sam4s");
    let mut config_c_file = File::create("mat91lib/config.h").expect("Could not create config.h file");
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

    let mut lib_file = File::create("src/lib.rs").expect("Could not create src/lib file");
    let mut prelude = File::open("src/prelude.rs").expect("Could not open prelude file");
    let mut bindings = File::open(out_path.join("bindings.rs")).expect("Could not open bindings.rs");

    let mut prelude_contents = Vec::with_capacity(200);
    let mut bindings_content = Vec::with_capacity(5000);

    prelude.read_to_end(&mut prelude_contents).expect("Could not read prelude.rs file");
    lib_file.write_all(&prelude_contents);


    bindings.read_to_end(&mut bindings_content).expect("Could not read bindings.rs file");
    lib_file.write_all(&bindings_content).expect("Could not write bindings to src/lib.rs");


}
