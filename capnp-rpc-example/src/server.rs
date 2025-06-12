//src/server.rs
use capnp::capability::Promise;
use capnp_rpc::{RpcSystem, pry, rpc_twoparty_capnp, twoparty};
use futures::AsyncReadExt;

use crate::token_capnp::{root, auth, token};

struct TokenImpl;

impl token::Server for TokenImpl {
    fn mint(
        &mut self, 
        _: token::MintParams,
        mut results: token::MintResults,
    ) -> Promise<(), capnp::Error> {
        results.get().set_result("Minted successfully!");
        Promise::ok(())
    }
}

struct AuthImpl;

impl auth::Server for AuthImpl {
    fn login(
        &mut self,
        params: auth::LoginParams,
        mut results: auth::LoginResults,
    ) -> Promise<(), capnp::Error> {
        let password = pry!(pry!(params.get()).get_password());
        if password == "secret" {
            let token = capnp_rpc::new_client(TokenImpl);
            results.get().set_token(token);
            Promise::ok(())
        } else {
            Promise::err(capnp::Error::failed("Invalid password".into()))
        }
    }
}

struct RootImpl;

impl root::Server for RootImpl {
    fn get_auth(
        &mut self,
        _params: root::GetAuthParams,
        mut results: root::GetAuthResults,
    ) -> Promise<(), capnp::Error> {
        results.get().set_auth(capnp_rpc::new_client(AuthImpl));
        Promise::ok(())
    }
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:4000";
    println!("Server listening on {}", addr);

    tokio::task::LocalSet::new()
        .run_until(async move {
            let listener = tokio::net::TcpListener::bind(&addr).await?;
            let root_client: root::Client = capnp_rpc::new_client(RootImpl);

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

                let rpc_system =
                    RpcSystem::new(Box::new(network), Some(root_client.clone().client));

                tokio::task::spawn_local(rpc_system);
            }
        })
        .await
}
