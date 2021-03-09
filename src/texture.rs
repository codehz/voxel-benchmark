use std::ops::Index;

use enum_map::{enum_map, Enum, EnumMap};
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Enum, EnumIter)]
pub enum BlockFace {
    North,
    South,
    East,
    West,
    Up,
    Down,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextureIndex(pub u16);

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextureInfo {
    index: u16,
    order: u8,
}

impl TextureInfo {
    fn new(index: u16, order: u8) -> Self {
        TextureInfo { index, order }
    }
}

impl TextureIndex {
    pub fn into_arr(&self) -> [TextureInfo; 4] {
        [
            TextureInfo::new(self.0, 0b00),
            TextureInfo::new(self.0, 0b01),
            TextureInfo::new(self.0, 0b11),
            TextureInfo::new(self.0, 0b10),
        ]
    }
}

impl Into<u32> for TextureInfo {
    fn into(self) -> u32 {
        ((self.index as u32) << 16) + self.order as u32
    }
}

// Only solid block
#[derive(Debug, Clone, Copy)]
pub struct SolidBlockDefinition(pub EnumMap<BlockFace, TextureIndex>);

impl<'a> IntoIterator for &'a SolidBlockDefinition {
    type Item = (BlockFace, &'a TextureIndex);

    type IntoIter = enum_map::Iter<'a, BlockFace, TextureIndex>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl Index<BlockFace> for SolidBlockDefinition {
    type Output = TextureIndex;

    fn index(&self, index: BlockFace) -> &Self::Output {
        &self.0[index]
    }
}

impl<F: FnMut(BlockFace) -> TextureIndex> From<F> for SolidBlockDefinition {
    fn from(f: F) -> Self {
        Self(EnumMap::from(f))
    }
}

impl From<TextureIndex> for SolidBlockDefinition {
    fn from(uv: TextureIndex) -> Self {
        Self::from(|_| uv)
    }
}

impl SolidBlockDefinition {
    pub fn new_simple_block(uv: TextureIndex) -> Self {
        Self::from(|_| uv)
    }

    pub fn new_block(
        north: TextureIndex,
        south: TextureIndex,
        east: TextureIndex,
        west: TextureIndex,
        up: TextureIndex,
        down: TextureIndex,
    ) -> Self {
        Self(enum_map! {
            BlockFace::North => north,
            BlockFace::South => south,
            BlockFace::East => east,
            BlockFace::West => west,
            BlockFace::Up => up,
            BlockFace::Down => down,
        })
    }
}
