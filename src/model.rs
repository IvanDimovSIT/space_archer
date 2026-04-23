use macroquad::math::{Rect, Vec2, vec2};

use crate::physics::Track;

#[derive(Debug)]
pub struct TargetTemplate {
    pub flipped: bool,
    pub speed: f32,
    pub positions: Vec<Vec2>,
    pub index: usize,
}
impl TargetTemplate {
    pub fn new(flipped: bool, speed: f32, positions: Vec<Vec2>, index: usize) -> Self {
        assert!(!positions.is_empty());
        assert!(index < positions.len());
        if positions.len() > 1 {
            assert!(speed > 0.01);
        }
        Self {
            flipped,
            speed,
            positions,
            index,
        }
    }

    pub fn new_static(flipped: bool, position: Vec2) -> Self {
        Self::new(flipped, 0.0, vec![position], 0)
    }

    pub fn instance(&'_ self) -> Target<'_> {
        Target {
            template: self,
            track: Track {
                points: &self.positions,
                index: self.index,
                position: self.positions[self.index],
                speed: self.speed,
            },
        }
    }
}

#[derive(Debug)]
pub struct Target<'a> {
    pub template: &'a TargetTemplate,
    pub track: Track<'a>,
}
impl<'a> Target<'a> {
    pub const WIDTH: f32 = 5.0;
    pub const HEIGHT: f32 = 22.0;

    pub const fn bounding_box(&self) -> Rect {
        if self.template.flipped {
            Rect {
                x: self.track.position.x - Self::HEIGHT / 2.0,
                y: self.track.position.y - Self::WIDTH / 2.0,
                w: Self::HEIGHT,
                h: Self::WIDTH,
            }
        } else {
            Rect {
                x: self.track.position.x - Self::WIDTH / 2.0,
                y: self.track.position.y - Self::HEIGHT / 2.0,
                w: Self::WIDTH,
                h: Self::HEIGHT,
            }
        }
    }
}

#[derive(Debug)]
pub struct PlanetTemplate {
    pub positions: Vec<Vec2>,
    pub speed: f32,
    pub index: usize,
    pub size: f32,
}
impl PlanetTemplate {
    pub fn new(size: f32, speed: f32, positions: Vec<Vec2>, index: usize) -> Self {
        assert!(!positions.is_empty());
        assert!(index < positions.len());
        if positions.len() > 1 {
            assert!(speed > 0.01);
        }
        Self {
            size,
            speed,
            positions,
            index,
        }
    }

    pub fn new_static(size: f32, position: Vec2) -> Self {
        Self::new(size, 0.0, vec![position], 0)
    }

    pub fn instance(&self) -> Planet<'_> {
        Planet {
            track: Track {
                points: &self.positions,
                index: self.index,
                position: self.positions[self.index],
                speed: self.speed,
            },
            size: self.size,
        }
    }
}

#[derive(Debug)]
pub struct Planet<'a> {
    pub track: Track<'a>,
    pub size: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrowState {
    Unfired,
    Moving,
    Missed,
    Hit,
}

#[derive(Debug, Clone, Copy)]
pub struct Arrow {
    pub position: Vec2,
    pub velocity: Vec2,
    pub state: ArrowState,
    pub flight_time_s: f32,
}
impl Default for Arrow {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            velocity: vec2(1.0, 0.0),
            state: ArrowState::Unfired,
            flight_time_s: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Bow {
    pub strength: f32,
    pub direction: Vec2,
}
impl Bow {
    pub const MAX_STRENGTH: f32 = 80.0;
}
impl Default for Bow {
    fn default() -> Self {
        Self {
            strength: 0.0,
            direction: vec2(1.0, 0.0),
        }
    }
}

#[derive(Debug)]
pub struct LevelTemplate {
    pub target: TargetTemplate,
    pub planets: Vec<PlanetTemplate>,
}
impl LevelTemplate {
    pub fn instance(&'_ self) -> Level<'_> {
        Level {
            target: self.target.instance(),
            arrow: Arrow::default(),
            bow: Bow::default(),
            accuracy: 0.0,
            planets: self.planets.iter().map(PlanetTemplate::instance).collect(),
        }
    }
}
impl Default for LevelTemplate {
    fn default() -> Self {
        Self {
            target: TargetTemplate::new_static(false, vec2(100.0, 0.0)),
            planets: vec![],
        }
    }
}

#[derive(Debug)]
pub struct Level<'a> {
    pub target: Target<'a>,
    pub planets: Vec<Planet<'a>>,
    pub bow: Bow,
    pub arrow: Arrow,
    pub accuracy: f32,
}
