use macroquad::math::{Vec2, vec2};

#[derive(Debug)]
pub struct TargetTemplate {
    pub flipped: bool,
    pub speed: f32,
    pub positions: Vec<Vec2>,
}
impl TargetTemplate {
    pub fn instance(&'_ self) -> Target<'_> {
        Target {
            template: self,
            position: self.positions[0],
            position_index: 0,
        }
    }
}

#[derive(Debug)]
pub struct Target<'a> {
    pub template: &'a TargetTemplate,
    pub position: Vec2,
    pub position_index: usize,
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
}
impl Default for Arrow {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            direction: vec2(1.0, 0.0),
            speed: 0.0,
            state: ArrowState::Unfired,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Bow {
    pub strength: f32,
    pub direction: Vec2,
}
impl Bow {
    pub const MAX_STRENGTH: f32 = 30.0;
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
