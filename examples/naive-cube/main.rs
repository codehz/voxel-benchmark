use glam::f32 as math;
use std::marker::PhantomData;

use anyhow::Result;
use enum_map::EnumMap;
use glium::{
    backend::Facade, buffer::WriteMapping, implement_vertex, index::PrimitiveType, uniform,
    BackfaceCullingMode, Depth, DrawParameters, Frame, IndexBuffer, Program, Surface, VertexBuffer,
};
use strum::IntoEnumIterator;
use voxel_benchmark::*;

#[derive(Copy, Clone)]
struct PosTex {
    position: [f32; 3],
    tex_info: u32,
}

implement_vertex!(PosTex, position, tex_info);

fn gen_cube_mesh<
    Id: BlockId,
    const SIZE: usize,
    const WIDTH: usize,
    const CHUNK_SIZE: usize,
    const CHUNK_WIDTH: usize,
>(
    vertex: &mut WriteMapping<[PosTex]>,
    index: &mut WriteMapping<[u32]>,
    chunk_pos: ChunkPos<SIZE, WIDTH>,
    block_pos: BlockSubPos<CHUNK_SIZE, CHUNK_WIDTH>,
    definitions: &EnumMap<Id, SolidBlockDefinition>,
    id: Id,
    current: u32,
    face: BlockFace,
) {
    let vertex_base = current * 4;
    let index_base = current * 6;
    &[0, 1, 2, 0, 2, 3]
        .iter()
        .map(|x| *x + vertex_base)
        .zip((0..6usize).into_iter().map(|x| x + index_base as usize))
        .for_each(|(value, i)| index.set(i, value));
    let origin = WorldPosition::from((chunk_pos, block_pos));
    match face {
        BlockFace::North => [origin.ix().iy(), origin.ix(), origin, origin.iy()],
        BlockFace::South => [
            origin.iz().iy(),
            origin.iz(),
            origin.iz().ix(),
            origin.iz().ix().iy(),
        ],
        BlockFace::East => [
            origin.ix().iy().iz(),
            origin.ix().iz(),
            origin.ix(),
            origin.ix().iy(),
        ],
        BlockFace::West => [origin.iy(), origin, origin.iz(), origin.iz().iy()],
        BlockFace::Up => [
            origin.iy(),
            origin.iy().iz(),
            origin.iy().iz().ix(),
            origin.iy().ix(),
        ],
        BlockFace::Down => [origin.iz(), origin, origin.ix(), origin.ix().iz()],
    }
    .iter()
    .zip(&definitions[id][face].into_arr())
    .map(|(&position, &uv)| PosTex {
        position: position.into(),
        tex_info: uv.into(),
    })
    .zip((0..4usize).into_iter().map(|x| x + vertex_base as usize))
    .for_each(|(value, i)| vertex.set(i, value));
}

struct BufferGroup {
    vertex: VertexBuffer<PosTex>,
    index: IndexBuffer<u32>,
    count: u32,
}

impl BufferGroup {
    fn new<F: Facade, const CHUNK_SIZE: usize>(facade: &F) -> Result<Self> {
        let vertex = VertexBuffer::empty_dynamic(facade, CHUNK_SIZE * 4 * 6)?;
        let index =
            IndexBuffer::empty_dynamic(facade, PrimitiveType::TrianglesList, CHUNK_SIZE * 6 * 6)?;
        Ok(Self {
            vertex,
            index,
            count: 0,
        })
    }
}

struct BasicRenderer<
    Id: BlockId,
    const SIZE: usize,
    const WIDTH: usize,
    const CHUNK_SIZE: usize,
    const CHUNK_WIDTH: usize,
> {
    phat: PhantomData<WorldInfo<Id, SIZE, WIDTH, CHUNK_SIZE, CHUNK_WIDTH>>,
    program: Program,
    buffers: Vec<BufferGroup>,
}

