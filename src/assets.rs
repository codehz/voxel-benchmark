use anyhow::Result;

use glium::{backend::Facade, texture::RawImage2d};

pub fn load_texture<F: Facade>(
    facade: &F,
    data: &'static [u8],
) -> Result<glium::texture::Texture2d> {
    let image = image::load_from_memory_with_format(data, image::ImageFormat::Png)?.to_rgba8();
    let image_dimensions = image.dimensions();
    let image = RawImage2d::from_raw_rgba(image.into_raw(), image_dimensions);
    Ok(glium::texture::Texture2d::new(facade, image)?)
}
