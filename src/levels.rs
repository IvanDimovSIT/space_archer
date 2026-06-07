use macroquad::{
    math::{Rect, vec2},
    prelude::error,
};

use crate::model::{
    BarierTemplate, KeyTemplate, LevelTemplate, PlanetAppearance, PlanetTemplate, Target,
    TargetFlip, TargetTemplate, UFOTemplate,
};

pub fn create_levels() -> Vec<LevelTemplate> {
    let levels = vec![
        introduction_level(),
        top_target_level(),
        moving_top_target_level(),
        planet_introduction_level(),
        planet_introduction_with_barier_level(),
        half_strength_level(),
        moving_2_planets_level(),
        ufo_introduction_level(),
        ufo_launch_down_with_planet_level(),
        three_planets_moving_brarier_level(),
        moving_two_bariers_level(),
        moving_ufo_with_target_level(),
        underside_with_moving_barier_level(),
        key_introdiction_level(),
        two_keys_with_planet_level(),
        moving_key_between_planets_level(),
    ];
    validate_levels(&levels);

    levels
}

fn validate_levels(levels: &[LevelTemplate]) {
    for (ind, level) in levels.iter().enumerate() {
        let has_keys = !level.keys.is_empty();
        let has_locked_bariers = level.bariers.iter().any(|b| b.locked);

        if has_keys != has_locked_bariers {
            error!(
                "Error in level template {}, has_keys != has_locked_bariers",
                ind
            );
        }
    }
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
    let barier = BarierTemplate::new_static(Rect::new(45.0, -75.0, 10.0, 80.0), false);

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
    let barier = BarierTemplate::new_static(Rect::new(53.0, 30.0, 10.0, 45.0), false);

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
    let barrier = BarierTemplate::new_static(Rect::new(90.0, 0.0, 10.0, 70.0), false);

    LevelTemplate {
        target,
        ufos: vec![ufo],
        bariers: vec![barrier],
        ..Default::default()
    }
}

fn ufo_launch_down_with_planet_level() -> LevelTemplate {
    let target = TargetTemplate::new_static(TargetFlip::Bottom, vec2(150.0, 35.0));
    let ufo = UFOTemplate::new_static(vec2(18.0, 40.0), vec2(110.0, -50.0));
    let barrier = BarierTemplate::new_static(Rect::new(80.0, -10.0, 10.0, 60.0), false);
    let planet = PlanetTemplate::new_static(19.0, vec2(115.0, 30.0), PlanetAppearance::Brown);

    LevelTemplate {
        target,
        ufos: vec![ufo],
        bariers: vec![barrier],
        planets: vec![planet],
        ..Default::default()
    }
}

fn three_planets_moving_brarier_level() -> LevelTemplate {
    let target = TargetTemplate::new_static(TargetFlip::Right, vec2(115.0, 48.0));
    let barrier = BarierTemplate::new_static(Rect::new(60.0, 30.0, 10.0, 40.0), false);
    let ufo = UFOTemplate::new_static(vec2(30.0, 65.0), vec2(75.0, -65.0));
    let planet1 = PlanetTemplate::new_static(14.0, vec2(50.0, -25.0), PlanetAppearance::Brown);
    let planet2 = PlanetTemplate::new_static(22.0, vec2(125.0, 15.0), PlanetAppearance::Red);

    LevelTemplate {
        target,
        bariers: vec![barrier],
        planets: vec![planet1, planet2],
        ufos: vec![ufo],
        ..Default::default()
    }
}

fn moving_two_bariers_level() -> LevelTemplate {
    let target = TargetTemplate::new_static(TargetFlip::Right, vec2(200.0, 0.0));
    let barier1 = BarierTemplate::new(
        vec2(10.0, 30.0),
        30.0,
        vec![vec2(80.0, -30.0), vec2(80.0, 30.0)],
        0,
        false,
    );
    let barier2 = BarierTemplate::new(
        vec2(10.0, 40.0),
        20.0,
        vec![vec2(160.0, -40.0), vec2(160.0, 40.0)],
        0,
        false,
    );
    let ufo = UFOTemplate::new(
        vec2(20.0, 80.0),
        40.0,
        vec![vec2(10.0, -60.0), vec2(80.0, -60.0), vec2(150.0, -60.0)],
        1,
    );

    LevelTemplate {
        target,
        bariers: vec![barier1, barier2],
        ufos: vec![ufo],
        ..Default::default()
    }
}

