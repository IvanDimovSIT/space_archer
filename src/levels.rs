use macroquad::math::{Rect, vec2};

use crate::model::{
    BarierTemplate, LevelTemplate, PlanetAppearance, PlanetTemplate, TargetFlip, TargetTemplate,
    UFOTemplate,
};

pub fn create_levels() -> Vec<LevelTemplate> {
    vec![
        introduction_level(),
        top_target_level(),
        moving_top_target_level(),
        planet_introduction_level(),
        planet_introduction_with_barier_level(),
        half_strength_level(),
        moving_2_planets_level(),
        ufo_introduction_level(),
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
    let planet = PlanetTemplate::new_static(10.0, vec2(50.0, 30.0), PlanetAppearance::Red);

    LevelTemplate {
        target,
        planets: vec![planet],
        ..Default::default()
    }
}

fn planet_introduction_with_barier_level() -> LevelTemplate {
    let target = TargetTemplate::new_static(TargetFlip::Right, vec2(200.0, 0.0));
    let planet = PlanetTemplate::new_static(10.0, vec2(50.0, 30.0), PlanetAppearance::Red);
    let barier = BarierTemplate::new_static(Rect::new(45.0, -75.0, 10.0, 80.0));

    LevelTemplate {
        target,
        planets: vec![planet],
        bariers: vec![barier],
        ..Default::default()
    }
}

fn half_strength_level() -> LevelTemplate {
    let target = TargetTemplate::new_static(TargetFlip::Bottom, vec2(120.0, 48.0));
    let planet = PlanetTemplate::new_static(20.0, vec2(90.0, 35.0), PlanetAppearance::Blue);
    let barier = BarierTemplate::new_static(Rect::new(53.0, 30.0, 10.0, 45.0));

    LevelTemplate {
        target,
        planets: vec![planet],
        bariers: vec![barier],
        ..Default::default()
    }
}

fn moving_2_planets_level() -> LevelTemplate {
    let target = TargetTemplate::new_static(TargetFlip::Right, vec2(200.0, 0.0));
    let planet1 = PlanetTemplate::new(
        13.0,
        12.0,
        vec![vec2(60.0, 45.0), vec2(60.0, 0.0), vec2(60.0, -45.0)],
        1,
        PlanetAppearance::Blue,
    );
    let planet2 = PlanetTemplate::new(
        13.0,
        12.0,
        vec![vec2(130.0, -45.0), vec2(130.0, 0.0), vec2(130.0, 45.0)],
        1,
        PlanetAppearance::Brown,
    );

    LevelTemplate {
        target,
        planets: vec![planet1, planet2],
        ..Default::default()
    }
}

fn ufo_introduction_level() -> LevelTemplate {
    let target = TargetTemplate::new_static(TargetFlip::Bottom, vec2(120.0, 20.0));
    let ufo = UFOTemplate::new(
        vec2(18.0, 40.0),
        20.0,
        vec![vec2(80.0, -50.0), vec2(130.0, -50.0)],
        0,
    );
    let barrier = BarierTemplate::new_static(Rect::new(90.0, 0.0, 10.0, 70.0));

    LevelTemplate {
        target,
        ufos: vec![ufo],
        bariers: vec![barrier],
        ..Default::default()
    }
}
