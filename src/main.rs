#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
use prometheus::{Opts, Registry, Gauge, TextEncoder, Encoder};


// TODO: Fix unsafe
// TODO: Fix metrics - use global registry xd
// TODO: Add persistance
// TODO: Add admin action authentication
// TODO: JSON support
// TODO: start FROM scratch and copy the required linked binaries / link statically
// TODO: Add swagger contract
// TODO: Add DMZ support
// TODO: Don't use GET (for adding, setting and taking one) - GETs should be idempotent


static mut ble: i32 = 0;


#[get("/count")]
fn get_mate() -> String {
    unsafe {
       format!("{}", ble)
    }
}

// TODO: Can I alias it somehow?
#[get("/")]
fn root() -> String {
    unsafe {
       format!("{}", ble)
    }
}

#[get("/add/<amount>")]
fn add_mate(amount: i32) -> String {
    let dddd = alter_mate_amount(amount);

    format!("{}", dddd)
}

#[get("/set/<amount>")]
fn set_mate(amount: i32) -> String {
    let dddd = alter_mate_amount(- alter_mate_amount(0) + amount);

    format!("{}", dddd)
}

#[get("/one")]
fn remove_single_mate() -> &'static str {
    alter_mate_amount(-1);

    "Smacznego! Możesz zamknąć tę stronę :)"
}

fn alter_mate_amount(fuj: i32) -> i32 {
    unsafe {
        ble += fuj;
        ble
    }
}

#[get("/metrics")]
fn get_metrics() -> String {
    let r = Registry::new();
    let counter_opts = Opts::new("mateometer_mate_count", "amount of mate in our Hackerspace");
    let counter = Gauge::with_opts(counter_opts).unwrap();

    unsafe {
        counter.set(ble.into());
    }

    r.register(Box::new(counter.clone())).unwrap();

    let mut buffer = vec![];
    let encoder = TextEncoder::new();
    let metric_families = r.gather();
    encoder.encode(&metric_families, &mut buffer).unwrap();

    String::from_utf8(buffer).unwrap()
}

fn main() {
    rocket::ignite().mount("/", routes![
        root,
        get_mate,
        get_metrics,
        add_mate,
        set_mate,
        remove_single_mate,
    ]).launch();
}
