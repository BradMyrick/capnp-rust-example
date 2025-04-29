// main.rs
mod client;
mod host;

use capnp_rpc::{rpc_twoparty_capnp, twoparty, RpcSystem};
use capnp_rpc_example::schema_capnp;
use futures::AsyncReadExt;
use std::{error::Error, net::SocketAddr};
use tokio::net::TcpListener;

pub async fn start_server(addr: &str) -> Result<(), Box<dyn Error>> {
    let addr: SocketAddr = addr.parse()?;

    tokio::task::LocalSet::new()
        .run_until(async move {
            let listener = TcpListener::bind(&addr).await?;
            let client: schema_capnp::point_tracker::Client =
                capnp_rpc::new_client(host::PointTrackerImpl);

            loop {
                let (stream, _) = listener.accept().await?;
                stream.set_nodelay(true)?;
                let (reader, writer) =
                    tokio_util::compat::TokioAsyncReadCompatExt::compat(stream).split();
                let network = twoparty::VatNetwork::new(
                    futures::io::BufReader::new(reader),
                    futures::io::BufWriter::new(writer),
                    rpc_twoparty_capnp::Side::Server,
                    Default::default(),
                );
                let rpc_system = RpcSystem::new(Box::new(network), Some(client.clone().client));
                tokio::task::spawn_local(rpc_system);
            }
        })
        .await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:4000";

    // Use LocalSet for !Send Cap'n Proto RPC
    let local = tokio::task::LocalSet::new();

    local
        .run_until(async {
            // Start server in a background local task
            let _server = tokio::task::spawn_local(async move {
                // Will run forever unless we break; for demo, let client run and then exit
                // TODO add a shutdown channel for graceful exit in real code
                let _ = start_server(addr).await;
            });

            // Give the server time to start up
            tokio::time::sleep(std::time::Duration::from_millis(200)).await;

            // Start client, do a single RPC call
            client::add_point(addr, 8.14, 2.71).await?;
            // After client is done, exit program gracefully
            // TODO, signal the server to shut down here
            std::process::exit(0);
            #[allow(unreachable_code)]
            Ok::<(), Box<dyn Error>>(())
        })
        .await?;

    Ok(())
}
