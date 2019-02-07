use std::sync::{Arc, RwLock};
use tarpc::server::Handler;
use futures::{
    stream::StreamExt,
    future::{ FutureExt, TryFutureExt },
};

pub mod rpc;


#[derive(Clone, Debug)]
pub struct State {
    pub router_client: Option<crate::router::rpc::Remote>
}
impl State {
    pub fn new() -> Self {
        Self {
            router_client: None,
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

pub async fn new(router_address: std::net::SocketAddr) -> std::io::Result<Service> {
    let state =  Arc::new(RwLock::new(State::new()));

    let transport = tarpc_bincode_transport::listen(&"0.0.0.0:0".parse().unwrap())?;
    let local_address = transport.local_addr();
    println!("worker: Listening: {} (router: {})", local_address, router_address);

    let rpc_server = tarpc::server::new(tarpc::server::Config::default())
        .incoming(transport)
        .respond_with(rpc::serve(rpc::Server::new(state.clone())));

    // Spawn the server future
    tokio_executor::spawn(rpc_server.unit_error().boxed().compat());

    // Register ourselves with the provided router
    let info = Arc::new(crate::router::rpc::RemoteInfo::new(0, &router_address));
    state.write().unwrap().router_client = Some(await!(crate::router::rpc::Remote::bootstrap(info))?);

    let mut s = state.read().unwrap().clone();
    let register_response = await!(s.router_client.as_mut().unwrap().client().register_worker(tarpc::context::current(), local_address))?;

    Ok(Service {
        state,
        address: local_address,
    })
}