use macroquad::{prelude::info, texture::Texture2D};

const BOW1: &[u8] = include_bytes!("../resources/bow1.png");

pub struct ResourceManager {
    pub bow: Vec<Texture2D>,
}
impl ResourceManager {
    pub fn load() -> Self {
        info!("loading resources");
        Self {
            bow: Self::load_bow(),
        }
    }

    fn load_bow() -> Vec<Texture2D> {
        vec![Texture2D::from_file_with_format(BOW1, None)]
    }
}
