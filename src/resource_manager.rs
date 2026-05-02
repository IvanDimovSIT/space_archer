use macroquad::{
    audio::{Sound, load_sound_from_bytes},
    prelude::info,
    texture::{FilterMode, Texture2D, build_textures_atlas},
};

use crate::model::PlanetAppearance;

const BOW1: &[u8] = include_bytes!("../resources/bow1.png");
const BOW2: &[u8] = include_bytes!("../resources/bow2.png");
const BOW3: &[u8] = include_bytes!("../resources/bow3.png");
const BOW4: &[u8] = include_bytes!("../resources/bow4.png");
const BOW5: &[u8] = include_bytes!("../resources/bow5.png");
const BOW6: &[u8] = include_bytes!("../resources/bow6.png");
const BOW7: &[u8] = include_bytes!("../resources/bow7.png");
const ARROW: &[u8] = include_bytes!("../resources/arrow.png");
const PLANET1: &[u8] = include_bytes!("../resources/planet1.png");
const PLANET2: &[u8] = include_bytes!("../resources/planet2.png");
const TARGET: &[u8] = include_bytes!("../resources/target.png");
const BG: &[u8] = include_bytes!("../resources/background.png");
const CLICK_SOUND: &[u8] = include_bytes!("../resources/click.wav");
const HIT_SOUND: &[u8] = include_bytes!("../resources/hit.wav");

pub struct Sounds {
    pub click: Sound,
    pub hit: Sound,
}
impl Sounds {
    pub async fn load() -> Self {
        Self {
            click: load_sound_from_bytes(CLICK_SOUND)
                .await
                .expect("Error loading click sound"),
            hit: load_sound_from_bytes(HIT_SOUND)
                .await
                .expect("Error loading hit sound"),
        }
    }
}

pub struct ResourceManager {
    pub sounds: Sounds,
    pub bow: Vec<Texture2D>,
    pub arrow: Texture2D,
    pub planets: Vec<Texture2D>,
    pub target: Texture2D,
    pub background: Texture2D,
}
impl ResourceManager {
    pub async fn load() -> Self {
        info!("loading resources");
        let resource_manager = Self {
            sounds: Sounds::load().await,
            bow: Self::load_bow(),
            arrow: Self::load_texture(ARROW),
            planets: Self::load_planets(),
            target: Self::load_texture(TARGET),
            background: Self::load_texture(BG),
        };
        build_textures_atlas();

        resource_manager
    }

    pub fn get_planet_texture(&self, planet_appearance: PlanetAppearance) -> &Texture2D {
        match planet_appearance {
            PlanetAppearance::Red => &self.planets[0],
            PlanetAppearance::Blue => &self.planets[1],
        }
    }

    fn load_planets() -> Vec<Texture2D> {
        let planet_files = [PLANET1, PLANET2];

        Self::load_textures(&planet_files)
    }

    fn load_bow() -> Vec<Texture2D> {
        let bow_files = [BOW1, BOW2, BOW3, BOW4, BOW5, BOW6, BOW7];

        Self::load_textures(&bow_files)
    }

    fn load_textures(bytes: &[&[u8]]) -> Vec<Texture2D> {
        bytes
            .iter()
            .map(|bytes| Self::load_texture(bytes))
            .collect()
    }

    fn load_texture(bytes: &[u8]) -> Texture2D {
        let texture = Texture2D::from_file_with_format(bytes, None);
        texture.set_filter(FilterMode::Nearest);

        texture
    }
}
