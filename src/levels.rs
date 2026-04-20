use macroquad::math::vec2;

use crate::model::{LevelTemplate, TargetTemplate};

pub fn create_levels() -> Vec<LevelTemplate> {
    vec![level1()]
}

fn level1() -> LevelTemplate {
    let target = TargetTemplate::new_static(false, vec2(100.0, 0.0));

    LevelTemplate { target }
}