impl<
        Id: BlockId,
        const SIZE: usize,
        const WIDTH: usize,
        const CHUNK_SIZE: usize,
        const CHUNK_WIDTH: usize,
    > BasicRenderer<Id, SIZE, WIDTH, CHUNK_SIZE, CHUNK_WIDTH>
{
    fn new<F: Facade>(facade: &F) -> Self {
        let mut buffers = Vec::with_capacity(SIZE);
        for _ in 0..SIZE {
            buffers.push(BufferGroup::new::<F, CHUNK_SIZE>(facade).unwrap());
        }
        Self {
            phat: Default::default(),
            program: shader_program!(facade, "shader").unwrap(),
            buffers,
        }
    }
}

struct Basic;

impl<
        Id: BlockId,
        const SIZE: usize,
        const WIDTH: usize,
        const CHUNK_SIZE: usize,
        const CHUNK_WIDTH: usize,
    > Renderer<Id, SIZE, WIDTH, CHUNK_SIZE, CHUNK_WIDTH>
    for BasicRenderer<Id, SIZE, WIDTH, CHUNK_SIZE, CHUNK_WIDTH>
{
    fn prepare(&mut self, info: &WorldInfo<Id, SIZE, WIDTH, CHUNK_SIZE, CHUNK_WIDTH>) {
        for ((chunk_pos, chunk), group) in info.world.as_ref().into_iter().zip(&mut self.buffers) {
            let mut writevertex = group.vertex.map_write();
            let mut writeindex = group.index.map_write();
            group.count = 0;
            for (block_pos, id) in chunk.iter_solid() {
                for face in BlockFace::iter() {
                    gen_cube_mesh(
                        &mut writevertex,
                        &mut writeindex,
                        chunk_pos,
                        block_pos,
                        info.definitions,
                        id,
                        group.count,
                        face,
                    );
                    group.count += 1;
                }
            }
        }
    }

    fn render(&self, mut frame: Frame, info: &WorldInfo<Id, SIZE, WIDTH, CHUNK_SIZE, CHUNK_WIDTH>) {
        frame.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
        let aspect_ratio = {
            let dim = frame.get_dimensions();
            dim.0 as f32 / dim.1 as f32
        };
        let perspective =
            math::Mat4::perspective_rh_gl(f32::to_radians(90.0), aspect_ratio, 0.1, 1024.0);
        let view_model = info.camera.get_matrix();
        let sampled = info
            .texture
            .sampled()
            .minify_filter(glium::uniforms::MinifySamplerFilter::LinearMipmapLinear)
            .magnify_filter(glium::uniforms::MagnifySamplerFilter::Linear)
            .wrap_function(glium::uniforms::SamplerWrapFunction::Clamp);
        let uniforms = uniform! {
            tile: sampled,
            perspective : perspective.to_cols_array_2d(),
            view_model: view_model,
        };
        for group in &self.buffers {
            let count = group.count as usize;
            if count == 0 {
                continue;
            }
            frame
                .draw(
                    group.vertex.slice(0..count * 4).unwrap(),
                    group.index.slice(0..count * 6).unwrap(),
                    &self.program,
                    &uniforms,
                    &DrawParameters {
                        depth: Depth {
                            test: glium::DepthTest::IfLess,
                            write: true,
                            ..Default::default()
                        },
                        backface_culling: BackfaceCullingMode::CullClockwise,
                        smooth: Some(glium::Smooth::Nicest),
                        ..Default::default()
                    },
                )
                .unwrap();
        }
        frame.finish().unwrap();
    }
}

impl RendererProvider for Basic {
    fn get_renderer<
        F: Facade,
        Id: BlockId,
        const SIZE: usize,
        const WIDTH: usize,
        const CHUNK_SIZE: usize,
        const CHUNK_WIDTH: usize,
    >(
        facade: &F,
        _info: &WorldInfo<Id, SIZE, WIDTH, CHUNK_SIZE, CHUNK_WIDTH>,
    ) -> Result<Box<dyn Renderer<Id, SIZE, WIDTH, CHUNK_SIZE, CHUNK_WIDTH>>> {
        Ok(Box::new(BasicRenderer::new(facade)))
    }
}

pub fn main() -> Result<()> {
    env_logger::Builder::from_default_env().init();
    run_renderer::<Basic>()
}
