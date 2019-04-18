#![allow(unused)]
#![feature(type_alias_enum_variants)]

#[macro_use]
extern crate serde_derive;

pub mod storage;

mod entity;
pub use crate::{
    entity::{EntityRef, ComponentRef, Id}
};
