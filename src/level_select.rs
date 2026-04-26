use macroquad::{
    audio::play_sound_once,
    camera::set_default_camera,
    color::{Color, WHITE},
    input::{MouseButton, is_mouse_button_released, mouse_position},
    math::{Rect, vec2},
    miniquad::window::screen_size,
    prelude::error,
    shapes::{draw_rectangle, draw_rectangle_lines},
    text::{TextParams, draw_text, draw_text_ex, measure_text},
};
use std::fmt::Write;

use crate::{
    draw::{accuracy_to_int, draw_background},
    model::LevelTemplate,
    resource_manager::ResourceManager,
};

#[derive(Debug)]
pub struct LevelSelection {
    pub is_in_menu: bool,
    page: usize,
    level_count: usize,
    completed: Vec<f32>,
}
impl LevelSelection {
    const ROWS: usize = 3;
    const COLUMNS: usize = 8;
    const LEVELS_PER_PAGE: usize = Self::ROWS * Self::COLUMNS;
    const BUTTON_OFFSET: f32 = 10.0;
    const BUTTON_SIZE: f32 = 0.09;
    const Y_POS: f32 = 0.25;
    const BG_BRIGHTNESS: f32 = 0.4;

    pub fn new(levels: &[LevelTemplate]) -> Self {
        Self {
            is_in_menu: true,
            page: 0,
            level_count: levels.len(),
            completed: Vec::with_capacity(levels.len()),
        }
    }

    pub fn add_completed(&mut self, level: usize, accuracy: f32) {
        assert!(level <= self.completed.len());
        if level < self.completed.len() {
            self.completed[level] = self.completed[level].max(accuracy)
        } else {
            self.completed.push(accuracy);
        }
    }

    /// returns selected level
    pub fn draw_level_selection(&mut self, resource_manager: &ResourceManager) -> Option<usize> {
        set_default_camera();
        draw_background(resource_manager, Self::BG_BRIGHTNESS);
        let (width, height) = screen_size();
        Self::draw_title(width, height);

        let mut main_text_buffer = String::with_capacity(2);
        let mut subtext_buffer = String::with_capacity(4);
        let mut selected = None;
        let range = (self.page * Self::LEVELS_PER_PAGE)
            ..((self.page + 1) * Self::LEVELS_PER_PAGE).min(self.level_count);
        let button_size = height * Self::BUTTON_SIZE;
        let x_start = (width
            - Self::COLUMNS as f32 * button_size
            - Self::BUTTON_OFFSET * (Self::COLUMNS - 1) as f32)
            / 2.0;
        let y_start = Self::Y_POS * height;

        for (index, level) in range.enumerate() {
            let y = y_start + (index / Self::COLUMNS) as f32 * (button_size + Self::BUTTON_OFFSET);
            let x = x_start + (index % Self::COLUMNS) as f32 * (button_size + Self::BUTTON_OFFSET);
            main_text_buffer.clear();
            subtext_buffer.clear();
            let res = write!(&mut main_text_buffer, "{}", level + 1);
            if let Err(err) = res {
                error!("Error writing to buffer: {}", err);
            }
            let button_rect = Rect::new(x, y, button_size, button_size);
            if self.completed.len() < index {
                draw_unselectable_button(button_rect, &main_text_buffer);
                continue;
            }

            if let Some(accuracy) = self.completed.get(index) {
                let accuracy_percent = accuracy_to_int(*accuracy);
                write!(&mut subtext_buffer, "{}%", accuracy_percent)
                    .expect("Error writing to subtext buffer");
            }
            if draw_button(
                resource_manager,
                button_rect,
                &main_text_buffer,
                &subtext_buffer,
            ) {
                self.is_in_menu = false;
                selected = Some(level);
            }
        }

        self.draw_nav_buttons(resource_manager, button_size, x_start, y_start);

        selected
    }

