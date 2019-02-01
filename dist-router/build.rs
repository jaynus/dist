use std::path::Path;

fn main() {
    let file = Path::new("../capnp/router.capnp");
    println!("cargo:rerun-if-changed={}", file.to_str().unwrap());

    ::capnpc::CompilerCommand::new()
        .file(file)
        .edition(capnpc::RustEdition::Rust2018)
        .output_path("")
        .src_prefix(env!("PWD"))
       // .extensions()
        .run().unwrap();
}