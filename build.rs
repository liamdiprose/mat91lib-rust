extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::fs::File;
use std::io::{Read,Write};
use std::vec::Vec;
use std::process::Command;

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

    //gcc::compile_library("libmat91lib.a", &["mat91lib/pwm/pwm.c"]);
//    gcc::Build::new()
//        .include("mat91lib")
//        .include("mat91lib/sam4s/")
//        .include("mat91lib/sam4s/atmel")
//        .define("__SAM4S__", None)
//        .define("__SAM4S8B__", None)
//        .file("mat91lib/mat91lib.h")
//        .file("mat91lib/sam4s/mcu.c")
//        .file("mat91lib/pwm/pwm.c")
//        .file("mat91lib/sam4s/pio.c")
//        .file("mat91lib/sam4s/mcu_sleep.c")
//        .warnings_into_errors(false)
//        .warnings(false)
//        .compile("mat91lib");

    Command::new("make")
        .args(&["-f", "mat91lib.mk",
                "MCU=SAM4S8B",
                "MAT91LIB_DIR=mat91lib/",
                "PERIPHERALS=\"pwm\"",
                "RUN_MODE=ROM",
                "OPT=-01",
                out_path.join("libmath91lib.a").as_path().display().to_string().as_str()
        ]);
}
