use std::sync::Arc;
use std::rc::Rc;
use std::cell::RefCell;
use std::time;
use capnp::capability::Promise;
use futures::future::Future;
use log::{trace};
use dist_data::RpcResult;
use crate::proto::router_service;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Interval {
    interval: time::Duration,
    last: Rc<RefCell<time::Instant>>,
}
impl Interval {
    pub fn new(interval: time::Duration,) -> Self {
        Self {
            interval,
            last: Rc::new(RefCell::new(time::Instant::now())),
        }
    }

    #[inline]
    pub fn tick(&self) -> bool {
        let now = time::Instant::now();
        let last = *self.last.borrow();
        match now.duration_since(last) >= self.interval {
            true => {
                *self.last.borrow_mut() = now;
                true
            },
            false => {
                false
            }
        }
    }
}

#[derive(Clone)]
struct WorkerHandle {
    pub inner: Arc<crate::proto::worker_service::Client>,
}
impl WorkerHandle {
    pub fn new(worker: crate::proto::worker_service::Client) -> Self {
        Self { inner: Arc::new(worker), }
    }
}
impl PartialEq for WorkerHandle {
    fn eq(&self, other: &WorkerHandle) -> bool {
        Arc::ptr_eq(&self.inner, &other.inner)
    }
}

pub struct RouterServiceImpl {
    inner: Rc<RefCell<RouterService>>,
}
impl RouterServiceImpl {
    pub fn new(inner: Rc<RefCell<RouterService>>,) -> Self {
        Self { inner }
    }
}

impl router_service::Server for RouterServiceImpl {
    fn register_worker(&mut self, request: router_service::RegisterWorkerParams<>, _: router_service::RegisterWorkerResults<>) -> RpcResult {
        trace!("register_worker");

        self.inner.borrow_mut().worker_interfaces.push(WorkerHandle::new(request.get()?.get_worker()?));

        Promise::ok(())
    }
}

pub struct RouterService {
    worker_interfaces: Vec<WorkerHandle>,
    keepalive: Interval,
}

pub struct RouterServiceFuture {
    inner: Rc<RefCell<RouterService>>,
}
impl RouterServiceFuture {
    pub fn new(inner: Rc<RefCell<RouterService>>,) -> Self {
        Self { inner }
    }
}
impl futures::Future for RouterServiceFuture {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> ::futures::Poll<Self::Item, Self::Error> {
        self.inner.borrow_mut().tick();

        futures::task::current().notify();
        Ok(futures::Async::NotReady)
    }
}

impl RouterService {
    pub fn new() -> Self {
        Self {
            worker_interfaces: Vec::new(),
            keepalive: Interval::new(time::Duration::from_secs(1)),
        }
    }

    pub fn tick(&mut self) {
        if self.keepalive.tick() {
            trace!("KEEPALIVE");
            let workers = self.worker_interfaces.clone();

            workers.iter().for_each(|handle| {
                let status_request =  handle.inner.status_request();
                trace!("iter");

                //        status_request.send().promise.wait().unwrap();z
                let promise = status_request.send().promise.and_then(|response| {
                    // We got a valid response, handle our local status tracking
                    trace!("Got response?");
                    Ok(())
                }).map_err(|e| {
                    trace!("response error");
                    match e.kind {
                        Disconnected => {
                            // Drop the worker, he disconnected
                            trace!("Got disconnected");
                            self.worker_interfaces.remove_item(handle);
                        },
                        _ => {
                            // Any other failure cases we report and ignore.
                        }
                    }
                });
                promise.wait();

            });
        }

    }
}

mod tests {
    use super::*;
    use crate::worker::*;
    use log::*;
    use dist_data::{Entity, Id};
    use crate::proto;

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
    impl proto::worker_service::Server for TestWorker {
        fn status(&mut self, _: proto::worker_service::StatusParams<>, mut result: proto::worker_service::StatusResults<>) -> ::capnp::capability::Promise<(), ::capnp::Error> {
            trace!("Status");

            let status =result.get().init_status();

            Promise::ok(())
        }
        fn shutdown(&mut self, _: proto::worker_service::ShutdownParams<>, _: proto::worker_service::ShutdownResults<>) -> ::capnp::capability::Promise<(), ::capnp::Error> { ::capnp::capability::Promise::err(::capnp::Error::unimplemented("method not implemented".to_string())) }
        fn set_region(&mut self, _: proto::worker_service::SetRegionParams<>, _: proto::worker_service::SetRegionResults<>) -> ::capnp::capability::Promise<(), ::capnp::Error> { ::capnp::capability::Promise::err(::capnp::Error::unimplemented("method not implemented".to_string())) }
        fn get_region(&mut self, _: proto::worker_service::GetRegionParams<>, _: proto::worker_service::GetRegionResults<>) -> ::capnp::capability::Promise<(), ::capnp::Error> { ::capnp::capability::Promise::err(::capnp::Error::unimplemented("method not implemented".to_string())) }
        fn give_entities(&mut self, _: proto::worker_service::GiveEntitiesParams<>, _: proto::worker_service::GiveEntitiesResults<>) -> ::capnp::capability::Promise<(), ::capnp::Error> { ::capnp::capability::Promise::err(::capnp::Error::unimplemented("method not implemented".to_string())) }
        fn take_entities(&mut self, _: proto::worker_service::TakeEntitiesParams<>, _: proto::worker_service::TakeEntitiesResults<>) -> ::capnp::capability::Promise<(), ::capnp::Error> { ::capnp::capability::Promise::err(::capnp::Error::unimplemented("method not implemented".to_string())) }
    }


    #[test]
    fn test_router() {
        env_logger::init();

        trace!("Entering test");
        use tokio::runtime::current_thread::Runtime;
        use tokio::prelude::*;

        let mut runtime = Runtime::new().unwrap();

        let router = Rc::new(RefCell::new(RouterService::new()));
        let worker = TestWorker{};

        let router_client = proto::router_service::ToClient::new(RouterServiceImpl::new(router.clone())).into_client::<::capnp_rpc::Server>();
        let worker_client = proto::worker_service::ToClient::new(worker).into_client::<::capnp_rpc::Server>();

        let mut request = router_client.register_worker_request();
        request.get().set_worker(worker_client.clone());
        let response = request.send().promise.wait().unwrap();

        runtime.block_on(RouterServiceFuture::new(router.clone()));

    }
}