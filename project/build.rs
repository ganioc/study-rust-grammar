
use std::env::var;

fn main(){
    let project_dir = var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search={}/src", project_dir);
    println!("cargo:rustc-link-lib=static=wmath");

}
