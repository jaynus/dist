#![feature(await_macro, async_await, futures_api, arbitrary_self_types)]

#![warn(clippy::all)]
#![forbid(overflowing_literals)]
#![deny(missing_copy_implementations)]
#![deny(missing_debug_implementations)]
#![deny(path_statements)]
#![deny(trivial_bounds)]
#![deny(type_alias_bounds)]
#![deny(unconditional_recursion)]
#![deny(unions_with_drop_fields)]
#![deny(while_true)]
//#![deny(unused)]
#![deny(bad_style)]
#![deny(future_incompatible)]
#![deny(rust_2018_compatibility)]
#![deny(rust_2018_idioms)]
#![allow(unused_unsafe)]
#![allow(unused)]

use std::future::Future as NewFuture;
use futures::Future as OldFuture;

// converts from a new style Future to an old style one:
fn backward<I,E>(f: impl NewFuture<Output=Result<I,E>>) -> impl OldFuture<Item=I, Error=E> {
    use tokio_async_await::compat::backward;
    backward::Compat::new(f)
}

// converts from an old style Future to a new style one:
fn forward<I,E>(f: impl OldFuture<Item=I, Error=E> + Unpin) -> impl NewFuture<Output=Result<I,E>> {
    use tokio_async_await::compat::forward::IntoAwaitable;
    f.into_awaitable()
}

use tokio::await;
use tokio::prelude::*;

use dist_router::worker::{WorkerService, WorkerResult};
use dist_data::{Entity, Id};

async fn hello_world() -> &'static str {
    "Hello World"
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct TestWorker {}
impl WorkerService for TestWorker {
    fn init(&mut self, ) -> WorkerResult<()> {
        let hello_world_result = async {
            let s = await!(hello_world());
            Ok::<_,()>(s)
        };

        let hello_world_old = backward(hello_world_result);

        tokio::spawn(hello_world_old.and_then(|_|{
            println!("Hello World");

            Ok(())
        }));

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

// Worker test case
fn main() {
    let mut worker = TestWorker{};

// use Delay from the tokio::timer module to sleep the task:
    async fn sleep(n: u64) {
        use tokio::timer::Delay;
        use std::time::{Duration, Instant};
        await!(Delay::new(Instant::now() + Duration::from_secs(n))).unwrap();
    };

// sleep a second before each line is printed:
    tokio::run_async(async move {
        await!(sleep(1));
        println!("One");

        worker.init();

        await!(sleep(1));
        println!("Two");
        await!(sleep(1));
        println!("Three");
    });

}