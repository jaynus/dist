#![feature(await_macro, async_await, futures_api, arbitrary_self_types)]

use flatbuffers;
include!("../schema/generated/include.rs");

use log::trace;


fn main() -> std::io::Result<()> {
    env_logger::init();
    let config = dist_services::router::consumer::load_config().unwrap();

    dist_services::router::consumer::run(config)
}