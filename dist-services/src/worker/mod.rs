use std::sync::{Arc, RwLock};
use tarpc::server::Handler;
use std::pin::Pin;
use futures::{ Poll,
               future::{ Future, FutureExt, TryFutureExt },
               task::{LocalWaker},
};
use log::{info, trace};
use dist_data::ComponentRef;

pub mod rpc;

#[derive(Clone, Debug)]
pub struct State<T> {
    pub router_remote: Option<crate::router::rpc::Remote>,
    pub id: u64,
    pub consumer: T,
    pub local_components: Vec<ComponentRef>,
}

#[derive(Clone)]
pub struct Service<T> {
    state: Arc<RwLock<State<T>>>,
    address: std::net::SocketAddr,
}
impl<T> crate::Service for Service<T> {
    fn address(&self) -> std::net::SocketAddr {
        self.address
    }
}
impl<T> futures::Future for Service<T>
    where T: Send + Sync + consumer::Worker
{
    type Output = ();

    fn poll(self: Pin<&mut Self>, lw: &LocalWaker) -> Poll<()> {
        // Tick the inner worker object

        self.state.write().unwrap().consumer.tick();

        // Run continously
        lw.wake();
        Poll::Pending
    }
}


pub async fn new<T>(router_address: std::net::SocketAddr, mut consumer: T) -> std::io::Result<Service<T>>
    where T: Send + Sync + consumer::Worker + 'static
{
    let state =  Arc::new(RwLock::new(State{
        consumer,
        id: 0,
        router_remote: None,
        local_components: Vec::new(),
    }));


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

#[derive(Default, Debug, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub router_address: String,
}

pub mod consumer {
    use std::sync::Arc;
    use failure::{Fail};
    use std::pin::Pin;
    use futures::{ Poll,
                   future::{ Future, FutureExt, TryFutureExt },
                   task::{LocalWaker},
    };

    #[derive(Clone, Fail, Debug)]
    #[fail(display = "my wrapping error")]
    pub enum WorkerError {
        #[fail(display = "my error")]
        Unimplemented,
    }
    pub type WorkerResult<T> = Result<T, WorkerError>;

    pub enum ComponentStatus {
        Stationary,
        Arriving,
        Leaving
    }

    pub struct Component<T> {
        inner: T,
    }

    pub trait Worker : Clone {
        fn initialize(&mut self, ) -> WorkerResult<()> { Err(WorkerError::Unimplemented) }
        fn shutdown(&mut self, ) -> WorkerResult<()> { Err(WorkerError::Unimplemented) }

        fn on_component_arrived<T>(&mut self, component: Component<T>) -> WorkerResult<()> { Err(WorkerError::Unimplemented) }
        fn on_component_pending<T>(&mut self, component: Component<T>, status: ComponentStatus) -> WorkerResult<()>  { Err(WorkerError::Unimplemented) }
        fn on_component_left<T>(&mut self, component: Component<T>) -> WorkerResult<()> { Err(WorkerError::Unimplemented) }
        fn on_component_updated<T>(&mut self, component: Component<T>, status: ComponentStatus) -> WorkerResult<()> { Err(WorkerError::Unimplemented) }

        fn tick(&mut self, ) -> WorkerResult<()> { Err(WorkerError::Unimplemented) }
    }

    async fn internal_run<T>(config: super::Config, worker: T) -> std::io::Result<()>
        where T: Send + Sync + Worker + 'static {
        let service = await!(super::new(config.router_address.parse().unwrap(), worker))?;

        await!(service);

        Ok(())
    }

    pub fn run<T>(config: super::Config, mut worker: T) -> std::io::Result<()>
        where T: Worker + Send + Sync + 'static
    {
        use futures::compat::TokioDefaultSpawner;
        use futures::future::{TryFutureExt, FutureExt};

        // Spin up a service for this worker
        tarpc::init(TokioDefaultSpawner);

        println!("Initializing on router: {}", config.router_address);
        tokio::run(
            internal_run(config, worker)
                .map_err(|e| {
                    eprintln!("Oh no: {}", e);
                })
                .boxed()
                .compat(),
        );
        Ok(())
    }

    pub fn load_config() -> Result<super::Config, config::ConfigError> {
        let mut config = config::Config::new();

        config.merge(config::File::with_name("WorkerConfig.toml"));
        config.merge(config::Environment::with_prefix("DIST"))?;
        config.merge(config::Environment::with_prefix("DIST_WORKER"))?;

        config.try_into()
    }

    async fn request_new_entity_id() -> std::io::Result<dist_data::Id> {
        Ok(dist_data::Id::new(1))
    }
}
