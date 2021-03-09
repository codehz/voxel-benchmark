use anyhow::Result;
use enum_map::EnumMap;
use glam::f32 as math;
use glium::{
    backend::Facade, buffer::WriteMapping, implement_vertex, uniform, BackfaceCullingMode, Depth,
    DrawParameters, Program, Surface, VertexBuffer,
};
use std::marker::PhantomData;
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
    chunk_pos: ChunkPos<SIZE, WIDTH>,
    block_pos: BlockSubPos<CHUNK_SIZE, CHUNK_WIDTH>,
    definitions: &EnumMap<Id, SolidBlockDefinition>,
    id: Id,
    current: u32,
    face: BlockFace,
) {
    let faceid = face as u8;
    let origin = WorldPosition::from((chunk_pos, block_pos));
    let def = definitions[id][face];
    vertex.set(
        current as usize,
        PosTex {
            position: origin.into(),
            tex_info: ((def.0 as u32) << 16u32) + faceid as u32,
        },
    );
}
struct BufferGroup {
    vertex: VertexBuffer<PosTex>,
    count: u32,
}

impl BufferGroup {
    fn new<F: Facade, const CHUNK_SIZE: usize>(facade: &F) -> Result<Self> {
        let vertex = VertexBuffer::empty_dynamic(facade, CHUNK_SIZE * 6)?;
        Ok(Self { vertex, count: 0 })
    }
}

struct GeometryFaceRenderer<
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
    > GeometryFaceRenderer<Id, SIZE, WIDTH, CHUNK_SIZE, CHUNK_WIDTH>
{
    fn new<F: Facade>(facade: &F) -> Self {
        let mut buffers = Vec::with_capacity(SIZE);
        for _ in 0..SIZE {
            buffers.push(BufferGroup::new::<F, CHUNK_SIZE>(facade).unwrap());
        }
        Self {
            phat: Default::default(),
            program: shader_program!(facade, "shader" with geometry).unwrap(),
            buffers,
        }
    }
}

struct GeometryFace;

impl<
        Id: BlockId,
        const SIZE: usize,
        const WIDTH: usize,
        const CHUNK_SIZE: usize,
        const CHUNK_WIDTH: usize,
    > Renderer<Id, SIZE, WIDTH, CHUNK_SIZE, CHUNK_WIDTH>
    for GeometryFaceRenderer<Id, SIZE, WIDTH, CHUNK_SIZE, CHUNK_WIDTH>
{
    fn prepare(&mut self, info: &WorldInfo<Id, SIZE, WIDTH, CHUNK_SIZE, CHUNK_WIDTH>) {
        for ((chunk_pos, chunk), group) in info.world.as_ref().into_iter().zip(&mut self.buffers) {
            let mut writevertex = group.vertex.map_write();
            group.count = 0;
            for (block_pos, id) in chunk.iter_solid() {
                for face in BlockFace::iter() {
                    gen_cube_mesh(
                        &mut writevertex,
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

    fn render(
        &self,
        mut frame: glium::Frame,
        info: &WorldInfo<Id, SIZE, WIDTH, CHUNK_SIZE, CHUNK_WIDTH>,
    ) {
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
                    group.vertex.slice(0..count).unwrap(),
                    glium::index::NoIndices(glium::index::PrimitiveType::Points),
                    &self.program,
                    &uniforms,
                    &DrawParameters {
                        depth: Depth {
                            test: glium::DepthTest::IfLess,
                            write: true,
                            ..Default::default()
                        },
                        backface_culling: BackfaceCullingMode::CullClockwise,
                        ..Default::default()
                    },
                )
                .unwrap();
        }
        frame.finish().unwrap();
    }
}

impl RendererProvider for GeometryFace {
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
        Ok(Box::new(GeometryFaceRenderer::new(facade)))
    }
}

fn main() -> Result<()> {
    env_logger::Builder::from_default_env().init();
    run_renderer::<GeometryFace>()
}
