use macroquad::{
    camera::{Camera2D, set_camera},
    color::BLACK,
    input::{is_mouse_button_down, is_mouse_button_released, mouse_position},
    math::{Vec2, vec2},
    window::{clear_background, screen_height, screen_width},
};

use crate::{
    draw::{draw_arrow, draw_bow},
    model::{ArrowState, Bow, Level, LevelTemplate},
    resource_manager::ResourceManager,
};

pub struct Game<'a> {
    levels: &'a [LevelTemplate],
    current_level_index: usize,
    level: Level<'a>,
    camera: Camera2D,
}
impl<'a> Game<'a> {
    pub fn new(levels: &'a [LevelTemplate], current_level_index: usize) -> Self {
        assert!(!levels.is_empty());
        assert!(current_level_index < levels.len());

        let camera = Camera2D {
            target: vec2(80.0, 0.0),
            zoom: vec2(0.01, 0.01),
            ..Default::default()
        };

        Self {
            levels,
            current_level_index,
            level: levels[current_level_index].instance(),
            camera,
        }
    }

    pub fn draw(&self, resource_manager: &ResourceManager) {
        clear_background(BLACK);
        draw_arrow(&self.level.arrow);
        draw_bow(&self.level.bow, resource_manager);
    }

    pub fn update(&mut self, delta: f32) {
        self.update_camera();
        set_camera(&self.camera);
        let player_aim = self.get_player_aim();
        self.update_arrow(delta, player_aim);
    }

    fn update_arrow(&mut self, delta: f32, aim: Vec2) {
        match self.level.arrow.state {
            ArrowState::Unfired => {
                self.level.arrow.direction =
                    (aim - self.level.arrow.position).normalize_or(vec2(1.0, 0.0));
                self.level.bow.direction = self.level.arrow.direction;
                if is_mouse_button_down(macroquad::input::MouseButton::Left) {
                    const BOW_PULL_SPEED: f32 = 8.0;
                    self.level.bow.strength = (self.level.bow.strength + BOW_PULL_SPEED * delta)
                        .clamp(0.0, Bow::MAX_STRENGTH);
                } else if is_mouse_button_released(macroquad::input::MouseButton::Left) {
                    self.level.arrow.state = ArrowState::Moving;
                    self.level.arrow.speed = self.level.bow.strength;
                    self.level.bow.strength = 0.0;
                }
            }
            ArrowState::Moving => {
                self.level.arrow.position +=
                    self.level.arrow.direction * self.level.arrow.speed * delta;
            }
            _ => {}
        }
    }

    fn update_camera(&mut self) {
        self.camera.zoom = vec2(0.01, 0.01 * screen_width() / screen_height());
    }

    fn get_player_aim(&self) -> Vec2 {
        let (mouse_x, mouse_y) = mouse_position();
        let aim = self.camera.screen_to_world(vec2(mouse_x, mouse_y));
        vec2(aim.x.max(0.0), aim.y)
    }
}
