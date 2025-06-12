
fn main() {
    println!("cargo::rerun-if-changed=schemas/schema.capnp");
    capnpc::CompilerCommand::new()
        .file("schemas/schema.capnp")
        .run()
        .expect("capnp compile failed");
}