    fn draw_nav_buttons(
        &mut self,
        resource_manager: &ResourceManager,
        button_size: f32,
        x_start: f32,
        y_start: f32,
    ) {
        const NAV_BUTTON_OFFSET: f32 = 20.0;
        let nav_buttons_y =
            y_start + Self::ROWS as f32 * (button_size + Self::BUTTON_OFFSET) + NAV_BUTTON_OFFSET;
        let x_back = x_start;
        let mut page_change = 0;
        if self.page > 0 {
            if draw_button(
                resource_manager,
                Rect::new(x_back, nav_buttons_y, button_size, button_size),
                "<",
                "",
            ) {
                page_change = -1;
            }
        }

        let max_page = (self.level_count - 1) / Self::LEVELS_PER_PAGE;
        let x_next = x_start + (Self::COLUMNS - 1) as f32 * (button_size + Self::BUTTON_OFFSET);
        if self.page < max_page {
            if draw_button(
                resource_manager,
                Rect::new(x_next, nav_buttons_y, button_size, button_size),
                ">",
                "",
            ) {
                page_change = 1;
            }
        }
        self.page = (self.page as i32 + page_change) as usize;
    }

    fn draw_title(width: f32, height: f32) {
        const TITLE: &str = "SPACE ARCHER";
        let y = 0.15 * height;
        let font_size = (0.1 * height) as u16;
        let shadow_offset = 0.005 * height;

        let text_width = measure_text(TITLE, None, font_size, 1.0).width;
        let x = (width - text_width) / 2.0;
        draw_text_ex(
            TITLE,
            x + shadow_offset,
            y + shadow_offset,
            TextParams {
                font_size,
                color: Color::from_rgba(255, 255, 255, 80),
                ..Default::default()
            },
        );
        draw_text_ex(
            TITLE,
            x,
            y,
            TextParams {
                font_size,
                color: WHITE,
                ..Default::default()
            },
        );
    }
}

const BUTTON_BORDER_SIZE: f32 = 2.0;
const BUTTON_TEXT_OFFSET: f32 = 4.0;
const BUTTON_FONT_SIZE: f32 = 0.9;

/// returns true if clicked
fn draw_button(resource_manager: &ResourceManager, size: Rect, text: &str, subtext: &str) -> bool {
    const BUTTON_COLOR: Color = Color::from_rgba(255, 255, 255, 120);
    const BUTTON_COLOR_HOVERED: Color = Color::from_rgba(255, 255, 255, 180);
    const BUTTON_BORDER_COLOR: Color = Color::from_rgba(255, 255, 255, 220);

    let (mouse_x, mouse_y) = mouse_position();
    let is_hovered = size.contains(vec2(mouse_x, mouse_y));
    let button_color = if is_hovered {
        BUTTON_COLOR_HOVERED
    } else {
        BUTTON_COLOR
    };
    let main_font_size = size.h * BUTTON_FONT_SIZE;
    let subtext_font_size = size.h * BUTTON_FONT_SIZE / 2.5;

    draw_rectangle(size.x, size.y, size.w, size.h, button_color);
    draw_rectangle_lines(
        size.x,
        size.y,
        size.w,
        size.h,
        BUTTON_BORDER_SIZE,
        BUTTON_BORDER_COLOR,
    );
    draw_text(
        text,
        size.x + BUTTON_TEXT_OFFSET,
        size.y + BUTTON_TEXT_OFFSET + main_font_size * 0.5,
        main_font_size,
        WHITE,
    );
    draw_text(
        subtext,
        size.x + BUTTON_TEXT_OFFSET,
        size.y + BUTTON_TEXT_OFFSET + main_font_size * 0.5 + subtext_font_size,
        subtext_font_size,
        WHITE,
    );

    let is_clicked = is_hovered && is_mouse_button_released(MouseButton::Left);
    if is_clicked {
        play_sound_once(&resource_manager.sounds.click);
    }

    is_clicked
}

fn draw_unselectable_button(size: Rect, text: &str) {
    let font_size = size.h * BUTTON_FONT_SIZE;
    const COLOR: Color = Color::from_rgba(60, 60, 60, 100);
    const BORDER_COLOR: Color = Color::from_rgba(80, 80, 80, 255);
    draw_rectangle(size.x, size.y, size.w, size.h, COLOR);
    draw_rectangle_lines(
        size.x,
        size.y,
        size.w,
        size.h,
        BUTTON_BORDER_SIZE,
        BORDER_COLOR,
    );
    draw_text(
        text,
        size.x + BUTTON_TEXT_OFFSET,
        size.y + BUTTON_TEXT_OFFSET + font_size * 0.5,
        font_size,
        WHITE,
    );
}
