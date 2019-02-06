#![allow(unused)]

#![feature(test)]
extern crate test;

use derive_more::From;
use failure::Fail;

use redis;
use log::trace;

mod entity;

pub use crate::{
    entity::{Entity, Id}
};

#[derive(Debug, Fail, From)]
pub enum AdapterError {
    #[fail(display = "{}", _0)]
    ErrorMessage(String),

    #[fail(display = "{}", _0)]
    RedisError(redis::RedisError),

    #[fail(display = "{}", _0)]
    IoError(std::io::Error),
}
type AdapterResult<T> = Result<T, AdapterError>;

trait Adapter {
    fn get_entity(&self, id: u64) -> AdapterResult<Option<Entity>>;
    fn get_component(&self, id: u64) -> AdapterResult<Option<Vec<u8>>>;

    fn commit_entity(&self, entity: &Entity) -> AdapterResult<()>;
    //fn commit_component(id: u64, component: T) -> AdapterResult<()>;
}

pub struct RedisAdapter {
    client: redis::Client,
    connection: redis::Connection,
}

impl RedisAdapter {
    pub fn new(conn_string: &str, ) -> Result<Self, redis::RedisError> {
        let client = redis::Client::open(conn_string)?;
        let connection = client.get_connection()?;

        Ok(Self {
            client,
            connection,
        })
    }
}

impl Adapter for RedisAdapter {
    fn get_entity(&self, id: u64) -> AdapterResult<Option<Entity>> {
        use redis::Commands;
        use std::io::Cursor;

        trace!("Getting entity: {}", id);

        Ok(None)

    }
    fn get_component(&self, id: u64) -> AdapterResult<Option<Vec<u8>>> {
        Ok(None)
    }

    fn commit_entity(&self, entity: &Entity) -> AdapterResult<()> {
        use redis::Commands;
        use std::io::Cursor;

        trace!("Commiting entity: {}", entity.id().id);


        Ok(())
    }
}

mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn redis_adapter_test() {
        let adapter = RedisAdapter::new("redis://172.17.0.2:7000/").unwrap();

        let test_entity = Entity::new(12345.into(), vec![8.into(), 9.into(), 10.into()]);

        adapter.commit_entity(&test_entity);

        let read_entity = adapter.get_entity(test_entity.id().into()).unwrap().unwrap();

        assert_eq!(read_entity.id(), test_entity.id());
    }

}
