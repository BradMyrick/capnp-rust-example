//client.rs
use capnp_rpc::{rpc_twoparty_capnp, twoparty, RpcSystem};
use capnp_rpc_example::schema_capnp;
use futures::{
    io::{BufReader, BufWriter},
    AsyncReadExt, FutureExt,
};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncReadCompatExt;

pub async fn add_point(addr: &str, x: f32, y: f32) -> capnp::Result<()> {
    let stream = TcpStream::connect(addr).await?;
    stream.set_nodelay(true)?;
    let (reader, writer) = stream.compat().split();

    let network = twoparty::VatNetwork::new(
        BufReader::new(reader),
        BufWriter::new(writer),
        rpc_twoparty_capnp::Side::Client,
        Default::default(),
    );

    let mut rpc_system = RpcSystem::new(Box::new(network), None);

    let tracker: schema_capnp::point_tracker::Client =
        rpc_system.bootstrap(rpc_twoparty_capnp::Side::Server);

    tokio::task::spawn_local(rpc_system.map(|_| ()));

    let mut request = tracker.add_point_request();
    {
        let mut point = request.get().init_p();
        point.set_x(x);
        point.set_y(y);
    }

    let response = request.send().promise.await?;
    let total_points = response.get()?.get_total_points();
    println!("Client: Server reports total points: {}", total_points);

    Ok(())
}
