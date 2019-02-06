use futures::future::{Ready, ready};
use tarpc::context;
use serde::{Serialize, Deserialize};
use std::sync::{Arc, RwLock};
use super::State;
use crate::Status;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkerRegisterError;

tarpc::service! {
    /// Returns a greeting for name.
    rpc status() -> Status;

    rpc begin();
    rpc terminate();
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
    // Each defined rpc generates two items in the trait, a fn that serves the RPC, and
    // an associated type representing the future output by the fn.

    type StatusFut = Ready<Status>;
    fn status(self, _: context::Context) -> Self::StatusFut {
        ready(Ok(()))
    }

    type BeginFut = Ready<()>;
    fn begin(self, _: context::Context) -> Self::BeginFut {
        ready(())
    }

    type TerminateFut = Ready<()>;
    fn terminate(self, _: context::Context) -> Self::TerminateFut {
        ready(())
    }
}