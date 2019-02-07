use std::sync::{Arc, RwLock};
use tarpc::server::Handler;
use futures::{
    stream::StreamExt,
    future::{ FutureExt, TryFutureExt },
};


pub mod rpc;

#[derive(Clone, Debug, Default)]
pub struct State {
    workers: Vec<crate::worker::rpc::Remote>,
}
impl State {
    pub fn new() -> Self {
        Self {
            workers: Vec::new(),
        }
    }
}

#[derive(Clone)]
pub struct Service {
    state: Arc<RwLock<State>>,
    address: std::net::SocketAddr,
}
impl Service {

    pub fn address(&self) -> std::net::SocketAddr {
        self.address.clone()
    }
}

pub async fn new() -> std::io::Result<Service> {
    let state =  Arc::new(RwLock::new(State::new()));

    let transport = tarpc_bincode_transport::listen(&"0.0.0.0:0".parse().unwrap())?;
    let address = transport.local_addr();
    println!("router: Listening: {}", address);

    let rpc_server = tarpc::server::new(tarpc::server::Config::default())
        .incoming(transport)
        .respond_with(rpc::serve(rpc::Server::new(state.clone())));

    // Spawn the server future
    tokio_executor::spawn(rpc_server.unit_error().boxed().compat());

    Ok(Service {
        state,
        address,
    })
}
