#![allow(unused)]

#![feature(test)]
extern crate test;

use derive_more::From;
use failure::Fail;

use redis;
use log::trace;

use capnp::capability::Promise;

pub mod dist_capnp {
    #![allow(unused)]
    include!("../../capnp/dist_capnp.rs");
}
pub use crate::dist_capnp as proto;

mod entity;
pub mod macro_traits;

pub use crate::{
    entity::{Entity, Id}
};

pub type RpcResult = Promise<(), capnp::Error>;


#[derive(Debug, Fail, From)]
pub enum AdapterError {
    #[fail(display = "{}", _0)]
    ErrorMessage(String),

    #[fail(display = "{}", _0)]
    RedisError(redis::RedisError),

    #[fail(display = "{}", _0)]
    IoError(std::io::Error),

    #[fail(display = "{}", _0)]
    CapnpError(capnp::Error),
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
        use capnp::serialize_packed;
        use std::io::Cursor;

        trace!("Getting entity: {}", id);

        let data: Option<Vec<u8>> = self.connection.get(id)?;
        match  data {
            Some(data) => {
                let mut cursor = Cursor::new(&data);

                let message_reader = serialize_packed::read_message(&mut cursor,
                                                                    ::capnp::message::ReaderOptions::new())?;
                let entity_reader = message_reader.get_root::<proto::entity_data::Reader>()?;

                Ok(Some(
                    Entity::new(
                        entity_reader.get_id()?.get_id().into(),
                        entity_reader.get_components()?.iter().map(|id| { Id { id: id.get_id() } }).collect()
                    )
                ))
            },
            None => Ok(None),
        }

    }
    fn get_component(&self, id: u64) -> AdapterResult<Option<Vec<u8>>> {
        Ok(None)
    }

    fn commit_entity(&self, entity: &Entity) -> AdapterResult<()> {
        use redis::Commands;
        use capnp::serialize_packed;
        use std::io::Cursor;

        trace!("Commiting entity: {}", entity.id().id);

        // Build the capnp representation
        let mut message = ::capnp::message::Builder::new_default();
        let builder = message.init_root::<proto::entity_data::Builder>();

        let mut id_builder = builder.init_id();
        id_builder.set_id(entity.id().into());

        let mut data: Vec<u8> = Vec::new();

        {
            let mut cursor = Cursor::new(&mut data);
            serialize_packed::write_message(&mut cursor, &message)?;
        }

        self.connection.set(entity.id().id, data)?;

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
