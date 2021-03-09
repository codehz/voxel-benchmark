use crate::{packs::SimpleBlockId, *};

use super::WorldGenerator;

#[derive(Debug)]
pub struct Flat<Id: BlockId> {
    receipe: Vec<Option<Id>>,
}

impl<Id: BlockId> Flat<Id> {
    fn get_block(&self, level: u16) -> Block<Id> {
        if let Some(&Some(id)) = self.receipe.get(level as usize) {
            Block::Solid { id }
        } else {
            Block::Empty
        }
    }

    pub fn new(receipe: Vec<Option<Id>>) -> Self {
        Self { receipe }
    }
}

impl<Id: SimpleBlockId> Flat<Id> {
    pub fn new_simple(level: usize) -> Self {
        Self {
            receipe: std::iter::repeat(Some(Id::get_simple_block()))
                .take(level - 1)
                .chain(std::iter::once(Some(Id::get_simple_top_block())))
                .collect(),
        }
    }
}

impl<
        Id: BlockId,
        const SIZE: usize,
        const WIDTH: usize,
        const CHUNK_SIZE: usize,
        const CHUNK_WIDTH: usize,
    > WorldGenerator<Id, SIZE, WIDTH, CHUNK_SIZE, CHUNK_WIDTH> for Flat<Id>
{
    fn generate(&self, world: &mut World<Id, SIZE, WIDTH, CHUNK_SIZE, CHUNK_WIDTH>) {
        for (_, chunk) in world {
            for (pos, block) in chunk {
                let (_, level, _) = pos.into();
                *block = self.get_block(level);
            }
        }
    }
}
