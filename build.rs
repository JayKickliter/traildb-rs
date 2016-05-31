extern crate bindgen;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let mut bindings = bindgen::builder();
    let src = bindings.header("src/ffi/gen.h")
        .builtins()
        .link("traildb", bindgen::LinkType::Dynamic)
        .rust_enums(false)
        .generate()
        .unwrap()
        .to_string();
    let out_dir = "src/ffi";
    let out_path = Path::new(&out_dir).join("mod.rs");
    let mut f = File::create(&out_path).unwrap();
    f.write_all(src.as_bytes()).unwrap();
}
