use futures::future::{Ready, ready};
use tarpc::context;
use serde::{Serialize, Deserialize};
use std::sync::{Arc, RwLock};
use super::State;
use crate::Status;

use log::trace;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkerError;

use dist_data::{ComponentRef};

tarpc::service! {
    rpc status() -> Status;

    rpc begin();
    rpc terminate();

   // rpc assign_entity() -> Result<()>;
   rpc assign_components(ids: Vec<ComponentRef>) -> Result<(), WorkerError>;
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
    // Each defined rpc generates two items in the trait, a fn that serves the RPC, and
    // an associated type representing the future output by the fn.

    type StatusFut = Ready<Status>;
    fn status(self, _: context::Context) -> Self::StatusFut {
        trace!("worker_service::status()");
        ready(Ok(()))
    }

    type BeginFut = Ready<()>;
    fn begin(self, _: context::Context) -> Self::BeginFut {
        trace!("worker_service::begin()");
        ready(())
    }

    type TerminateFut = Ready<()>;
    fn terminate(self, _: context::Context) -> Self::TerminateFut {
        trace!("worker_service::terminate()");
        ready(())
    }

    type AssignComponentsFut = Ready<Result<(), WorkerError>>;
    fn assign_components(self, _: context::Context, ids: Vec<ComponentRef>) -> Self::AssignComponentsFut {
        trace!("worker_service::assign_components()");

        self.state.write().unwrap().local_components.extend(ids);
        // TODO: trigger entity assignment event

        ready(Ok(()))
    }
}
