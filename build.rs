use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rustc-link-lib=framework=IOKit");
    println!("cargo:rustc-link-lib=framework=CoreFoundation");
    println!("cargo:rerun-if-changed=wrapper.h");

    let sdk_path = std::process::Command::new("xcrun")
        .args(["--sdk", "macosx", "--show-sdk-path"])
        .output()
        .expect("xcrun failed")
        .stdout;
    let sdk_path = std::str::from_utf8(&sdk_path).unwrap().trim();

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-isysroot{sdk_path}"))
        // Types
        .allowlist_type("io_service_t")
        .allowlist_type("io_object_t")
        .allowlist_type("io_iterator_t")
        .allowlist_type("kern_return_t")
        // Functions
        .allowlist_function("IOServiceGetMatchingService")
        .allowlist_function("IOServiceGetMatchingServices")
        .allowlist_function("IOServiceMatching")
        .allowlist_function("IOIteratorNext")
        .allowlist_function("IOObjectRelease")
        .allowlist_function("IODisplayGetFloatParameter")
        .allowlist_function("IODisplaySetFloatParameter")
        // Constants
        .allowlist_var("kIOMainPortDefault")
        .allowlist_var("kIOMasterPortDefault")
        .allowlist_var("kIODisplayBrightnessKey")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out.join("iokit_bindings.rs"))
        .expect("Couldn't write bindings");
}
