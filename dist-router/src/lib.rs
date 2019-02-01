#![feature(await_macro, async_await, futures_api, arbitrary_self_types)]
#![feature(vec_remove_item)]

pub mod router;
pub mod worker;

use dist_data::proto as dist_capnp;
pub mod router_capnp {
    #![allow(unused)]
    include!("../../capnp/router_capnp.rs");
}
pub use crate::router_capnp as proto;