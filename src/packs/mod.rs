use enum_map::EnumMap;

use crate::{BlockId, SolidBlockDefinition};

pub mod basic;

#[macro_use]
pub mod utils;

pub trait Pack {
    type Id: BlockId;

    fn get_textures() -> Vec<glium::texture::RawImage2d<'static, u8>>;
    fn get_map() -> &'static EnumMap<Self::Id, SolidBlockDefinition>;
}

pub trait SimpleBlockId: BlockId {
    fn get_simple_block() -> Self;

    fn get_simple_top_block() -> Self;

    fn get_random_block() -> Self;
}
