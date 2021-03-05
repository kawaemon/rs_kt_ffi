fn main() {
    // TODO: switch to release library on release build
    println!("cargo:rustc-link-search=kt/build/bin/native/debugStatic");
    println!("cargo:rustc-link-lib=kt");
    println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("./binding.rs")
        .expect("Couldn't write bindings!");
}
