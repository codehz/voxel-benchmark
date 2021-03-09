use crate::{packs::SimpleBlockId, Block};

use super::WorldGenerator;

#[derive(Debug)]
pub enum RandomGenerator {
    Fill,
    Odd,
    FillRate(f32),
}

impl<
        Id: SimpleBlockId,
        const SIZE: usize,
        const WIDTH: usize,
        const CHUNK_SIZE: usize,
        const CHUNK_WIDTH: usize,
    > WorldGenerator<Id, SIZE, WIDTH, CHUNK_SIZE, CHUNK_WIDTH> for RandomGenerator
{
    fn generate(&self, world: &mut crate::World<Id, SIZE, WIDTH, CHUNK_SIZE, CHUNK_WIDTH>) {
        for (_, chunk) in world {
            for (pos, block) in chunk {
                *block = match self {
                    RandomGenerator::Fill => Block::Solid {
                        id: Id::get_random_block(),
                    },
                    RandomGenerator::Odd => {
                        let (x, y, z) = pos.into();
                        if (x ^ y ^ z) % 2 == 0 {
                            Block::Solid {
                                id: Id::get_random_block(),
                            }
                        } else {
                            Block::Empty
                        }
                    }
                    RandomGenerator::FillRate(rate) => {
                        if rand::random::<f32>() <= *rate {
                            Block::Solid {
                                id: Id::get_random_block(),
                            }
                        } else {
                            Block::Empty
                        }
                    }
                }
            }
        }
    }
}
