// automatically generated by the FlatBuffers compiler, do not modify



pub mod dist {

pub enum BaseComponentOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct BaseComponent<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for BaseComponent<'a> {
    type Inner = BaseComponent<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> BaseComponent<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        BaseComponent {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args BaseComponentArgs) -> flatbuffers::WIPOffset<BaseComponent<'bldr>> {
      let mut builder = BaseComponentBuilder::new(_fbb);
      builder.add_id(args.id);
      builder.finish()
    }

    pub const VT_ID: flatbuffers::VOffsetT = 4;

  #[inline]
  pub fn id(&self) -> u64 {
    self._tab.get::<u64>(BaseComponent::VT_ID, Some(0)).unwrap()
  }
}

pub struct BaseComponentArgs {
    pub id: u64,
}
impl<'a> Default for BaseComponentArgs {
    #[inline]
    fn default() -> Self {
        BaseComponentArgs {
            id: 0,
        }
    }
}
pub struct BaseComponentBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> BaseComponentBuilder<'a, 'b> {
  #[inline]
  pub fn add_id(&mut self, id: u64) {
    self.fbb_.push_slot::<u64>(BaseComponent::VT_ID, id, 0);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> BaseComponentBuilder<'a, 'b> {
    let start = _fbb.start_table();
    BaseComponentBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<BaseComponent<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

#[inline]
pub fn get_root_as_base_component<'a>(buf: &'a [u8]) -> BaseComponent<'a> {
  flatbuffers::get_root::<BaseComponent<'a>>(buf)
}

#[inline]
pub fn get_size_prefixed_root_as_base_component<'a>(buf: &'a [u8]) -> BaseComponent<'a> {
  flatbuffers::get_size_prefixed_root::<BaseComponent<'a>>(buf)
}

pub const BASE_COMPONENT_EXTENSION: &'static str = "bfbs";

#[inline]
pub fn finish_base_component_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<BaseComponent<'a>>) {
  fbb.finish(root, None);
}

#[inline]
pub fn finish_size_prefixed_base_component_buffer<'a, 'b>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>, root: flatbuffers::WIPOffset<BaseComponent<'a>>) {
  fbb.finish_size_prefixed(root, None);
}
}  // pub mod dist
