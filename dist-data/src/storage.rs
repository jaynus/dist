use bincode::{serialize, deserialize};
use derive_more::From;
use failure::Fail;

use redis;
use log::trace;

use crate::{Id, EntityRef, ComponentRef};

#[derive(Debug, Fail, From)]
pub enum AdapterError {
    #[fail(display = "{}", _0)]
    ErrorMessage(String),

    #[fail(display = "{}", _0)]
    RedisError(redis::RedisError),

    #[fail(display = "{}", _0)]
    IoError(std::io::Error),

    #[fail(display = "{}", _0)]
    BincodeError(std::boxed::Box<bincode::ErrorKind>),
}


type AdapterResult<T> = Result<T, AdapterError>;

trait Adapter {
    fn commit<T>(&self, id: Id, obj: &T) -> AdapterResult<()> where T: serde::Serialize;
    //fn commit_component(id: u64, component: T) -> AdapterResult<()>;

    fn get<T>(&self, id: Id) -> AdapterResult<Option<T>> where T: serde::de::DeserializeOwned;
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
    fn get<T>(&self, id: Id) -> AdapterResult<Option<T>>
        where T: serde::de::DeserializeOwned
    {
        use redis::Commands;
        use std::io::Cursor;

        match self.connection.get::<Vec<u8>, Vec<u8>>(serialize(&id)?) {
            Ok(v) => {
                Ok(Some(deserialize(&v)?))
            },
            Err(e) => Err(AdapterError::RedisError(e))
        }
    }

    fn commit<T>(&self, id: Id, obj: &T) -> AdapterResult<()>
        where T: serde::Serialize
    {
        use redis::Commands;
        use std::io::Cursor;

        self.connection.set::<Vec<u8>, Vec<u8>, Vec<u8>>(serialize(&id)?, serialize(&obj)?)?;

        Ok(())
    }
}

mod tests {
    use super::*;

    #[test]
    fn redis_adapter_test() {
        let adapter = RedisAdapter::new("redis://172.17.0.2:7000/").unwrap();

        let test_entity = EntityRef::new(12345.into());

        adapter.commit(test_entity.id(), &test_entity);

        let read_entity: EntityRef = adapter.get(test_entity.id().into()).unwrap().unwrap();

        assert_eq!(read_entity.id(), test_entity.id());
    }

}
