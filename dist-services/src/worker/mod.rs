use std::sync::{Arc, RwLock};
use tarpc::server::Handler;
use futures::{
    future::{ FutureExt, TryFutureExt },
};
use log::{info, trace};
use dist_data::ComponentRef;

pub mod rpc;

#[derive(Clone, Debug, Default)]
pub struct State {
    pub router_remote: Option<crate::router::rpc::Remote>,
    pub id: u64,

    pub local_components: Vec<ComponentRef>,
}
impl State {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Clone)]
pub struct Service {
    state: Arc<RwLock<State>>,
    address: std::net::SocketAddr,
}
impl crate::Service for Service {
    fn address(&self) -> std::net::SocketAddr {
        self.address
    }
}

pub async fn new(router_address: std::net::SocketAddr) -> std::io::Result<Service> {
    let state =  Arc::new(RwLock::new(State::new()));

    let transport = tarpc_bincode_transport::listen(&"0.0.0.0:0".parse().unwrap())?;
    let local_address = transport.local_addr();
    trace!("worker: Listening: {} (router: {})", local_address, router_address);

    let rpc_server = tarpc::server::new(tarpc::server::Config::default())
        .incoming(transport)
        .respond_with(rpc::serve(rpc::Server::new(state.clone())));

    // Spawn the server future
    tokio_executor::spawn(rpc_server.unit_error().boxed().compat());

    // Register ourselves with the provided router
    let info = Arc::new(crate::router::rpc::RemoteInfo::new(&router_address));
    state.write().unwrap().router_remote = Some(await!(crate::router::rpc::Remote::bootstrap(info))?);

    let mut s = state.read().unwrap().clone();
    let res = await!(s.router_remote.as_mut().unwrap().client().register_worker(tarpc::context::current(), local_address))?;
    let id = res?.id;
    state.write().unwrap().id = id;

    info!("Worker registered as ID: {}", id);

    Ok(Service {
        state,
        address: local_address,
    })
}

mod consumer {
    // Trait specifying the actual client of our worker API
    trait WorkerIpc {
        fn message_entity<T>(id: u64, data: &T);
    }

    trait WorkerEvents {
        fn assign_entities(ids: &[u64]);
        fn begin_drop_entities(id: &[u64]);
        fn finalize_drop_entities(id: &[u64]);
    }

}