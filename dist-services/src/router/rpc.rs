use futures::future::{Ready, ready, TryFutureExt, FutureExt};
use tarpc::context;
use serde::{Serialize, Deserialize};
use std::sync::{Arc, RwLock};
use super::State;
use crate::Status;

use log::{error, trace};

trait RpcError: core::fmt::Debug {

}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkerRegisterError;

impl From<WorkerRegisterError> for std::io::Error
{
    fn from(rhv: WorkerRegisterError) -> Self {
        rhv.into()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkerRegisterResponse {
    pub id: u64,
}

tarpc::service! {
    /// Returns a greeting for name.
    rpc status() -> crate::Status;

    rpc register_worker(address: std::net::SocketAddr) -> Result<WorkerRegisterResponse, WorkerRegisterError>;
}
bootstrap_remote_client!();

#[derive(Clone)]
pub struct Server {
    state: Arc<RwLock<State>>,
}
impl Server {
    pub fn new(state: Arc<RwLock<State>>) -> Self {
        Self {
            state,
        }
    }
}

impl Service for Server {
    type StatusFut = Ready<Status>;
    fn status(self, _: context::Context) -> Self::StatusFut {
        trace!("router_service::status()");
        ready(Ok(()))
    }

    type RegisterWorkerFut = Ready<Result<WorkerRegisterResponse, WorkerRegisterError>>;
    fn register_worker(self, _: context::Context, address: std::net::SocketAddr) -> Self::RegisterWorkerFut {
        trace!("router_service::register_worker()");
        let id: u64;
        {
            use std::sync::atomic::{Ordering};
            let id_increment = &self.state.read().unwrap().id_increment;
            let prev_id = id_increment.fetch_add(1, Ordering::SeqCst);
            id = prev_id + 1;
        }
        let info = crate::worker::rpc::RemoteInfo::new(&address);
        let info_ref = Arc::new(info.clone());
        let info_ref_2 = info_ref.clone();

        let state_clone = self.state.clone();

        tokio_executor::spawn(async move || -> std::io::Result<()> {
            let mut remote = await!(crate::worker::rpc::Remote::bootstrap(info_ref_2))?;

            await!(remote.client().begin(context::current()))?;

            // Confirm the server is up
            match await!(remote.client().status(context::current()))? {
                Ok(()) => {
                    let workers = &mut state_clone.write().unwrap().workers;
                    workers.push(super::WorkerInfo::new(id, remote));
                },
                Err(e) => {
                    error!("RemoteWorker status failed: {}, dropping and calling terminate", e);
                    await!(remote.client().terminate(context::current()))?;
                }
            }

            trace!("Worker registration complete");

            Ok(())
        }().map_err(|e| error!("Finalizing worker registration failed: {}", e))
            .boxed()
            .compat());

        ready(Ok(WorkerRegisterResponse {
            id,
        }))
    }
}