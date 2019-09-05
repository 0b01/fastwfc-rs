extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let dst = cmake::build("fast-wfc");
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=fastwfc");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-std=c++17")
        .clang_arg("-x")
        .clang_arg("c++")

        .opaque_type("std::.*")
        .opaque_type("Wave")
        .opaque_type("Array2D")
        .opaque_type("Array3D")

        .whitelist_type("ArrayColor2D")
        .whitelist_function("ArrayColor2D::set_width")
        .whitelist_function("ArrayColor2D::set_height")

        .whitelist_function("run_overlapping")

        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

