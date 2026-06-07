use macroquad::{
    audio::play_sound_once,
    camera::{Camera2D, set_camera, set_default_camera},
    input::{
        KeyCode, MouseButton, is_key_released, is_mouse_button_down, is_mouse_button_released,
        mouse_position,
    },
    math::{Rect, Vec2, vec2},
    prelude::{error, info},
    window::{screen_height, screen_width},
};

use crate::{
    draw::{
        accuracy_to_int, draw_arrow, draw_background, draw_barier, draw_bow,
        draw_current_level_number, draw_effect, draw_future_arrow_movements, draw_key,
        draw_miss_text, draw_planet, draw_target, draw_ufo, draw_win_text,
    },
    level_select::LevelSelection,
    model::{Arrow, ArrowState, Bow, Effect, Level, LevelTemplate, TargetFlip},
    physics::{
        arrow_has_hit_barrier, calculate_static_movement, move_arrow,
        simulate_future_arrow_movement,
    },
    resource_manager::ResourceManager,
    ui::draw_button,
};

pub struct Game<'a> {
    resource_manager: &'a ResourceManager,
    levels: &'a [LevelTemplate],
    current_level_index: usize,
    level: Level<'a>,
    camera: Camera2D,
    should_exit: bool,
}
impl<'a> Game<'a> {
    const GAME_BOUNDARY: Rect = Rect::new(-60.0, -100.0, 300.0, 220.0);
    const MAX_ARROW_FLIGHT_TIME_S: f32 = 10.0;

