use macroquad::math::{Rect, vec2};

use crate::model::{BarierTemplate, LevelTemplate, PlanetTemplate, TargetFlip, TargetTemplate};

pub fn create_levels() -> Vec<LevelTemplate> {
    vec![
        introduction_level(),
        top_target_level(),
        moving_top_target_level(),
        planet_introduction_level(),
        planet_introduction_with_barier_level(),
    ]
}

fn introduction_level() -> LevelTemplate {
    let target = TargetTemplate::new_static(TargetFlip::Right, vec2(200.0, 0.0));

    LevelTemplate {
        target,
        ..Default::default()
    }
}

fn top_target_level() -> LevelTemplate {
    let target = TargetTemplate::new_static(TargetFlip::Top, vec2(100.0, -40.0));

    LevelTemplate {
        target,
        ..Default::default()
    }
}

fn moving_top_target_level() -> LevelTemplate {
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

fn planet_introduction_level() -> LevelTemplate {
    let target = TargetTemplate::new_static(TargetFlip::Right, vec2(200.0, 0.0));
    let planet = PlanetTemplate::new_static(10.0, vec2(50.0, 30.0));

    LevelTemplate {
        target,
        planets: vec![planet],
        ..Default::default()
    }
}

fn planet_introduction_with_barier_level() -> LevelTemplate {
    let target = TargetTemplate::new_static(TargetFlip::Right, vec2(200.0, 0.0));
    let planet = PlanetTemplate::new_static(10.0, vec2(50.0, 30.0));
    let barier = BarierTemplate::new_static(Rect::new(45.0, -75.0, 10.0, 80.0));

    LevelTemplate {
        target,
        planets: vec![planet],
        bariers: vec![barier],
        ..Default::default()
    }
}
