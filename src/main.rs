#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use std::sync::atomic::{AtomicU16, Ordering};

use prometheus::{Opts, Registry, Gauge, TextEncoder, Encoder};
use rocket::State;


// TODO: See Trello - metric
// TODO: Add persistance

// TODO: Add admin action authentication
// TODO: JSON support
// TODO: start FROM scratch and copy the required linked binaries / link statically
// TODO: Add swagger contract
// TODO: Add DMZ support
// TODO: Don't use GET (for adding, setting and taking one) - GETs should be idempotent
#[get("/count")]
fn get_mate(mate_count: State<AtomicU16>) -> String {
    mate_count.load(Ordering::Relaxed).to_string()
}

// TODO: Can I alias it somehow?
#[get("/")]
fn root(mate_count: State<AtomicU16>) -> String {
    mate_count.load(Ordering::Relaxed).to_string()
}

#[get("/add/<amount>")]
fn add_mate(mate_count: State<AtomicU16>, amount: u16) -> String {
    let current_mate_count = mate_count.load(Ordering::Relaxed);

    let new_mate_count = current_mate_count + amount;

    mate_count.store(new_mate_count, Ordering::Relaxed);

    new_mate_count.to_string()
}

#[get("/set/<amount>")]
fn set_mate(mate_count: State<AtomicU16>, amount: u16) -> String {
    mate_count.store(amount, Ordering::Relaxed);

    amount.to_string()
}

#[get("/one")]
fn remove_single_mate(mate_count: State<AtomicU16>) -> &'static str {
    let current_mate_count = mate_count.load(Ordering::Relaxed);

    let new_mate_count = current_mate_count - 1;

    mate_count.store(new_mate_count, Ordering::Relaxed);

    "Smacznego! Możesz zamknąć tę stronę :)"
}

#[get("/metrics")]
fn get_metrics(mate_gauge: State<Gauge>, metrics_registry: State<Registry>, mate_count: State<AtomicU16>) -> String {
    mate_gauge.set(mate_count.load(Ordering::Relaxed).into());

    let mut buffer = vec![];

    TextEncoder::new()
        .encode(&metrics_registry.gather(), &mut buffer)
        .unwrap();

    String::from_utf8(buffer)
        .unwrap()
}

fn main() {
    let mate_count = AtomicU16::new(0);

    // TODO: Less painfull to extract matecount - type with some impls
    // TODO: Type for passing metrics around
    let mate_gauge = Gauge::with_opts(Opts::new("mateometer_mate_count", "amount of mate in our Hackerspace"))
        .unwrap();

    let metrics_registry = Registry::new();

    metrics_registry.register(Box::new(mate_gauge.clone()))
        .unwrap();

    rocket::ignite()
        .manage(metrics_registry)
        .manage(mate_gauge)
        .manage(mate_count)
        .mount("/", routes![
            root,
            get_mate,
            get_metrics,
            add_mate,
            set_mate,
            remove_single_mate,
        ]
    )
    .launch();
}
