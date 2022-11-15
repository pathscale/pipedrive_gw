fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../codegen");
    println!("cargo:rerun-if-changed=../service");
    println!("cargo:rerun-if-changed=../model");
    // FIXME: this should be included but it triggers rebuild every time
    // println!("cargo:rerun-if-changed=../docs/error_codes/error_codes_en.json");

    codegen::main().unwrap()
}
