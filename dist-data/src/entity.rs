use dist_macro::Dirty;

use crate::proto;
use capnp::capability::Promise;
use crate::RpcResult;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Id {
    pub id: u64,
}
impl Id {
    #[inline]
    pub fn new(id: u64) -> Self {
        Self { id }
    }
}
impl From<u64> for Id {
    #[inline]
    fn from(id: u64) -> Self {
        Id { id }
    }
}
impl From<Id> for u64 {
    #[inline]
    fn from(id: Id) -> Self {
        id.id
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Dirty)]
pub struct Entity {
    id: Id,
    components: Vec<Id>,
}
impl Entity {
    pub fn new(id: Id, components: Vec<Id>) -> Self {
        Self {
            id,
            components
        }
    }

    #[inline]
    pub fn id(&self) -> Id {
        self.id
    }

    #[inline]
    pub fn components(&self) -> &[Id] {
        self.components.as_slice()
    }

    #[inline]
    pub fn components_mut(&mut self) -> &mut [Id] {
        self.components.as_mut_slice()
    }

    #[inline]
    pub fn components_mut_vec(&mut self) -> &mut Vec<Id> {
        &mut self.components
    }
}


impl proto::entity::Server for Entity {
    fn get_id(&mut self, _: proto::entity::GetIdParams<>, mut result: proto::entity::GetIdResults<>) -> RpcResult {
        result.get().init_id().set_id(self.id().into());

        Promise::ok(())
    }
    fn get_components(&mut self, _: proto::entity::GetComponentsParams<>, mut result: proto::entity::GetComponentsResults<>) -> RpcResult {

        result.get().init_components(self.components().len() as u32);
        for (i, c) in self.components().iter().enumerate() {
            result.get().get_components().unwrap().get(i as u32).set_id(self.components()[i].into());
        }

        Promise::ok(())
    }
}

