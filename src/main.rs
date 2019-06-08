use canteen::*;
use canteen::utils;
use prometheus::{Opts, Registry, Gauge, TextEncoder, Encoder};


// TODO: Fix unsafe
// TODO: Add persistance
// TODO: https://docs.rs/prometheus/0.6.1/prometheus/#static-metrics
// TODO: Add admin action authentication
// TODO: JSON support
// TODO: Fix metrics passing - instead of static use handler
// TODO: start FROM scratch and copy the required linked binaries / link statically
// TODO: Add swagger contract
// TODO: Add DMZ support

static mut ble: i32 = 0;

fn main()
{
    let mut cnt = Canteen::new();

    // TODO: Parametrize - PORT + DEV (127.0.0.1)
    cnt.bind(("0.0.0.0", 8080));

    cnt.add_route("/mate", &[Method::Get], get_mate);
    cnt.add_route("/metrics", &[Method::Get], get_metrics);

    // TODO: Don't use get here - GETs should be idempotent
    cnt.add_route("/took_one", &[Method::Get], remove_single_mate);
    cnt.add_route("/add/<int:amount>", &[Method::Get], add_mate);

    // aliases:
    cnt.add_route("/", &[Method::Get], get_mate); // alias for /mate
    cnt.add_route("/one", &[Method::Get], remove_single_mate); // alias for took_one
    
    // default to 404 - without it this hangs :c
    cnt.set_default(utils::err_404);

    cnt.run();
}

fn get_mate(_req: &Request) -> Response
{
    unsafe {
        utils::make_response(format!("{}", ble), "text/plain", 200)
    }
}

fn add_mate(req: &Request) -> Response
{
    let amount: i32 = req.get("amount");

    let dddd = alter_mate_amount(amount);

    utils::make_response(format!("{}", dddd), "text/plain", 200)
}

fn remove_single_mate(req: &Request) -> Response
{
    let dddd = alter_mate_amount(-1);

    utils::make_response(format!("{}", dddd), "text/plain", 200)
}

fn alter_mate_amount(fuj: i32) -> i32
{
    unsafe {
        ble += fuj;
        ble
    }
}

fn get_metrics(_req: &Request) -> Response
{
    let r = Registry::new();
    let counter_opts = Opts::new("mateometer_mate_count", "amout of mate in our Hackerspace");
    let counter = Gauge::with_opts(counter_opts).unwrap();

    unsafe {
        counter.set(ble.into());
    }

    r.register(Box::new(counter.clone())).unwrap();

    let mut buffer = vec![];
    let encoder = TextEncoder::new();
    let metric_families = r.gather();
    encoder.encode(&metric_families, &mut buffer).unwrap();

    utils::make_response(format!("{}", String::from_utf8(buffer).unwrap()), "text/plain", 200)
}
