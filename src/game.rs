use macroquad::{
    camera::{Camera2D, set_camera},
    color::BLACK,
    input::{
        KeyCode, MouseButton, is_key_down, is_mouse_button_down, is_mouse_button_released,
        mouse_position,
    },
    math::{Rect, Vec2, vec2},
    prelude::info,
    window::{clear_background, screen_height, screen_width},
};

use crate::{
    draw::{draw_arrow, draw_bow, draw_future_arrow_movements, draw_planet, draw_target},
    model::{ArrowState, Bow, Level, LevelTemplate, Target},
    physics::{calculate_static_movement, move_arrow, simulate_future_arrow_movement},
    resource_manager::ResourceManager,
};

pub struct Game<'a> {
    levels: &'a [LevelTemplate],
    current_level_index: usize,
    level: Level<'a>,
    camera: Camera2D,
}
impl<'a> Game<'a> {
    const GAME_BOUNDARY: Rect = Rect::new(-60.0, -100.0, 300.0, 220.0);
    const MAX_ARROW_FLIGHT_TIME_S: f32 = 15.0;

    pub fn new(levels: &'a [LevelTemplate], current_level_index: usize) -> Self {
        assert!(!levels.is_empty());
        assert!(current_level_index < levels.len());

        let camera = Camera2D {
            target: vec2(100.0, 0.0),
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
        // draw_rectangle_lines(
        //     Self::GAME_BOUNDARY.x,
        //     Self::GAME_BOUNDARY.y,
        //     Self::GAME_BOUNDARY.w,
        //     Self::GAME_BOUNDARY.h,
        //     15.0,
        //     WHITE,
        // );
        let should_draw_future_movements = self.level.arrow.state == ArrowState::Unfired
            && self.level.bow.strength > Bow::MAX_STRENGTH / 20.0;
        if should_draw_future_movements {
            let future_movements = simulate_future_arrow_movement(
                self.level.arrow,
                &self.level.planets,
                &self.level.bow,
                12,
            );
            draw_future_arrow_movements(&future_movements);
        }
        draw_arrow(&self.level.arrow);
        draw_bow(&self.level.bow, resource_manager);
        draw_target(&self.level.target);
        for p in &self.level.planets {
            draw_planet(p);
        }
    }

    pub fn update(&mut self, delta: f32) {
        self.update_camera();
        set_camera(&self.camera);
        let player_aim = self.get_player_aim();
        if !matches!(self.level.arrow.state, ArrowState::Missed | ArrowState::Hit) {
            self.update_static_movement(delta);
        }
        self.update_arrow(delta, player_aim);
        if self.level.arrow.state != ArrowState::Hit && is_key_down(KeyCode::R) {
            self.reset_level();
        }
        if is_mouse_button_released(MouseButton::Left) {
            match self.level.arrow.state {
                ArrowState::Missed => self.reset_level(),
                ArrowState::Hit => self.next_level(),
                _ => {}
            }
        }
    }

    fn update_static_movement(&mut self, delta: f32) {
        calculate_static_movement(&mut self.level.target.track, delta);
        for planet in &mut self.level.planets {
            calculate_static_movement(&mut planet.track, delta);
        }
    }

    fn reset_level(&mut self) {
        info!("Reset level {}", self.current_level_index);
        self.level = self.levels[self.current_level_index].instance();
    }

    fn next_level(&mut self) {
        self.current_level_index = (self.current_level_index + 1).min(self.levels.len() - 1);
        self.level = self.levels[self.current_level_index].instance();
        info!("New level {}", self.current_level_index);
    }

    fn update_arrow(&mut self, delta: f32, aim: Vec2) {
        match self.level.arrow.state {
            ArrowState::Unfired => {
                self.level.bow.direction =
                    (aim - self.level.arrow.position).normalize_or(vec2(1.0, 0.0));
                self.level.arrow.velocity = self.level.bow.direction;
                if is_mouse_button_down(macroquad::input::MouseButton::Left) {
                    const BOW_PULL_SPEED: f32 = 35.0;
                    self.level.bow.strength = (self.level.bow.strength + BOW_PULL_SPEED * delta)
                        .clamp(0.0, Bow::MAX_STRENGTH);
                } else if is_mouse_button_released(macroquad::input::MouseButton::Left) {
                    const REQUIRED_BOW_STRENGTH: f32 = Bow::MAX_STRENGTH * 0.1;
                    if self.level.bow.strength >= REQUIRED_BOW_STRENGTH {
                        self.level.arrow.state = ArrowState::Moving;
                        self.level.arrow.velocity *= self.level.bow.strength;
                    }
                    self.level.bow.strength = 0.0;
                }
            }
            ArrowState::Moving => {
                self.level.arrow.flight_time_s += delta;
                move_arrow(&mut self.level.arrow, &self.level.planets, delta);
                if self.arrow_has_missed() {
                    info!("Missed, location: {}", self.level.arrow.position);
                    self.level.arrow.state = ArrowState::Missed;
                }
                self.detect_arrow_hit_target();
            }
            _ => {}
        }
    }

    fn arrow_has_missed(&self) -> bool {
        if self.level.arrow.flight_time_s > Self::MAX_ARROW_FLIGHT_TIME_S
            || !Self::GAME_BOUNDARY.contains(self.level.arrow.position)
        {
            return true;
        }

        for planet in &self.level.planets {
            if self.level.arrow.position.distance(planet.track.position) < planet.size {
                return true;
            }
        }

        false
    }

    fn detect_arrow_hit_target(&mut self) {
        if self
            .level
            .target
            .bounding_box()
            .contains(self.level.arrow.position)
        {
            self.determine_accuracy();
            self.level.arrow.state = ArrowState::Hit;
            info!(
                "Hit, location: {}, accuracy: {}",
                self.level.arrow.position, self.level.accuracy
            );
        }
    }

    fn determine_accuracy(&mut self) {
        let target = &self.level.target;
        let arrow_pos = self.level.arrow.position;
        self.level.accuracy = if target.template.flipped {
            (Target::WIDTH - (target.track.position.x - arrow_pos.x).abs() * 2.0) / Target::WIDTH
        } else {
            (Target::HEIGHT - (target.track.position.y - arrow_pos.y).abs() * 2.0) / Target::HEIGHT
        };
    }

    fn update_camera(&mut self) {
        const ZOOM: f32 = 0.008;
        self.camera.zoom = vec2(ZOOM, ZOOM * screen_width() / screen_height());
    }

    fn get_player_aim(&self) -> Vec2 {
        let (mouse_x, mouse_y) = mouse_position();
        let aim = self.camera.screen_to_world(vec2(mouse_x, mouse_y));
        vec2(aim.x.max(0.0), aim.y)
    }
}
