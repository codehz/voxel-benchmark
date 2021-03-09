use crate::{BlockId, World};

pub mod flat;
pub mod random;

pub trait WorldGenerator<
    Id: BlockId,
    const SIZE: usize,
    const WIDTH: usize,
    const CHUNK_SIZE: usize,
    const CHUNK_WIDTH: usize,
>
{
    fn generate(&self, world: &mut World<Id, SIZE, WIDTH, CHUNK_SIZE, CHUNK_WIDTH>);
}
