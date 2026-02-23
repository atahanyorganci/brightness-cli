use std::{env, io::Write, path::PathBuf};

fn main() {
    println!("cargo:rustc-link-search=framework=/System/Library/PrivateFrameworks");
    println!("cargo:rustc-link-lib=framework=DisplayServices");
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
        .allowlist_function("DisplayServicesGetBrightness")
        .allowlist_function("DisplayServicesSetBrightness")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    let bindings_path = out.join("display_services_bindings.rs");
    let code = bindings.to_string();
    // Recent Rust requires `unsafe extern "C"` blocks; bindgen may generate plain `extern "C"`.
    let code = code.replace("extern \"C\"", "unsafe extern \"C\"");
    std::fs::File::create(&bindings_path)
        .and_then(|mut f| f.write_all(code.as_bytes()))
        .expect("Couldn't write bindings");
}