fn moving_ufo_with_target_level() -> LevelTemplate {
    let speed = 20.0;
    let target = TargetTemplate::new(
        TargetFlip::Top,
        speed,
        vec![vec2(60.0, -35.0), vec2(140.0, -35.0)],
        0,
    );
    let ufo = UFOTemplate::new(
        vec2(30.0, 60.0),
        speed,
        vec![
            vec2(60.0 - Target::WIDTH / 2.0, -48.0),
            vec2(140.0 - Target::WIDTH / 2.0, -48.0),
        ],
        0,
    );
    let barier = BarierTemplate::new(
        vec2(8.0, 18.0),
        speed,
        vec![
            vec2(45.0 - Target::WIDTH / 2.0, -45.0),
            vec2(125.0 - Target::WIDTH / 2.0, -45.0),
        ],
        0,
        false,
    );
    let planet = PlanetTemplate::new_static(23.0, vec2(33.0, -70.0), PlanetAppearance::Brown);

    LevelTemplate {
        target,
        ufos: vec![ufo],
        bariers: vec![barier],
        planets: vec![planet],
        ..Default::default()
    }
}

fn underside_with_moving_barier_level() -> LevelTemplate {
    let target = TargetTemplate::new_static(TargetFlip::Top, vec2(80.0, -50.0));
    let planet = PlanetTemplate::new_static(14.0, vec2(50.0, 30.0), PlanetAppearance::Blue);
    let static_barier = BarierTemplate::new_static(Rect::new(45.0, -75.0, 10.0, 80.0), false);
    let moving_barier = BarierTemplate::new(
        vec2(20.0, 8.0),
        20.0,
        vec![vec2(55.0, -30.0), vec2(100.0, -30.0)],
        0,
        false,
    );

    LevelTemplate {
        target,
        planets: vec![planet],
        bariers: vec![static_barier, moving_barier],
        ..Default::default()
    }
}

fn key_introdiction_level() -> LevelTemplate {
    let target = TargetTemplate::new_static(TargetFlip::Right, vec2(140.0, 0.0));
    let barier = BarierTemplate::new_static(Rect::new(90.0, -50.0, 15.0, 100.0), true);
    let key = KeyTemplate::new_static(vec2(60.0, 40.0));

    LevelTemplate {
        target,
        bariers: vec![barier],
        keys: vec![key],
        ..Default::default()
    }
}

fn two_keys_with_planet_level() -> LevelTemplate {
    let target = TargetTemplate::new_static(TargetFlip::Right, vec2(200.0, 0.0));
    let barier1 = BarierTemplate::new_static(Rect::new(170.0, -50.0, 15.0, 100.0), true);
    let barier2 = BarierTemplate::new(
        vec2(10.0, 30.0),
        25.0,
        vec![vec2(60.0, -60.0), vec2(60.0, 10.0)],
        0,
        false,
    );
    let key1 = KeyTemplate::new_static(vec2(90.0, 40.0));
    let key2 = KeyTemplate::new(15.0, vec![vec2(90.0, -40.0), vec2(130.0, -40.0)], 0);
    let planet = PlanetTemplate::new_static(16.0, vec2(40.0, 40.0), PlanetAppearance::Red);

    LevelTemplate {
        target,
        bariers: vec![barier1, barier2],
        keys: vec![key1, key2],
        planets: vec![planet],
        ..Default::default()
    }
}

fn moving_key_between_planets_level() -> LevelTemplate {
    let target = TargetTemplate::new_static(TargetFlip::Right, vec2(195.0, -35.0));
    let planet1 = PlanetTemplate::new_static(10.0, vec2(60.0, 0.0), PlanetAppearance::Red);
    let planet2 = PlanetTemplate::new_static(17.0, vec2(150.0, 0.0), PlanetAppearance::Blue);
    let key1 = KeyTemplate::new(50.0, vec![vec2(80.0, 0.0), vec2(120.0, 0.0)], 0);
    let key2 = KeyTemplate::new_static(vec2(180.0, 40.0));
    let barier1 = BarierTemplate::new_static(Rect::new(160.0, -80.0, 10.0, 60.0), true);
    let barier2 = BarierTemplate::new_static(Rect::new(165.0, 10.0, 60.0, 10.0), false);
    let barier3 = BarierTemplate::new_static(Rect::new(50.0, -80.0, 10.0, 60.0), false);

    LevelTemplate {
        target,
        planets: vec![planet1, planet2],
        bariers: vec![barier1, barier2, barier3],
        keys: vec![key1, key2],
        ..Default::default()
    }
}
