//src/main.rs
mod client;
mod server;

pub mod token_capnp {
    include!(concat!(env!("OUT_DIR"), "/schemas/token_capnp.rs"));
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = ::std::env::args().collect();
    if args.len() >= 2 {
        match &args[1][..] {
            "client" => return client::run().await,
            "server" => return server::run().await,
            "fail" => return client::fail().await,
            _ => (),
        }
    }

    println!("usage: {} [client | server | fail] ADDRESS", args[0]);
    Ok(())
}
