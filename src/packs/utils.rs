use std::{collections::HashMap, iter::FromIterator, ops::Index};

use glium::texture::RawImage2d;
use image::{DynamicImage, GenericImageView};

use crate::TextureIndex;

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct SpriteDefinition {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl SpriteDefinition {
    pub fn crop_image(&self, image: &image::DynamicImage) -> image::DynamicImage {
        let &SpriteDefinition {
            x,
            y,
            width,
            height,
        } = self;
        image.crop_imm(x, y, width, height)
    }
}

impl From<(u32, u32, u32, u32)> for SpriteDefinition {
    fn from((x, y, width, height): (u32, u32, u32, u32)) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

#[macro_export]
macro_rules! sprite {
    ($name:literal) => {
        SPRITESHEET[concat!($name, ".png")].into()
    };
}

#[macro_export]
macro_rules! spritesheet {
    [<TextureAtlas imagePath=$path:literal>$(<SubTexture name=$name:literal x=$x:literal y=$y:literal width=$width:literal height=$height:literal />)*</TextureAtlas>] => {
        [$(($name, SpriteDefinition::from(($x.parse::<u32>().unwrap(), $y.parse::<u32>().unwrap(), $width.parse::<u32>().unwrap(), $height.parse::<u32>().unwrap())))),*].iter().cloned().collect()
    }
}

#[derive(Default)]
pub struct SpriteArray {
    tiles: Vec<SpriteDefinition>,
    map: HashMap<&'static str, TextureIndex>,
}

impl SpriteArray {
    pub fn get_image_array(
        &self,
        origin: &'static DynamicImage,
    ) -> Vec<glium::texture::RawImage2d<'static, u8>> {
        self.tiles
            .iter()
            .map(|def| def.crop_image(origin))
            .map(|img| {
                let dim = img.dimensions();
                RawImage2d::from_raw_rgba(img.into_bytes(), dim)
            })
            .collect()
    }
}

impl Index<&'static str> for SpriteArray {
    type Output = TextureIndex;

    fn index(&self, index: &'static str) -> &Self::Output {
        &self.map[index]
    }
}

impl FromIterator<(&'static str, SpriteDefinition)> for SpriteArray {
    fn from_iter<T: IntoIterator<Item = (&'static str, SpriteDefinition)>>(iter: T) -> Self {
        let mut ret: Self = Default::default();
        for (name, def) in iter {
            let idx = ret.tiles.len() as u16;
            ret.tiles.push(def);
            ret.map.insert(name, TextureIndex(idx));
        }
        ret
    }
}
