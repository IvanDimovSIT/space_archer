use macroquad::math::{Rect, Vec2, vec2};

use crate::physics::Track;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetFlip {
    Right,
    Top,
    Bottom,
}

#[derive(Debug)]
pub struct TargetTemplate {
    pub flipped: TargetFlip,
    pub speed: f32,
    pub positions: Vec<Vec2>,
    pub index: usize,
}
impl TargetTemplate {
    pub fn new(flipped: TargetFlip, speed: f32, positions: Vec<Vec2>, index: usize) -> Self {
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

    pub fn new_static(flipped: TargetFlip, position: Vec2) -> Self {
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
    pub const WIDTH: f32 = 10.0;
    pub const HEIGHT: f32 = 22.0;

    pub const fn bounding_box(&self) -> Rect {
        if !matches!(self.template.flipped, TargetFlip::Right) {
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

#[derive(Debug, Clone, Copy)]
pub enum PlanetAppearance {
    Red,
    Blue,
    Brown,
}

#[derive(Debug)]
pub struct PlanetTemplate {
    pub positions: Vec<Vec2>,
    pub speed: f32,
    pub index: usize,
    pub size: f32,
    pub appearance: PlanetAppearance,
}
impl PlanetTemplate {
    pub fn new(
        size: f32,
        speed: f32,
        positions: Vec<Vec2>,
        index: usize,
        appearance: PlanetAppearance,
    ) -> Self {
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
            appearance,
        }
    }

    pub fn new_static(size: f32, position: Vec2, appearance: PlanetAppearance) -> Self {
        Self::new(size, 0.0, vec![position], 0, appearance)
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
            appearance: self.appearance,
        }
    }
}

#[derive(Debug)]
pub struct Planet<'a> {
    pub track: Track<'a>,
    pub size: f32,
    pub appearance: PlanetAppearance,
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
impl Arrow {
    pub const SIZE: f32 = 15.0;
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
    pub const SIZE: f32 = 20.0;
    pub const MAX_STRENGTH: f32 = 120.0;
    pub const LOCATION: Vec2 = Vec2::ZERO;
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
pub struct BarierTemplate {
    pub size: Vec2,
    pub positions: Vec<Vec2>,
    pub speed: f32,
    pub index: usize,
}
impl BarierTemplate {
    pub fn new(size: Vec2, speed: f32, positions: Vec<Vec2>, index: usize) -> Self {
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

    pub fn new_static(rect: Rect) -> Self {
        Self::new(vec2(rect.w, rect.h), 0.0, vec![vec2(rect.x, rect.y)], 0)
    }

    pub fn instance(&self) -> Barier<'_> {
        Barier {
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
pub struct Barier<'a> {
    pub track: Track<'a>,
    pub size: Vec2,
}
impl<'a> Barier<'a> {
    pub fn get_rect(&self) -> Rect {
        Rect {
            x: self.track.position.x,
            y: self.track.position.y,
            w: self.size.x,
            h: self.size.y,
        }
    }
}

#[derive(Debug)]
pub struct UFOTemplate {
    pub positions: Vec<Vec2>,
    pub speed: f32,
    pub index: usize,
    pub field_size: Vec2,
}
impl UFOTemplate {
    pub const UFO_SIZE: Vec2 = vec2(10.0, 10.0);
    pub const FIELD_FORCE: f32 = 500.0;

    pub fn new(field_size: Vec2, speed: f32, positions: Vec<Vec2>, index: usize) -> Self {
        assert!(!positions.is_empty());
        assert!(index < positions.len());
        assert!(field_size.x > 0.1);
        assert!(field_size.y > 0.1);
        if positions.len() > 1 {
            assert!(speed > 0.01);
        }
        Self {
            speed,
            positions,
            index,
            field_size,
        }
    }

    pub fn new_static(field_size: Vec2, position: Vec2) -> Self {
        Self::new(field_size, 0.0, vec![position], 0)
    }

    pub fn instance(&self) -> UFO<'_> {
        UFO {
            track: Track {
                points: &self.positions,
                index: self.index,
                position: self.positions[self.index],
                speed: self.speed,
            },
            field_size: self.field_size,
        }
    }
}

#[derive(Debug)]
pub struct UFO<'a> {
    pub track: Track<'a>,
    pub field_size: Vec2,
}
impl<'a> UFO<'a> {
    pub fn field_bb(&self) -> Rect {
        Rect::new(
            self.track.position.x,
            self.track.position.y,
            self.field_size.x,
            self.field_size.y,
        )
    }
}

#[derive(Debug)]
pub struct LevelTemplate {
    pub target: TargetTemplate,
    pub planets: Vec<PlanetTemplate>,
    pub bariers: Vec<BarierTemplate>,
    pub ufos: Vec<UFOTemplate>,
}
impl LevelTemplate {
    pub fn instance(&'_ self) -> Level<'_> {
        Level {
            target: self.target.instance(),
            arrow: Arrow::default(),
            bow: Bow::default(),
            planets: self.planets.iter().map(PlanetTemplate::instance).collect(),
            bariers: self.bariers.iter().map(BarierTemplate::instance).collect(),
            ufos: self.ufos.iter().map(UFOTemplate::instance).collect(),
            time: 0.0,
            accuracy: 0.0,
        }
    }
}
impl Default for LevelTemplate {
    fn default() -> Self {
        Self {
            target: TargetTemplate::new_static(TargetFlip::Right, vec2(100.0, 0.0)),
            planets: vec![],
            bariers: vec![],
            ufos: vec![],
        }
    }
}

#[derive(Debug)]
pub struct Level<'a> {
    pub target: Target<'a>,
    pub planets: Vec<Planet<'a>>,
    pub bariers: Vec<Barier<'a>>,
    pub ufos: Vec<UFO<'a>>,
    pub bow: Bow,
    pub arrow: Arrow,
    pub accuracy: f32,
    pub time: f32,
}
