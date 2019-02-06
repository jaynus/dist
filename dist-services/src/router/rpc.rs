use futures::future::{Ready, ready, TryFutureExt, FutureExt};
use tarpc::context;
use serde::{Serialize, Deserialize};
use std::sync::{Arc, RwLock};
use super::State;
use crate::Status;

use log::error;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkerRegisterError;

tarpc::service! {
    /// Returns a greeting for name.
    rpc status() -> crate::Status;

    rpc register_worker(connection_string: String) -> Result<crate::worker::rpc::RemoteInfo, WorkerRegisterError>;
}
bootstrap_remote_client!();

#[derive(Clone)]
pub struct Server {
    state: Arc<RwLock<State>>,
}
impl Server {
    pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(State::new())),
        }
    }
}

impl Service for Server {
    type StatusFut = Ready<Status>;
    fn status(self, _: context::Context) -> Self::StatusFut {
        ready(Ok(()))
    }

    type RegisterWorkerFut = Ready<Result<crate::worker::rpc::RemoteInfo, WorkerRegisterError>>;
    fn register_worker(self, _: context::Context, connection_string: String) -> Self::RegisterWorkerFut {
        let id: u64;
        {
            let workers = &self.state.read().unwrap().workers;
            id = workers.len() as u64;
        }
        let info = crate::worker::rpc::RemoteInfo::new(id, &connection_string);
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
                    workers.push(remote);
                },
                Err(e) => {
                    error!("RemoteWorker status failed: {}, dropping and calling terminate", e);
                    await!(remote.client().terminate(context::current()))?;
                }
            }

            Ok(())
        }().map_err(|e| error!("Finalizing worker registration failed: {}", e))
            .boxed()
            .compat());

        ready(Ok(info))
    }
}