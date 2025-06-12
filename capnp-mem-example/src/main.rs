
mod token_capnp {
    include!(concat!(env!("OUT_DIR"), "/schemas/schema_capnp.rs"));
}
use token_capnp::token;
use capnp::message::{Builder, ReaderOptions};
use capnp::serialize;

fn main() {
    // Build a FooBar message
    let mut message = Builder::new_default();
    let mut tkn = message.init_root::<token::Builder>();
    tkn.set_name("Oko");
    tkn.set_capacity(42);

    // Serialize to bytes
    let words = serialize::write_message_to_words(&message);

    // Deserialize from bytes
    let reader = capnp::serialize::read_message(words.as_slice(), ReaderOptions::new())
        .expect("failed to read message");
    let token_reader = reader
        .get_root::<token_capnp::token::Reader>()
        .expect("failed to get root");

    println!("Token Name: {:?}", token_reader.get_name().unwrap());
    println!("Token Capacity: {:?}", token_reader.get_capacity());
}
