use std::{alloc::{alloc_zeroed, Layout}, fmt::Display, marker::PhantomData, ops::{Index, IndexMut}};

use crate::{BlockId, Chunk};

#[derive(Debug, Clone, Copy)]
pub struct World<
    Id: BlockId,
    const SIZE: usize,
    const WIDTH: usize,
    const CHUNK_SIZE: usize,
    const CHUNK_WIDTH: usize,
>(pub [Chunk<Id, CHUNK_SIZE, CHUNK_WIDTH>; SIZE]);

impl<
        Id: BlockId,
        const SIZE: usize,
        const WIDTH: usize,
        const CHUNK_SIZE: usize,
        const CHUNK_WIDTH: usize,
    > Display for World<Id, SIZE, WIDTH, CHUNK_SIZE, CHUNK_WIDTH>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "world<{}, {}, {}, {}, {}>",
            std::any::type_name::<Id>(),
            SIZE,
            WIDTH,
            CHUNK_SIZE,
            CHUNK_WIDTH
        )?;
        for (pos, chunk) in self {
            writeln!(f, "{}: {}", pos, chunk)?;
        }
        Ok(())
    }
}

impl<
        Id: BlockId,
        const SIZE: usize,
        const WIDTH: usize,
        const CHUNK_SIZE: usize,
        const CHUNK_WIDTH: usize,
    > World<Id, SIZE, WIDTH, CHUNK_SIZE, CHUNK_WIDTH>
{
    pub const LENGTH: usize = SIZE / WIDTH;

    pub fn create() -> Box<Self> {
        let layout = Layout::new::<World<Id, SIZE, WIDTH, CHUNK_SIZE, CHUNK_WIDTH>>();
        unsafe {
            let pointer = alloc_zeroed(layout);
            Box::from_raw(pointer as *mut World<Id, SIZE, WIDTH, CHUNK_SIZE, CHUNK_WIDTH>)
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChunkPos<const SIZE: usize, const WIDTH: usize>(usize);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkPosIterator<const SIZE: usize, const WIDTH: usize>{
    phat: PhantomData<ChunkPos<SIZE, WIDTH>>,
    current: std::ops::Range<usize>,
}

impl<const SIZE: usize, const WIDTH: usize> Default for ChunkPosIterator<SIZE, WIDTH> {
    fn default() -> Self {
        Self {
            phat: Default::default(),
            current: 0..SIZE,
        }
    }
}

impl<const SIZE: usize, const WIDTH: usize> Iterator for ChunkPosIterator<SIZE, WIDTH> {
    type Item = ChunkPos<SIZE, WIDTH>;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.next().map(ChunkPos::from_index)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.current.size_hint()
    }
}

impl<const SIZE: usize, const WIDTH: usize> Display for ChunkPos<SIZE, WIDTH> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (x, z) = (*self).into();
        write!(f, "({}, {})", x, z)
    }
}

impl<const SIZE: usize, const WIDTH: usize> Into<(u16, u16)> for ChunkPos<SIZE, WIDTH> {
    fn into(self) -> (u16, u16) {
        let input = self.0;
        let x = (input % WIDTH) as u16;
        let z = (input / WIDTH) as u16;
        (x, z)
    }
}

impl<const SIZE: usize, const WIDTH: usize> ChunkPos<SIZE, WIDTH> {
    pub fn new(x: u16, z: u16) -> Self {
        assert!((x as usize) < WIDTH);
        assert!((z as usize) < SIZE / WIDTH);
        Self((x as usize) + (z as usize) * WIDTH)
    }

    pub fn as_index(self) -> usize {
        self.0
    }

    pub fn from_index(input: usize) -> Self {
        assert!(input < SIZE);
        Self(input)
    }
}

impl<
        Id: BlockId,
        const SIZE: usize,
        const WIDTH: usize,
        const CHUNK_SIZE: usize,
        const CHUNK_WIDTH: usize,
    > Index<ChunkPos<SIZE, WIDTH>> for World<Id, SIZE, WIDTH, CHUNK_SIZE, CHUNK_WIDTH>
{
    type Output = Chunk<Id, CHUNK_SIZE, CHUNK_WIDTH>;

    fn index(&self, index: ChunkPos<SIZE, WIDTH>) -> &Self::Output {
        &self.0[index.as_index()]
    }
}

impl<
        Id: BlockId,
        const SIZE: usize,
        const WIDTH: usize,
        const CHUNK_SIZE: usize,
        const CHUNK_WIDTH: usize,
    > IndexMut<ChunkPos<SIZE, WIDTH>> for World<Id, SIZE, WIDTH, CHUNK_SIZE, CHUNK_WIDTH>
{
    fn index_mut(&mut self, index: ChunkPos<SIZE, WIDTH>) -> &mut Self::Output {
        &mut self.0[index.as_index()]
    }
}

impl<
        'world,
        Id: BlockId,
        const SIZE: usize,
        const WIDTH: usize,
        const CHUNK_SIZE: usize,
        const CHUNK_WIDTH: usize,
    > IntoIterator for &'world World<Id, SIZE, WIDTH, CHUNK_SIZE, CHUNK_WIDTH>
{
    type Item = (
        ChunkPos<SIZE, WIDTH>,
        &'world Chunk<Id, CHUNK_SIZE, CHUNK_WIDTH>,
    );

    type IntoIter = std::iter::Zip<
        ChunkPosIterator<SIZE, WIDTH>,
        std::slice::Iter<'world, Chunk<Id, CHUNK_SIZE, CHUNK_WIDTH>>,
    >;

    fn into_iter(self) -> Self::IntoIter {
        ChunkPosIterator::default().zip(self.0.iter())
    }
}

impl<
        'world,
        Id: BlockId,
        const SIZE: usize,
        const WIDTH: usize,
        const CHUNK_SIZE: usize,
        const CHUNK_WIDTH: usize,
    > IntoIterator for &'world mut World<Id, SIZE, WIDTH, CHUNK_SIZE, CHUNK_WIDTH>
{
    type Item = (
        ChunkPos<SIZE, WIDTH>,
        &'world mut Chunk<Id, CHUNK_SIZE, CHUNK_WIDTH>,
    );

    type IntoIter = std::iter::Zip<
        ChunkPosIterator<SIZE, WIDTH>,
        std::slice::IterMut<'world, Chunk<Id, CHUNK_SIZE, CHUNK_WIDTH>>,
    >;

    fn into_iter(self) -> Self::IntoIter {
        ChunkPosIterator::default().zip(self.0.iter_mut())
    }
}
