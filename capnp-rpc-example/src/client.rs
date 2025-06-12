//src/client.rs
use crate::token_capnp::{root, token};
use capnp_rpc::{RpcSystem, rpc_twoparty_capnp, twoparty};
use futures::AsyncReadExt;
use tokio::task::LocalSet;
use std::net::ToSocketAddrs;

// This function demonstrates the correct flow: obtaining capabilities in order
// and using them to perform authorized actions.
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = ::std::env::args().collect();
    if args.len() != 4 {
        println!("usage: {} client HOST:PORT PASSWORD", args[0]);
        return Ok(());
    }

    let addr = args[2]
        .to_socket_addrs()?
        .next()
        .expect("could not parse address");
    let password = &args[3];
    if password.is_empty() {
        println!("Password cannot be empty");
        return Ok(());
    }
    println!("Connecting to {}", addr);

    let local = LocalSet::new();
    local.run_until(async move {
        let stream = tokio::net::TcpStream::connect(&addr).await?;
        stream.set_nodelay(true)?;

        let (reader, writer) =
            tokio_util::compat::TokioAsyncReadCompatExt::compat(stream).split();
        let network = twoparty::VatNetwork::new(
            futures::io::BufReader::new(reader),
            futures::io::BufWriter::new(writer),
            rpc_twoparty_capnp::Side::Client,
            Default::default(),
        );

        let mut rpc_system = RpcSystem::new(Box::new(network), None);

        // Bootstrap to the Root capability.
        let root_client: root::Client = rpc_system.bootstrap(rpc_twoparty_capnp::Side::Server);

        tokio::task::spawn_local(rpc_system);

        // Get the Auth capability from Root.
        let get_auth_req = root_client.get_auth_request();
        let get_auth_resp = get_auth_req.send().promise.await?;
        let auth_client = get_auth_resp.get()?.get_auth()?;

        // Login to get the Token capability.
        let mut login_req = auth_client.login_request();
        login_req.get().set_password(password);

        let login_resp = login_req.send().promise.await?;
        let token_client = login_resp.get()?.get_token()?;

        // Mint using the Token capability.
        let mint_req = token_client.mint_request();
        let reply = mint_req.send().promise.await?;
        println!("Mint succeeded: {:?}", reply.get()?.get_result()?);

        Ok(())
    }).await
}

// This function attempts to use a capability the client does not possess.
pub async fn fail() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = ::std::env::args().collect();
    if args.len() != 3 {
        println!("usage: {} fail HOST:PORT", args[0]);
        return Ok(());
    }

    let addr = args[2]
        .to_socket_addrs()?
        .next()
        .expect("could not parse address");

    println!("Connecting to {} (fail example)", addr);

    let local = LocalSet::new();
    local.run_until(async move {
        let stream = tokio::net::TcpStream::connect(&addr).await?;
        stream.set_nodelay(true)?;

        let (reader, writer) =
            tokio_util::compat::TokioAsyncReadCompatExt::compat(stream).split();
        let network = twoparty::VatNetwork::new(
            futures::io::BufReader::new(reader),
            futures::io::BufWriter::new(writer),
            rpc_twoparty_capnp::Side::Client,
            Default::default(),
        );

        let mut rpc_system = RpcSystem::new(Box::new(network), None);

        // Bootstrap to the Root capability.
        let _root_client: root::Client = rpc_system.bootstrap(rpc_twoparty_capnp::Side::Server);

        tokio::task::spawn_local(rpc_system);

        // Attempt to use a Token capability without having one.
        // simulating a missing capability by not obtaining it.
        let maybe_token: Option<token::Client> = None;

        if let Some(token) = maybe_token {
            let req = token.mint_request();
            let reply = req.send().promise.await?;
            println!("Unexpected success: {:?}", reply.get()?.get_result()?);
        } else {
            // This is the expected path: we do not have the capability, so we cannot even attempt the call.
            println!("Not Authorized: no capability present (as expected)");
        }

        Ok(())
    }).await
}

