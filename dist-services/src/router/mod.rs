use std::sync::{Arc, RwLock};
use std::pin::Pin;
use futures::{ Poll,
    future::{ Future, FutureExt, TryFutureExt },
    task::{LocalWaker},
};
use std::sync::atomic::{AtomicU64};

use tarpc::server::Handler;
use log::trace;

pub mod rpc;

type Vec3 = nalgebra::Vector3<f64>;

#[derive(Clone, Debug, PartialEq)]
struct Box {
    origin:  Vec3,
    extents: Vec3,
}
impl Default for Box {
    fn default() -> Self {
        Self {
            origin: Vec3::new(0.0, 0.0, 0.0),
            extents: Vec3::new(0.0, 0.0, 0.0),
        }
    }
}


#[derive(Clone, Debug)]
pub struct WorkerInfo {
    id: u64,
    remote: crate::worker::rpc::Remote,
    area: Box,
}
impl WorkerInfo {
    pub fn new(id: u64, remote: crate::worker::rpc::Remote, ) -> Self {
        Self {
            id,
            remote,
            area: Box::default(),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct State {
    workers: Vec<WorkerInfo>,
    id_increment: Arc<AtomicU64>,
}
impl State {
    pub fn new() -> Self {
        Self {
            workers: Vec::new(),
            id_increment: Arc::new(AtomicU64::new(0)),
        }
    }
}

pub async fn new(config: Config) -> std::io::Result<Service> {
    let state =  Arc::new(RwLock::new(State::new()));

    let transport = tarpc_bincode_transport::listen(&config.listen_address.parse().unwrap())?;
    let address = transport.local_addr();
    trace!("router: Listening: {}", address);

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

#[derive(Default, Debug, serde::Serialize, serde::Deserialize)]
pub struct Config {
    listen_address: String,
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

impl Future for Service {
    type Output = ();

    fn poll(self: Pin<&mut Self>, lw: &LocalWaker) -> Poll<()> {
        // Run continously
        lw.wake();
        Poll::Pending
    }
}

pub mod consumer {
    pub fn load_config() -> Result<super::Config, config::ConfigError> {
        let mut config = config::Config::new();

        config.merge(config::File::with_name("RouterConfig.toml"));
        config.merge(config::Environment::with_prefix("DIST"))?;
        config.merge(config::Environment::with_prefix("DIST_ROUTER"))?;

        config.try_into()
    }

    async fn internal_run(config: super::Config) -> std::io::Result<()> {
        let service = await!(super::new(config))?;

        await!(service);

        Ok(())
    }

    pub fn run(config: super::Config) -> std::io::Result<()> {
        use futures::compat::TokioDefaultSpawner;
        use futures::future::{TryFutureExt, FutureExt};

        // Spin up a service for this worker
        tarpc::init(TokioDefaultSpawner);

        println!("Initializing router");

        tokio::run(
            internal_run(config)
                .map_err(|e| eprintln!("Oh no: {}", e))
                .boxed()
                .compat(),
        );
        Ok(())
    }
}