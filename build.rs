// build.rs
fn main() {
    capnpc::CompilerCommand::new()
        .import_path("schema")
        .file("schema/schema.capnp")
        .src_prefix("schema")
        .run()
        .expect("Failed to compile schema");
}
