use std::{path::PathBuf, env};

fn main() {

    let jdk_location = std::env::var("JAVA_HOME").expect("JAVA_HOME not set");

    println!("cargo:rustc-link-search={}\\lib",jdk_location);
    println!("cargo:rustc-link-lib=jvm");
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
    // The input header we would like to generate
    // bindings for.
    .header(format!("{}\\include\\jni.h",jdk_location))
    .header(format!("{}\\include\\jvmti.h",jdk_location))
    .header(format!("{}\\include\\win32\\jni_md.h",jdk_location))
    // Tell cargo to invalidate the built crate whenever any of the
    // included header files changed. //a
    .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    // Finish the builder and generate the bindings.
    .generate()
    // Unwrap the Result and panic on failure.
    .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        // .write_to_file("./binds.rs")
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}