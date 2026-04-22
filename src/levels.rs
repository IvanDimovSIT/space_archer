use macroquad::math::vec2;

use crate::model::{LevelTemplate, PlanetTemplate, TargetTemplate};

pub fn create_levels() -> Vec<LevelTemplate> {
    vec![level1(), level2()]
}

fn level1() -> LevelTemplate {
    let target = TargetTemplate::new_static(false, vec2(200.0, 0.0));

    LevelTemplate {
        target,
        ..Default::default()
    }
}

fn level2() -> LevelTemplate {
    let target = TargetTemplate::new_static(false, vec2(200.0, 0.0));
    let planet = PlanetTemplate::new_static(10.0, vec2(50.0, 30.0));

    LevelTemplate {
        target,
        planets: vec![planet],
        ..Default::default()
    }
}
