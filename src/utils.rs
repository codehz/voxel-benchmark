use crate::{BlockSubPos, ChunkPos};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct WorldPosition {
    pub x: u32,
    pub y: u16,
    pub z: u32,
}

impl Into<[f32; 3]> for WorldPosition {
    fn into(self) -> [f32; 3] {
        [self.x as f32, self.y as f32, self.z as f32]
    }
}

impl<const SIZE: usize, const WIDTH: usize, const CHUNK_SIZE: usize, const CHUNK_WIDTH: usize>
    From<(ChunkPos<SIZE, WIDTH>, BlockSubPos<CHUNK_SIZE, CHUNK_WIDTH>)> for WorldPosition
{
    fn from((chk, blk): (ChunkPos<SIZE, WIDTH>, BlockSubPos<CHUNK_SIZE, CHUNK_WIDTH>)) -> Self {
        let (chk_x, chk_z) = chk.into();
        let (blk_x, blk_y, blk_z) = blk.into();
        Self {
            x: (chk_x as u32) * (CHUNK_WIDTH as u32) + (blk_x as u32),
            y: blk_y,
            z: (chk_z as u32) * (CHUNK_WIDTH as u32) + (blk_z as u32),
        }
    }
}

impl WorldPosition {
    pub fn ix(self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
            z: self.z,
        }
    }
    pub fn iy(self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
            z: self.z,
        }
    }
    pub fn iz(self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            z: self.z + 1,
        }
    }
}

#[macro_export]
macro_rules! shader_program {
    ($display:expr, $shader:literal) => {
        glium::Program::from_source(
            $display,
            include_str!(concat!($shader, ".vert")),
            include_str!(concat!($shader, ".frag")),
            None,
        )
    };
    ($display:expr, $shader:literal with geometry) => {
        glium::Program::from_source(
            $display,
            include_str!(concat!($shader, ".vert")),
            include_str!(concat!($shader, ".frag")),
            Some(include_str!(concat!($shader, ".geom"))),
        )
    };
}
