// build.rs
fn main() {
    println!("cargo::rerun-if-changed=schemas/token.capnp");
    capnpc::CompilerCommand::new()
        .file("schemas/token.capnp")
        .run()
        .expect("capnp compile failed");
}
