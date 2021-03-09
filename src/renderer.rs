use std::time::Instant;

use anyhow::Result;
use enum_map::EnumMap;
use glium::{backend::Facade, glutin, Frame};

use crate::{
    camera::{model_camera::ModelCamera, Camera, CameraCreation, CameraInput},
    generator::WorldGenerator,
    packs::{basic::*, Pack, SimpleBlockId},
    BlockId, SolidBlockDefinition, World,
};

pub struct WorldInfo<
    Id: BlockId,
    const SIZE: usize,
    const WIDTH: usize,
    const CHUNK_SIZE: usize,
    const CHUNK_WIDTH: usize,
> {
    pub camera: Box<dyn Camera>,
    pub world: Box<World<Id, SIZE, WIDTH, CHUNK_SIZE, CHUNK_WIDTH>>,
    pub definitions: &'static EnumMap<Id, SolidBlockDefinition>,
    pub texture: glium::texture::srgb_texture2d_array::SrgbTexture2dArray,
}

fn mock_gen_world<
    F: Facade,
    P: Pack,
    const SIZE: usize,
    const WIDTH: usize,
    const CHUNK_SIZE: usize,
    const CHUNK_WIDTH: usize,
>(
    facade: &F,
) -> Result<WorldInfo<P::Id, SIZE, WIDTH, CHUNK_SIZE, CHUNK_WIDTH>>
where
    P::Id: SimpleBlockId,
{
    let texture = glium::texture::srgb_texture2d_array::SrgbTexture2dArray::new(
        facade,
        BasicPack::get_textures(),
    )?;
    let generator = crate::generator::random::RandomGenerator::Odd;
    let mut world = World::create();
    generator.generate(world.as_mut());
    let width = WIDTH * CHUNK_WIDTH;
    let height = CHUNK_SIZE / CHUNK_WIDTH / CHUNK_WIDTH;
    let length = SIZE / WIDTH * CHUNK_WIDTH;
    Ok(WorldInfo {
        camera: Box::new(ModelCamera::new(width, height, length)),
        world,
        definitions: P::get_map(),
        texture,
    })
}

pub trait Renderer<
    Id: BlockId,
    const SIZE: usize,
    const WIDTH: usize,
    const CHUNK_SIZE: usize,
    const CHUNK_WIDTH: usize,
>
{
    fn prepare(&mut self, info: &WorldInfo<Id, SIZE, WIDTH, CHUNK_SIZE, CHUNK_WIDTH>);
    fn render(&self, frame: Frame, info: &WorldInfo<Id, SIZE, WIDTH, CHUNK_SIZE, CHUNK_WIDTH>);
}

pub trait RendererProvider {
    fn get_renderer<
        F: Facade,
        Id: BlockId,
        const SIZE: usize,
        const WIDTH: usize,
        const CHUNK_SIZE: usize,
        const CHUNK_WIDTH: usize,
    >(
        facade: &F,
        info: &WorldInfo<Id, SIZE, WIDTH, CHUNK_SIZE, CHUNK_WIDTH>,
    ) -> Result<Box<dyn Renderer<Id, SIZE, WIDTH, CHUNK_SIZE, CHUNK_WIDTH>>>;
}

pub fn run_renderer<P: RendererProvider>() -> Result<()> {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_multisampling(4);
    let display = glium::Display::new(wb, cb, &event_loop)?;

    let start = Instant::now();
    let mut world = mock_gen_world::<glium::Display, BasicPack, 16, 4, 8192, 16>(&display)?;
    log::info!("start {:?}", start.elapsed());
    let world_created = Instant::now();
    let mut renderer = P::get_renderer(&display, &world)?;
    log::info!("renderer {:?}", world_created.elapsed());
    let renderer_created = Instant::now();
    renderer.prepare(&world);
    log::info!("renderer prepare {:?}", renderer_created.elapsed());
    // let renderer_prepared = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = glutin::event_loop::ControlFlow::Poll;
        // *control_flow = glutin::event_loop::ControlFlow::WaitUntil(
        //     Instant::now() + Duration::from_nanos(16_666_667),
        // );
        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                    match input.virtual_keycode {
                        Some(glutin::event::VirtualKeyCode::PageUp) => {
                            world.camera.feed_input(CameraInput::Move([0.0, 1.0, 0.0]));
                        }
                        Some(glutin::event::VirtualKeyCode::PageDown) => {
                            world.camera.feed_input(CameraInput::Move([0.0, -1.0, 0.0]));
                        }
                        Some(glutin::event::VirtualKeyCode::Up) => {
                            world.camera.feed_input(CameraInput::Move([0.0, 0.0, 1.0]));
                        }
                        Some(glutin::event::VirtualKeyCode::Down) => {
                            world.camera.feed_input(CameraInput::Move([0.0, 0.0, -1.0]));
                        }
                        Some(glutin::event::VirtualKeyCode::Left) => {
                            world.camera.feed_input(CameraInput::Move([-1.0, 0.0, 0.0]));
                        }
                        Some(glutin::event::VirtualKeyCode::Right) => {
                            world.camera.feed_input(CameraInput::Move([1.0, 0.0, 0.0]));
                        }
                        Some(glutin::event::VirtualKeyCode::Escape) => {
                            *control_flow = glutin::event_loop::ControlFlow::Exit;
                            return;
                        }
                        Some(_) => {}
                        None => {}
                    }
                }
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                glutin::event::StartCause::Poll => (),
                _ => return,
            },
            _ => return,
        }

        // renderer.prepare(&world);
        renderer.render(display.draw(), &world);
    });
}
