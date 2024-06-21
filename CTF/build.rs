use std::{env, path::PathBuf};

fn main ()
{
    let mut curr_dir = std::env::current_dir().unwrap();
    curr_dir.push("src");
    println!("cargo:rustc-link-search={}", curr_dir.to_str().unwrap()); // -L $LIB_NAME_PATH
    println!("cargo:rustc-link-lib=static=encrypt_nativelib"); // -l $LIB_NAME

    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("src/crypto.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}