use std::{
    fmt::Display,
    marker::PhantomData,
    ops::{Index, IndexMut},
};

use crate::{Block, BlockId};

const fn calc_height(size: usize, width: usize) -> usize {
    size / width / width
}

#[derive(Debug, Clone, Copy)]
pub struct Chunk<Id: BlockId, const SIZE: usize, const WIDTH: usize>(pub [Block<Id>; SIZE]);

impl<Id: BlockId, const SIZE: usize, const WIDTH: usize> Default for Chunk<Id, SIZE, WIDTH> {
    fn default() -> Self {
        Self([Default::default(); SIZE])
    }
}

impl<Id: BlockId, const SIZE: usize, const WIDTH: usize> Display for Chunk<Id, SIZE, WIDTH> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Chunk<{}, {}, {}>",
            std::any::type_name::<Id>(),
            SIZE,
            WIDTH
        )?;
        for (pos, block) in self {
            writeln!(f, "{}: {}", pos, block)?;
        }
        Ok(())
    }
}

impl<Id: BlockId, const SIZE: usize, const WIDTH: usize> Chunk<Id, SIZE, WIDTH> {
    pub const HEIGHT: usize = calc_height(SIZE, WIDTH);
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockSubPos<const SIZE: usize, const WIDTH: usize>(usize);

#[derive(PartialEq, Eq)]
pub struct BlockSubPosIterator<const SIZE: usize, const WIDTH: usize> {
    phat: PhantomData<BlockSubPos<SIZE, WIDTH>>,
    current: std::ops::Range<usize>,
}

impl<const SIZE: usize, const WIDTH: usize> Default for BlockSubPosIterator<SIZE, WIDTH> {
    fn default() -> Self {
        Self {
            phat: Default::default(),
            current: 0..SIZE,
        }
    }
}

impl<const SIZE: usize, const WIDTH: usize> Iterator for BlockSubPosIterator<SIZE, WIDTH> {
    type Item = BlockSubPos<SIZE, WIDTH>;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.next().map(BlockSubPos::from_index)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.current.size_hint()
    }
}

impl<const SIZE: usize, const WIDTH: usize> Display for BlockSubPos<SIZE, WIDTH> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (x, y, z) = (*self).into();
        write!(f, "({}, {}, {})", x, y, z)
    }
}

impl<const SIZE: usize, const WIDTH: usize> Into<(u16, u16, u16)> for BlockSubPos<SIZE, WIDTH> {
    fn into(self) -> (u16, u16, u16) {
        let input = self.0;
        let x = (input % WIDTH) as u16;
        let z = ((input / WIDTH) % WIDTH) as u16;
        let y = (input / WIDTH / WIDTH) as u16;
        (x, y, z)
    }
}

impl<const SIZE: usize, const WIDTH: usize> BlockSubPos<SIZE, WIDTH> {
    pub const HEIGHT: usize = SIZE / WIDTH / WIDTH;

    pub fn new(x: u16, y: u16, z: u16) -> Self {
        assert!((x as usize) < WIDTH);
        assert!((y as usize) < SIZE / WIDTH / WIDTH);
        assert!((z as usize) < WIDTH);
        Self((x as usize) + (z as usize + (y as usize) * WIDTH) * WIDTH)
    }

    pub fn from_index(input: usize) -> Self {
        assert!(input < SIZE);
        Self(input)
    }

    pub fn as_index(self) -> usize {
        self.0
    }
}

impl<Id: BlockId, const SIZE: usize, const WIDTH: usize> Index<BlockSubPos<SIZE, WIDTH>>
    for Chunk<Id, SIZE, WIDTH>
{
    type Output = Block<Id>;

    fn index(&self, index: BlockSubPos<SIZE, WIDTH>) -> &Self::Output {
        &self.0[index.as_index()]
    }
}

impl<Id: BlockId, const SIZE: usize, const WIDTH: usize> IndexMut<BlockSubPos<SIZE, WIDTH>>
    for Chunk<Id, SIZE, WIDTH>
{
    fn index_mut(&mut self, index: BlockSubPos<SIZE, WIDTH>) -> &mut Self::Output {
        &mut self.0[index.as_index()]
    }
}

impl<'chunk, Id: BlockId, const SIZE: usize, const WIDTH: usize> IntoIterator
    for &'chunk Chunk<Id, SIZE, WIDTH>
{
    type Item = (BlockSubPos<SIZE, WIDTH>, &'chunk Block<Id>);

    type IntoIter =
        std::iter::Zip<BlockSubPosIterator<SIZE, WIDTH>, std::slice::Iter<'chunk, Block<Id>>>;

    fn into_iter(self) -> Self::IntoIter {
        BlockSubPosIterator::default().zip(self.0.iter())
    }
}

impl<'chunk, Id: BlockId, const SIZE: usize, const WIDTH: usize> IntoIterator
    for &'chunk mut Chunk<Id, SIZE, WIDTH>
{
    type Item = (BlockSubPos<SIZE, WIDTH>, &'chunk mut Block<Id>);

    type IntoIter =
        std::iter::Zip<BlockSubPosIterator<SIZE, WIDTH>, std::slice::IterMut<'chunk, Block<Id>>>;

    fn into_iter(self) -> Self::IntoIter {
        BlockSubPosIterator::default().zip(self.0.iter_mut())
    }
}

impl<Id: BlockId, const SIZE: usize, const WIDTH: usize> Chunk<Id, SIZE, WIDTH> {
    pub fn iter_solid(&self) -> impl Iterator<Item = (BlockSubPos<SIZE, WIDTH>, Id)> + '_ {
        self.into_iter().filter_map(|(pos, blk)| {
            if let &Block::Solid { id } = blk {
                Some((pos, id))
            } else {
                None
            }
        })
    }
}
