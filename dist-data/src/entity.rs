use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Id {
    pub raw: u64,
}
impl Id {
    #[inline]
    pub fn new(raw: u64) -> Self {
        Self { raw }
    }
}
impl From<u64> for Id {
    #[inline]
    fn from(raw: u64) -> Self {
        Id { raw }
    }
}
impl From<Id> for u64 {
    #[inline]
    fn from(id: Id) -> Self {
        id.raw
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ComponentRef {
    id:        Id,
    parent:    Id,
    type_id:   Id,
}
impl ComponentRef {
    pub fn new(id: Id, parent: Id, type_id: Id) -> Self {
        Self {
            id,
            parent,
            type_id
        }
    }
}

#[derive(Debug, Clone, Default,  Serialize, Deserialize)]
pub struct EntityRef {
    id: Id,
    components: Vec<ComponentRef>,
}
impl PartialEq<EntityRef> for EntityRef {
    #[inline]
    fn eq(&self, rhv: &EntityRef) -> bool {
        self.id == rhv.id
    }
}

impl EntityRef {
    pub fn new(id: Id) -> Self {
        Self {
            id,
            components: Vec::new(),
        }
    }
    pub fn with_components(id: Id, components: Vec<ComponentRef>) -> Self {
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
    pub fn components(&self) -> &[ComponentRef] {
        self.components.as_slice()
    }

    #[inline]
    pub fn components_mut(&mut self) -> &mut [ComponentRef] {
        self.components.as_mut_slice()
    }

    #[inline]
    pub fn components_mut_vec(&mut self) -> &mut Vec<ComponentRef> {
        &mut self.components
    }
}