    pub fn new(
        resource_manager: &'a ResourceManager,
        levels: &'a [LevelTemplate],
        current_level_index: usize,
    ) -> Self {
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
            should_exit: false,
            resource_manager,
        }
    }

    pub fn draw(&mut self) {
        const BG_BRIGHTNESS: f32 = 0.8;
        set_default_camera();
        draw_background(self.resource_manager, BG_BRIGHTNESS);
        set_camera(&self.camera);

        let should_draw_future_movements = self.level.arrow.state == ArrowState::Unfired
            && self.level.bow.strength > Bow::MAX_STRENGTH * 0.1;
        if should_draw_future_movements {
            let future_movements = simulate_future_arrow_movement(
                self.level.arrow,
                &self.level.planets,
                &self.level.ufos,
                &self.level.bariers,
                &self.level.bow,
                12,
            );
            draw_future_arrow_movements(&future_movements);
        }
        for effect in &self.level.effects {
            draw_effect(effect, self.resource_manager);
        }
        draw_target(&self.level.target, self.resource_manager);
        draw_arrow(&self.level.arrow, self.resource_manager);
        draw_bow(&self.level.bow, self.resource_manager);
        for p in &self.level.planets {
            draw_planet(p, self.resource_manager);
        }
        for b in &self.level.bariers {
            draw_barier(b, self.level.time);
        }
        for k in &self.level.keys {
            draw_key(k, self.level.time, self.resource_manager);
        }
        for u in &self.level.ufos {
            draw_ufo(u, self.resource_manager);
        }

        set_default_camera();
        match self.level.arrow.state {
            ArrowState::Hit => draw_win_text(self.resource_manager, self.level.accuracy),
            ArrowState::Missed => draw_miss_text(self.resource_manager),
            _ => {}
        }
        self.handle_in_game_buttons();
        draw_current_level_number(self.resource_manager, self.current_level_index);
    }

    pub fn update(&mut self, delta: f32, level_selection: &mut LevelSelection) {
        self.update_camera();
        set_camera(&self.camera);
        if is_key_released(KeyCode::M) {
            self.should_exit = true;
        }

        let player_aim = self.get_player_aim();
        if !matches!(self.level.arrow.state, ArrowState::Missed | ArrowState::Hit) {
            self.level.time += delta;
            self.update_static_movement(delta);
            self.update_arrow(delta, player_aim, level_selection);
            self.update_effects(delta);
        }
        if is_key_released(KeyCode::R) || is_key_released(KeyCode::Escape) {
            self.reset_level();
        }
    }

    fn update_effects(&mut self, delta: f32) {
        for effect in &mut self.level.effects {
            effect.life -= delta;
        }

        self.level.effects.retain(|eff| eff.life > 0.0);
    }

    pub fn set_level(&mut self, new_level: usize) {
        debug_assert!(new_level < self.levels.len());
        self.current_level_index = new_level;
        self.reset_level();
    }

    fn update_static_movement(&mut self, delta: f32) {
        let tracks = self
            .level
            .planets
            .iter_mut()
            .map(|x| &mut x.track)
            .chain(self.level.bariers.iter_mut().map(|x| &mut x.track))
            .chain(self.level.ufos.iter_mut().map(|x| &mut x.track))
            .chain(self.level.keys.iter_mut().map(|x| &mut x.track))
            .chain(vec![&mut self.level.target.track]);

        for track in tracks {
            calculate_static_movement(track, delta);
        }
    }

    fn reset_level(&mut self) {
        info!("Reset level {}", self.current_level_index);
        self.level = self.levels[self.current_level_index].instance();
    }

    pub fn should_exit(&mut self) -> bool {
        if self.should_exit {
            self.should_exit = false;
            true
        } else {
            false
        }
    }

    fn next_level(&mut self) {
        if self.current_level_index + 1 == self.levels.len() {
            self.should_exit = true;
            return;
        }
        self.current_level_index += 1;
        self.level = self.levels[self.current_level_index].instance();
        info!("New level {}", self.current_level_index);
    }

    fn update_arrow(&mut self, delta: f32, aim: Vec2, level_selection: &mut LevelSelection) {
        match self.level.arrow.state {
            ArrowState::Unfired => {
                self.level.bow.direction = (aim - Bow::LOCATION).normalize_or(vec2(1.0, 0.0));
                self.level.arrow.velocity = self.level.bow.direction;
                self.level.arrow.position = self.compute_arrow_position_unfired();
                if is_mouse_button_down(macroquad::input::MouseButton::Left) {
                    const BOW_PULL_SPEED: f32 = 70.0;
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
                const DISTANCE_FOR_TRAIL: f32 = 8.0;
                self.level.arrow.flight_time_s += delta;
                let start_location = self.level.arrow.position;
                move_arrow(
                    &mut self.level.arrow,
                    &self.level.planets,
                    &self.level.ufos,
                    delta,
                );
                let end_location = self.level.arrow.position;
                self.level.arrow.flight_distance_before_trail +=
                    start_location.distance(end_location);
                if self.level.arrow.flight_distance_before_trail >= DISTANCE_FOR_TRAIL {
                    self.level.arrow.flight_distance_before_trail -= DISTANCE_FOR_TRAIL;
                    self.level
                        .effects
                        .push(Effect::new_trail(start_location, end_location));
                }
                if self.arrow_has_missed() {
                    info!("Missed, location: {}", self.level.arrow.position);
                    self.level.arrow.state = ArrowState::Missed;
                }
                self.detect_arrow_hit_target(level_selection);
                self.detect_arrow_hit_key();
            }
            _ => {}
        }
    }

    fn compute_arrow_position_unfired(&self) -> Vec2 {
        const STRENGTH_MOD: f32 = Bow::MAX_STRENGTH / 1800.0;
        const BASE_ARROW_POSITION_X: f32 = Arrow::SIZE * 0.95;
        self.level.bow.direction * (BASE_ARROW_POSITION_X - self.level.bow.strength * STRENGTH_MOD)
    }

    fn arrow_has_missed(&self) -> bool {
        if self.level.arrow.flight_time_s > Self::MAX_ARROW_FLIGHT_TIME_S
            || !Self::GAME_BOUNDARY.contains(self.level.arrow.position)
        {
            return true;
        }

        if arrow_has_hit_barrier(&self.level.arrow, &self.level.planets, &self.level.bariers) {
            play_sound_once(&self.resource_manager.sounds.hit);
            return true;
        }

        false
    }

    fn detect_arrow_hit_key(&mut self) {
        let mut hit_key_index = None;
        for (index, key) in self.level.keys.iter().enumerate() {
            if !key.bounding_box().contains(self.level.arrow.position) {
                continue;
            }

            hit_key_index = Some(index);
            break;
        }

        if let Some(index) = hit_key_index {
            let key_pos = self.level.keys[index].track.position;
            self.level.effects.push(Effect::new_key_pickup(key_pos));
            self.level.keys.remove(index);
            if self.level.keys.is_empty() {
                self.remove_locked_bariers();
            }
            self.level.arrow = Arrow::default();
            play_sound_once(&self.resource_manager.sounds.pick_up_key);
        }
    }

    fn remove_locked_bariers(&mut self) {
        self.level.bariers.retain(|b| !b.locked);
    }

    fn detect_arrow_hit_target(&mut self, level_selection: &mut LevelSelection) {
        if self
            .level
            .target
            .bounding_box()
            .contains(self.level.arrow.position)
        {
            self.determine_accuracy();
            level_selection.add_completed(
                self.current_level_index,
                accuracy_to_int(self.level.accuracy),
            );
            self.level.arrow.state = ArrowState::Hit;
            play_sound_once(&self.resource_manager.sounds.hit);
            info!(
                "Hit, location: {}, accuracy: {}",
                self.level.arrow.position, self.level.accuracy
            );
        }
    }

    fn determine_accuracy(&mut self) {
        const EXTRA_ACCURACY: f32 = 0.05;
        let target_bb = &self.level.target.bounding_box();
        let arrow_pos = self.level.arrow.position;
        self.level.accuracy = if self.level.target.template.flipped == TargetFlip::Right {
            (target_bb.h - (target_bb.h / 2.0 + target_bb.y - arrow_pos.y).abs() * 2.0)
                / target_bb.h
        } else {
            (target_bb.w - (target_bb.w / 2.0 + target_bb.x - arrow_pos.x).abs() * 2.0)
                / target_bb.w
        };
        if self.level.accuracy < 0.0 || self.level.accuracy > 1.0 {
            error!("Invalid accuracy: {}", self.level.accuracy);
        }
        self.level.accuracy = (self.level.accuracy + EXTRA_ACCURACY).clamp(0.0, 1.0);
    }

    fn update_camera(&mut self) {
        const ZOOM: f32 = 0.008;
        self.camera.zoom = vec2(ZOOM, ZOOM * screen_width() / screen_height());
    }

    fn get_player_aim(&self) -> Vec2 {
        const MIN_DISTANCE: f32 = 30.0;
        let (mouse_x, mouse_y) = mouse_position();
        let mut aim = self.camera.screen_to_world(vec2(mouse_x, mouse_y));
        aim.x = aim.x.max(Bow::LOCATION.x);
        let dist_to_bow = Bow::LOCATION.distance(aim);

        if dist_to_bow <= f32::EPSILON {
            return vec2(100.0, 0.0);
        }
        if dist_to_bow >= MIN_DISTANCE {
            return aim;
        }

        Bow::LOCATION + aim * (MIN_DISTANCE / dist_to_bow)
    }

    fn handle_in_game_buttons(&mut self) {
        const MARGIN: f32 = 10.0;
        const BUTTON_RELATIVE_SIZE: f32 = 0.1;
        const RESET_BUTTON_RELATIVE_SIZE_TO_BACK: f32 = 2.5;
        let back_size = BUTTON_RELATIVE_SIZE * screen_height();
        if draw_button(
            self.resource_manager,
            Rect::new(MARGIN, MARGIN, back_size, back_size),
            "<",
            "",
        ) {
            self.should_exit = true;
            return;
        }

        let reset_button_width = RESET_BUTTON_RELATIVE_SIZE_TO_BACK * back_size;
        let should_draw_reset_button = (self.level.arrow.flight_time_s > 3.0
            && self.level.arrow.state == ArrowState::Moving)
            || matches!(self.level.arrow.state, ArrowState::Missed | ArrowState::Hit);
        if should_draw_reset_button
            && draw_button(
                self.resource_manager,
                Rect::new(
                    2.0 * MARGIN + back_size,
                    MARGIN,
                    reset_button_width,
                    back_size,
                ),
                "Reset",
                " (Press R)",
            )
        {
            self.reset_level();
            return;
        }

        if is_mouse_button_released(MouseButton::Left) {
            match self.level.arrow.state {
                ArrowState::Missed => self.reset_level(),
                ArrowState::Hit => self.next_level(),
                _ => {}
            }
        }
    }
}
