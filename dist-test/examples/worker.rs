#![feature(await_macro, async_await, futures_api, arbitrary_self_types)]

use flatbuffers;
include!("../schema/generated/include.rs");


use log::trace;

use dist_services::worker::consumer::{WorkerResult, Component, ComponentStatus};


#[derive(Clone)]
struct MyWorker;
impl dist_services::worker::consumer::Worker for MyWorker {
    fn initialize(&mut self, ) -> WorkerResult<()> {
        println!("initialize()");
        Ok(())
    }
    fn shutdown(&mut self, ) -> WorkerResult<()> {
        println!("shutdown()");
        Ok(())
    }
    fn on_component_arrived<T>(&mut self, component: Component<T>) -> WorkerResult<()> {
        println!("on_component_arrived()");
        Ok(())
    }
    fn on_component_pending<T>(&mut self, component: Component<T>, status: ComponentStatus) -> WorkerResult<()> {
        println!("on_component_pending()");
        Ok(())
    }
    fn on_component_left<T>(&mut self, component: Component<T>) -> WorkerResult<()> {
        println!("on_component_left()");
        Ok(())
    }
    fn on_component_updated<T>(&mut self, component: Component<T>, status: ComponentStatus) -> WorkerResult<()> {
        println!("on_component_updated()");
        Ok(())
    }

    fn tick(&mut self, ) -> WorkerResult<()> {
        Ok(())
    }
}
fn main() -> std::io::Result<()> {
    env_logger::init();
    let mut worker = MyWorker{};

    let config = dist_services::worker::consumer::load_config().unwrap();

    dist_services::worker::consumer::run(config, worker)
}