use std::{env, path::Path};

fn main() {
    let project_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let cpp_path = Path::new(&project_dir).join("cpp");

    println!("cargo:rerun-if-changed=cpp/");

    cc::Build::new()
        .cpp(true)
        .file(cpp_path.join("math_lib.cpp"))
        .file(cpp_path.join("wrapper.cpp"))
        .include(&cpp_path)
        .compile("mathlib");
}
