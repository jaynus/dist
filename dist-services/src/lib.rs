#![feature(futures_api, arbitrary_self_types, await_macro, async_await, proc_macro_hygiene)]

#[macro_use]
pub mod macros;
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