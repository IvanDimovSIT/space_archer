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
    const WIDTH: f32 = 3.0;
    const HEIGHT: f32 = 15.0;

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
    pub direction: Vec2,
    pub speed: f32,
    pub state: ArrowState,
    pub flight_time_s: f32,
}
impl Default for Arrow {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            direction: vec2(1.0, 0.0),
            speed: 0.0,
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
}
impl LevelTemplate {
    pub fn instance(&'_ self) -> Level<'_> {
        Level {
            template: self,
            target: self.target.instance(),
            arrow: Arrow::default(),
            bow: Bow::default(),
        }
    }
}

#[derive(Debug)]
pub struct Level<'a> {
    pub template: &'a LevelTemplate,
    pub target: Target<'a>,
    pub bow: Bow,
    pub arrow: Arrow,
}
impl<'a> Level<'a> {
    pub fn reset(&mut self) {
        *self = self.template.instance();
    }
}
