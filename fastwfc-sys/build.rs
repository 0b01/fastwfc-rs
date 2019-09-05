extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
    cc::Build::new()
        .cpp(true)
        .include("fast-wfc/src/include")
        .include("fast-wfc/src/lib")
        .file("wrapper.cpp")
        .file("fast-wfc/src/lib/propagator.cpp")
        .file("fast-wfc/src/lib/wfc.cpp")
        .file("fast-wfc/src/lib/wave.cpp")
        .flag("-std=c++17")
        .compile("fastwfc");

    let bindings = bindgen::Builder::default()
        .header("wrapper.cpp")
        .clang_arg("-std=c++17")
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-I./fast-wfc/src/include")
        .opaque_type("std::.*")
        .opaque_type("Wave")
        .opaque_type("Array2D")
        .opaque_type("Array3D")
        .whitelist_type("ArrayColor2D")
        .whitelist_function(".+array_color_2d")
        .whitelist_function("array_color_2d.+")
        .whitelist_function("run_overlapping")
        .whitelist_function("run_overlapping_with_seed")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
