use macroquad::{
    prelude::info,
    texture::{FilterMode, Texture2D},
};

use crate::model::Planet;

const BOW1: &[u8] = include_bytes!("../resources/bow1.png");
const BOW2: &[u8] = include_bytes!("../resources/bow2.png");
const BOW3: &[u8] = include_bytes!("../resources/bow3.png");
const BOW4: &[u8] = include_bytes!("../resources/bow4.png");
const BOW5: &[u8] = include_bytes!("../resources/bow5.png");
const BOW6: &[u8] = include_bytes!("../resources/bow6.png");
const BOW7: &[u8] = include_bytes!("../resources/bow7.png");
const ARROW: &[u8] = include_bytes!("../resources/arrow.png");
const PLANET1: &[u8] = include_bytes!("../resources/planet1.png");

pub struct ResourceManager {
    pub bow: Vec<Texture2D>,
    pub arrow: Texture2D,
    pub planets: Vec<Texture2D>
}
impl ResourceManager {
    pub fn load() -> Self {
        info!("loading resources");
        Self {
            bow: Self::load_bow(),
            arrow: Self::load_arrow(),
            planets: Self::load_planets(),
        }
    }

    fn load_arrow() -> Texture2D {
        let arrow = Texture2D::from_file_with_format(ARROW, None);
        arrow.set_filter(FilterMode::Nearest);

        arrow
    }

    fn load_planets() -> Vec<Texture2D> {
        let planet_files = [PLANET1];

        Self::load_textures(&planet_files)
    }

    fn load_bow() -> Vec<Texture2D> {
        let bow_files = [BOW1, BOW2, BOW3, BOW4, BOW5, BOW6, BOW7];

        Self::load_textures(&bow_files)
    }

    fn load_textures(bytes: &[&[u8]]) -> Vec<Texture2D> {
        let textures: Vec<_> = bytes.into_iter()
            .map(|bytes| Texture2D::from_file_with_format(bytes, None))
            .collect();
        for texture in &textures {
            texture.set_filter(FilterMode::Nearest);
        }

        textures
    }
}
