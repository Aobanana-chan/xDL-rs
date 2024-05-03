use std::{env::{self, set_var}, path::{self, PathBuf}};
use glob::glob;

fn main() {
    let files = glob("xdl_src/*.c").unwrap();
    let mut builder = cc::Build::new();
    for file in files{
        if let Ok(file) = file{
            builder.file(file);
        }
    }
    builder.flag("-std=c17");
    builder.include("xdl_src/include");
    builder.compile("xdl");

    if !path::Path::new("src/bindings.rs").exists(){
        set_var("RUST_BACKTRACE", "1");
        let sysroot = env::var("SYSROOT").unwrap();
        let bindings = bindgen::builder()
        .header("xdl_src/include/xdl.h")
        .size_t_is_usize(true)
        .clang_arg(format!("--sysroot={}", sysroot))
        .clang_args(vec!["-I", "usr/include"])
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");
        let out_path = PathBuf::from("src/");
        bindings
            .write_to_file(out_path.join("bindings.rs"))
            .expect("Couldn't write bindings!");
    }

}