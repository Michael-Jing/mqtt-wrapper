extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {


    // println!("cargo:rustc-link-search={}/libs/arm64-v8a", manifest_dir);
    // println!("cargo:rustc-link-search={}/libs/x64", manifest_dir);
    cc::Build::new()
        .file("src/core_mqtt.c")
        .file("src/core_mqtt_serializer.c")
        .file("src/core_mqtt_state.c")
        .compile("libmqtt.a");

    println!("cargo:rustc-link-lib=mqtt");
    let bindings = bindgen::Builder::default()
        .header("src/wrapper.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

}
