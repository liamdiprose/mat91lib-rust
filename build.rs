extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::path::Path;
use std::fs::File;
use std::io::{Read,Write};
use std::vec::Vec;
use std::process::Command;

fn main() {
    //println!("cargo:root=mat91lib/sam4s");
    let mut config_c_file = File::create("mat91lib/config.h").expect("Could not create config.h file");


    let config_c_file = Path::new("config.h");

    std::fs::copy(&config_c_file, &Path::new("mat91lib/config.h")).unwrap();

    let bindings = bindgen::Builder::default()
        .use_core()
        .ctypes_prefix("ctypes")  // Use custom ctypes instead of std::ctypes
        .clang_arg("-Imat91lib")
        .clang_arg("-Imat91lib/sam4s")
        .clang_arg("-Imat91lib/sam4s/atmel")
        .header("wrapper.h")
        .generate()
        .expect("Could not generate bindings!");

    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = PathBuf::from(&out_dir);
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Could not write bindings");


    // Linker (FLASH / SRAM / .bss / etc)
    File::create(out_path.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();

    let mut lib_file = File::create("src/lib.rs").expect("Could not create src/lib file");
    let mut prelude = File::open("src/prelude.rs").expect("Could not open prelude file");
    let mut bindings = File::open(out_path.join("bindings.rs")).expect("Could not open bindings.rs");

    let mut prelude_contents = Vec::with_capacity(200);
    let mut bindings_content = Vec::with_capacity(5000);

    prelude.read_to_end(&mut prelude_contents).expect("Could not read prelude.rs file");
    lib_file.write_all(&prelude_contents);


    bindings.read_to_end(&mut bindings_content).expect("Could not read bindings.rs file");
    lib_file.write_all(&bindings_content).expect("Could not write bindings to src/lib.rs");

    Command::new("git")
        .current_dir("mat91lib")
        .args(&["apply", "../static-target.patch"])
        .output()
        .expect("Applying patch failed");

    let output = Command::new("make")
        .current_dir(&Path::new("mat91lib"))
        .args(&["-f", "mat91lib.mk",
                "MCU=SAM4S8B",
                "MAT91LIB_DIR=.",
                "PERIPHERALS=pwm spi",
                "RUN_MODE=ROM",
                "OPT=-O1",
                format!("{}/libmat91lib.a", out_dir).as_str()
        ])
        .output()
        .expect("Failed to make mat91lib");

    if !output.status.success() {
        panic!(format!("Failed to make: Exit {}\n {}", output.status, String::from_utf8_lossy(&output.stderr)));
    }



//    let mut dest_library = PathBuf::from(&out_dir)
//        .join("libmat91lib.a");
//
//    File::create(&dest_library).unwrap();
//    std::fs::copy(&Path::new("mat91lib/libmat91lib.a"), &dest_library)
//        .expect(format!("Could not copy library at {} to out_path {}", "mat91lib/libmat91lib.a", &out_dir).as_str());

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=mat91lib");
}
