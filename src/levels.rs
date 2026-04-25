use macroquad::math::vec2;

use crate::model::{LevelTemplate, PlanetTemplate, TargetFlip, TargetTemplate};

pub fn create_levels() -> Vec<LevelTemplate> {
    vec![level1(), level2(), level3(), level4()]
}

fn level1() -> LevelTemplate {
    let target = TargetTemplate::new_static(TargetFlip::Right, vec2(200.0, 0.0));

    LevelTemplate {
        target,
        ..Default::default()
    }
}

fn level2() -> LevelTemplate {
    let target = TargetTemplate::new_static(TargetFlip::Top, vec2(100.0, -40.0));

    LevelTemplate {
        target,
        ..Default::default()
    }
}

fn level3() -> LevelTemplate {
    let target = TargetTemplate::new(
        TargetFlip::Top,
        15.0,
        vec![vec2(50.0, -40.0), vec2(120.0, -40.0)],
        0,
    );

    LevelTemplate {
        target,
        ..Default::default()
    }
}

fn level4() -> LevelTemplate {
    let target = TargetTemplate::new_static(TargetFlip::Right, vec2(200.0, 0.0));
    let planet = PlanetTemplate::new_static(10.0, vec2(50.0, 30.0));

    LevelTemplate {
        target,
        planets: vec![planet],
        ..Default::default()
    }
}
