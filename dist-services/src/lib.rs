#![feature(futures_api, arbitrary_self_types, await_macro, async_await, proc_macro_hygiene)]
#![feature(integer_atomics)]

#![warn(rust_2018_idioms, trivial_casts, trivial_numeric_casts)]
#![warn(clippy::pedantic, clippy::perf)]
#![allow(clippy::doc_markdown)]

#[macro_use] pub mod macros;

pub mod router;
pub mod worker;

use failure::{Fail};
use serde::{Serialize, Deserialize};

#[derive(Clone, Fail, Debug, Serialize, Deserialize)]
#[fail(display = "my wrapping error")]
pub enum StatusError {
    #[fail(display = "my error")]
    Died,
}
pub type Status = Result<(), StatusError>;

pub trait Service {
    fn address(&self) -> std::net::SocketAddr;
}

mod utils {
    //TODO: move this better place
    use futures::{Future, Poll, future::FutureObj, FutureExt};
    use std::pin::Pin;
    use pin_utils::pin_mut;
    use futures::task::LocalWaker;
    use std::time::{Duration, Instant};

    struct IntervalFuture {
        interval: Duration,
        next: Instant,
        last: Instant,
        fut_obj: FutureObj<'static, ()>,
    }

    impl IntervalFuture {
        pub fn new<F>(interval: Duration, future: F, ) -> Self
            where F: Future<Output=()> + Send + 'static
        {
            let now = Instant::now();
            Self {
                interval,
                last: now,
                next: now + interval,
                fut_obj: FutureObj::new(future.boxed()),
            }
        }
    }

    impl Future for IntervalFuture {
        type Output = ();

        fn poll(self: Pin<&mut Self>, lw: &LocalWaker) -> Poll<()> {
            let now = Instant::now();
            let mut ret = Poll::Pending;

            if self.last >= self.next {
                let s_mut = self.get_mut();
                s_mut.last = now;
                s_mut.next = now + s_mut.interval;

                let future = &mut s_mut.fut_obj;
                pin_mut!(future);
                ret = future.poll(lw)
            }

            lw.wake();
            ret
        }
    }
}

