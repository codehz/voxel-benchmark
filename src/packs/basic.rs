use super::{
    utils::{SpriteArray, SpriteDefinition},
    Pack, SimpleBlockId,
};
use crate::*;
use enum_map::{enum_map, Enum};
use image::DynamicImage;
use lazy_static::lazy_static;
use rand::Rng;
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{Display, EnumCount, EnumIter};

pub struct BasicPack;

#[derive(Display, Debug, PartialEq, Eq, Clone, Copy, Enum, EnumCount, EnumIter)]
pub enum BasicId {
    Dirt,
    DirtGrass,
    DirtSand,
    DirtSnow,
    WoodRed,
    Wood,
    BrickGrey,
    BrickRed,
    Stone,
    Sand,
    RedStone,
    RedSand,
    GreyStone,
    GreySand,
}

impl SimpleBlockId for BasicId {
    fn get_simple_block() -> Self {
        Self::Dirt
    }

    fn get_simple_top_block() -> Self {
        Self::DirtGrass
    }

    fn get_random_block() -> Self {
        let tmp = rand::thread_rng().gen_range(0..BasicId::COUNT);
        BasicId::iter().nth(tmp as usize).unwrap()
    }
}

lazy_static! {
    static ref TILES: DynamicImage = image::load_from_memory_with_format(
        include_bytes!("../../assets/tiles.png"),
        image::ImageFormat::Png
    )
    .unwrap();
    static ref SPRITESHEET: SpriteArray = spritesheet! [
        <TextureAtlas imagePath="spritesheet_tiles.png">
            <SubTexture name="brick_grey.png" x="512" y="256" width="128" height="128"/>
            <SubTexture name="brick_red.png" x="1024" y="384" width="128" height="128"/>
            <SubTexture name="cactus_inside.png" x="1024" y="256" width="128" height="128"/>
            <SubTexture name="cactus_side.png" x="1024" y="128" width="128" height="128"/>
            <SubTexture name="cactus_top.png" x="1024" y="0" width="128" height="128"/>
            <SubTexture name="cotton_blue.png" x="896" y="1152" width="128" height="128"/>
            <SubTexture name="cotton_green.png" x="896" y="1024" width="128" height="128"/>
            <SubTexture name="cotton_red.png" x="896" y="896" width="128" height="128"/>
            <SubTexture name="cotton_tan.png" x="896" y="768" width="128" height="128"/>
            <SubTexture name="dirt.png" x="896" y="640" width="128" height="128"/>
            <SubTexture name="dirt_grass.png" x="896" y="512" width="128" height="128"/>
            <SubTexture name="dirt_sand.png" x="896" y="384" width="128" height="128"/>
            <SubTexture name="dirt_snow.png" x="896" y="256" width="128" height="128"/>
            <SubTexture name="fence_stone.png" x="896" y="128" width="128" height="128"/>
            <SubTexture name="fence_wood.png" x="896" y="0" width="128" height="128"/>
            <SubTexture name="glass.png" x="768" y="1152" width="128" height="128"/>
            <SubTexture name="glass_frame.png" x="768" y="1024" width="128" height="128"/>
            <SubTexture name="grass1.png" x="768" y="896" width="128" height="128"/>
            <SubTexture name="grass2.png" x="768" y="768" width="128" height="128"/>
            <SubTexture name="grass3.png" x="768" y="640" width="128" height="128"/>
            <SubTexture name="grass4.png" x="768" y="512" width="128" height="128"/>
            <SubTexture name="grass_brown.png" x="768" y="384" width="128" height="128"/>
            <SubTexture name="grass_tan.png" x="768" y="256" width="128" height="128"/>
            <SubTexture name="grass_top.png" x="768" y="128" width="128" height="128"/>
            <SubTexture name="gravel_dirt.png" x="768" y="0" width="128" height="128"/>
            <SubTexture name="gravel_stone.png" x="640" y="1152" width="128" height="128"/>
            <SubTexture name="greysand.png" x="640" y="1024" width="128" height="128"/>
            <SubTexture name="greystone.png" x="640" y="896" width="128" height="128"/>
            <SubTexture name="greystone_ruby.png" x="640" y="768" width="128" height="128"/>
            <SubTexture name="greystone_ruby_alt.png" x="640" y="640" width="128" height="128"/>
            <SubTexture name="greystone_sand.png" x="640" y="512" width="128" height="128"/>
            <SubTexture name="ice.png" x="640" y="384" width="128" height="128"/>
            <SubTexture name="lava.png" x="640" y="256" width="128" height="128"/>
            <SubTexture name="leaves.png" x="640" y="128" width="128" height="128"/>
            <SubTexture name="leaves_orange.png" x="640" y="0" width="128" height="128"/>
            <SubTexture name="leaves_orange_transparent.png" x="512" y="1152" width="128" height="128"/>
            <SubTexture name="leaves_transparent.png" x="512" y="1024" width="128" height="128"/>
            <SubTexture name="mushroom_brown.png" x="512" y="896" width="128" height="128"/>
            <SubTexture name="mushroom_red.png" x="512" y="768" width="128" height="128"/>
            <SubTexture name="mushroom_tan.png" x="512" y="640" width="128" height="128"/>
            <SubTexture name="oven.png" x="512" y="512" width="128" height="128"/>
            <SubTexture name="redsand.png" x="512" y="384" width="128" height="128"/>
            <SubTexture name="redstone.png" x="1024" y="512" width="128" height="128"/>
            <SubTexture name="redstone_emerald.png" x="512" y="128" width="128" height="128"/>
            <SubTexture name="redstone_emerald_alt.png" x="512" y="0" width="128" height="128"/>
            <SubTexture name="redstone_sand.png" x="384" y="1152" width="128" height="128"/>
            <SubTexture name="rock.png" x="384" y="1024" width="128" height="128"/>
            <SubTexture name="rock_moss.png" x="384" y="896" width="128" height="128"/>
            <SubTexture name="sand.png" x="384" y="768" width="128" height="128"/>
            <SubTexture name="snow.png" x="384" y="640" width="128" height="128"/>
            <SubTexture name="stone.png" x="384" y="512" width="128" height="128"/>
            <SubTexture name="stone_browniron.png" x="384" y="384" width="128" height="128"/>
            <SubTexture name="stone_browniron_alt.png" x="384" y="256" width="128" height="128"/>
            <SubTexture name="stone_coal.png" x="384" y="128" width="128" height="128"/>
            <SubTexture name="stone_coal_alt.png" x="384" y="0" width="128" height="128"/>
            <SubTexture name="stone_diamond.png" x="256" y="1152" width="128" height="128"/>
            <SubTexture name="stone_diamond_alt.png" x="256" y="1024" width="128" height="128"/>
            <SubTexture name="stone_dirt.png" x="256" y="896" width="128" height="128"/>
            <SubTexture name="stone_gold.png" x="256" y="768" width="128" height="128"/>
            <SubTexture name="stone_gold_alt.png" x="256" y="640" width="128" height="128"/>
            <SubTexture name="stone_grass.png" x="256" y="512" width="128" height="128"/>
            <SubTexture name="stone_iron.png" x="256" y="384" width="128" height="128"/>
            <SubTexture name="stone_iron_alt.png" x="256" y="256" width="128" height="128"/>
            <SubTexture name="stone_sand.png" x="256" y="128" width="128" height="128"/>
            <SubTexture name="stone_silver.png" x="256" y="0" width="128" height="128"/>
            <SubTexture name="stone_silver_alt.png" x="128" y="1152" width="128" height="128"/>
            <SubTexture name="stone_snow.png" x="128" y="1024" width="128" height="128"/>
            <SubTexture name="table.png" x="128" y="896" width="128" height="128"/>
            <SubTexture name="track_corner.png" x="128" y="768" width="128" height="128"/>
            <SubTexture name="track_corner_alt.png" x="128" y="640" width="128" height="128"/>
            <SubTexture name="track_straight.png" x="128" y="512" width="128" height="128"/>
            <SubTexture name="track_straight_alt.png" x="128" y="384" width="128" height="128"/>
            <SubTexture name="trunk_bottom.png" x="128" y="256" width="128" height="128"/>
            <SubTexture name="trunk_mid.png" x="128" y="128" width="128" height="128"/>
            <SubTexture name="trunk_side.png" x="128" y="0" width="128" height="128"/>
            <SubTexture name="trunk_top.png" x="0" y="1152" width="128" height="128"/>
            <SubTexture name="trunk_white_side.png" x="0" y="1024" width="128" height="128"/>
            <SubTexture name="trunk_white_top.png" x="0" y="896" width="128" height="128"/>
            <SubTexture name="water.png" x="0" y="768" width="128" height="128"/>
            <SubTexture name="wheat_stage1.png" x="0" y="640" width="128" height="128"/>
            <SubTexture name="wheat_stage2.png" x="0" y="512" width="128" height="128"/>
            <SubTexture name="wheat_stage3.png" x="0" y="384" width="128" height="128"/>
            <SubTexture name="wheat_stage4.png" x="0" y="256" width="128" height="128"/>
            <SubTexture name="wood.png" x="0" y="128" width="128" height="128"/>
            <SubTexture name="wood_red.png" x="0" y="0" width="128" height="128"/>
        </TextureAtlas>
    ];
    static ref DEFINITIONS: enum_map::EnumMap<BasicId, SolidBlockDefinition> = enum_map! {
        BasicId::Dirt => sprite!("dirt"),
        BasicId::DirtGrass => SolidBlockDefinition(enum_map! {
            BlockFace::Up => sprite!("grass_top"),
            BlockFace::Down => sprite!("dirt"),
            _ => sprite!("dirt_grass"),
        }),
        BasicId::DirtSand => SolidBlockDefinition(enum_map! {
            BlockFace::Up => sprite!("sand"),
            BlockFace::Down => sprite!("dirt"),
            _ => sprite!("dirt_sand"),
        }),
        BasicId::DirtSnow => SolidBlockDefinition(enum_map! {
            BlockFace::Up => sprite!("snow"),
            BlockFace::Down => sprite!("dirt"),
            _ => sprite!("dirt_snow"),
        }),
        BasicId::WoodRed => sprite!("wood_red"),
        BasicId::Wood => sprite!("wood"),
        BasicId::BrickGrey => sprite!("brick_grey"),
        BasicId::BrickRed => sprite!("brick_red"),
        BasicId::Stone => sprite!("stone"),
        BasicId::Sand => sprite!("sand"),
        BasicId::RedStone => sprite!("redstone"),
        BasicId::RedSand => sprite!("redsand"),
        BasicId::GreyStone => sprite!("greystone"),
        BasicId::GreySand => sprite!("greysand"),
    };
}

impl Pack for BasicPack {
    type Id = BasicId;

    fn get_textures() -> Vec<glium::texture::RawImage2d<'static, u8>> {
        SPRITESHEET.get_image_array(&TILES)
    }

    fn get_map() -> &'static enum_map::EnumMap<Self::Id, SolidBlockDefinition> {
        &*DEFINITIONS
    }
}
