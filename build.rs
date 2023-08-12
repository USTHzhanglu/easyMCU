extern crate embed_resource;
fn main() {
    println!("cargo:rerun-if-changed=src/main.rs");
    embed_resource::compile("./ico/icon.rc",embed_resource::NONE);
}