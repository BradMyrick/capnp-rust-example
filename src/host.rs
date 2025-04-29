// host.rs
use capnp::capability::Promise;
use capnp_rpc::pry;
use capnp_rpc_example::schema_capnp;

pub struct PointTrackerImpl;

impl schema_capnp::point_tracker::Server for PointTrackerImpl {
    fn add_point(
        &mut self,
        params: schema_capnp::point_tracker::AddPointParams,
        mut results: schema_capnp::point_tracker::AddPointResults,
    ) -> Promise<(), capnp::Error> {
        let params = pry!(params.get());
        let point = pry!(params.get_p());
        let x = point.get_x();
        let y = point.get_y();
        println!("Host: Received point ({}, {})", x, y);
        results.get().set_total_points(1);
        Promise::ok(())
    }
}
