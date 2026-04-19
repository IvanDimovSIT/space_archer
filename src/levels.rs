use macroquad::math::vec2;

use crate::model::{LevelTemplate, TargetTemplate};

pub fn create_levels() -> Vec<LevelTemplate> {
    vec![level1()]
}

fn level1() -> LevelTemplate {
    let target = TargetTemplate {
        flipped: false,
        speed: 0.0,
        positions: vec![vec2(100.0, 0.0)],
    };

    LevelTemplate { target }
}
