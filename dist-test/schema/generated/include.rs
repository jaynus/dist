pub mod components {
use std::mem;
use std::cmp::Ordering;
use flatbuffers::EndianScalar;

include!("../../../schema/base_component_generated.rs");

include!("second_component_generated.rs");
include!("test_component_generated.rs");

pub enum Types<'a> { 
    SecondComponent(SecondComponent<'a>),
    TestComponent(TestComponent<'a>),
    Unknown,
}
pub enum TypeIds { 
    SecondComponent = 0,
    TestComponent = 1,
    Unknown,
}

#[inline(always)]
pub fn parse_type<'a>(id: u64, buf: &'a [u8]) -> Types<'a> { 
    match id {
        0 => Types::SecondComponent(get_root_as_second_component(buf)),
        1 => Types::TestComponent(get_root_as_test_component(buf)),
        _ => Types::Unknown,
    }
}

#[inline(always)]
pub fn parse_base_component<'a>(buf: &'a [u8]) -> Types<'a> { 
    let base = dist::get_root_as_base_component(buf);
    parse_type(base.id(), buf)
}


}