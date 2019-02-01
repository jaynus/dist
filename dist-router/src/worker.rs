use std::sync::Arc;
use dist_data::{Entity, Id};
use futures::future::{Future};
use failure::Fail;
use derive_more::From;

#[derive(Debug, Fail, From)]
pub enum WorkerError {
    #[fail(display = "{}", _0)]
    ErrorMessage(String),

    #[fail(display = "{}", _0)]
    IoError(std::io::Error),

    #[fail(display = "{}", _0)]
    CapnpError(capnp::Error),
}

pub type WorkerResult<T> = Result<T, WorkerError>;

pub trait WorkerService : Send {
    fn init(&mut self, ) -> WorkerResult<()>;

    fn new_entities(&mut self, entity: Entity, ) -> WorkerResult<()>;

    fn release_entities(&mut self, entity: Entity, ) -> WorkerResult<()>;

    fn shutdown(&mut self) -> WorkerResult<()>;
}

struct ServiceContainer {
    inner: Arc<dyn WorkerService>,
    interface: Arc<dyn crate::proto::worker_service::Server>
}

impl futures::Future for ServiceContainer {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> ::futures::Poll<Self::Item, Self::Error> {
        self.tick();

        futures::task::current().notify();
        Ok(futures::Async::NotReady)
    }
}

impl ServiceContainer {
    fn tick(&mut self, ) {

    }
}


mod tests {
    use super::*;
    use log::*;

    pub struct TestWorker {}
    impl WorkerService for TestWorker {
        fn init(&mut self, ) -> WorkerResult<()> {
            Ok(())
        }

        fn new_entities(&mut self, entity: Entity, ) -> WorkerResult<()> {
            Ok(())
        }

        fn release_entities(&mut self, entity: Entity, ) -> WorkerResult<()> {
            Ok(())
        }

        fn shutdown(&mut self) -> WorkerResult<()> {
            Ok(())
        }
    }

    #[test]
    fn impl_worker() {
        let test = TestWorker{};


    }
}
