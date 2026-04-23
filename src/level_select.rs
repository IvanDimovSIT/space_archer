use macroquad::{
    camera::set_default_camera,
    color::{BLACK, Color, WHITE},
    input::{MouseButton, is_mouse_button_down, is_mouse_button_released, mouse_position},
    math::{Rect, vec2},
    miniquad::window::screen_size,
    prelude::error,
    shapes::{DrawRectangleParams, draw_rectangle, draw_rectangle_ex, draw_rectangle_lines},
    text::draw_text,
    window::clear_background,
};
use std::fmt::Write;

use crate::{model::LevelTemplate, resource_manager::ResourceManager};

#[derive(Debug)]
pub struct LevelSelection {
    pub is_in_menu: bool,
    page: usize,
    level_count: usize,
}
impl LevelSelection {
    pub fn new(levels: &[LevelTemplate]) -> Self {
        Self {
            is_in_menu: true,
            page: 0,
            level_count: levels.len(),
        }
    }

    /// returns selected level
    pub fn draw_level_selection(&mut self, _resource_manager: &ResourceManager) -> Option<usize> {
        const ROWS: usize = 3;
        const COLUMNS: usize = 8;
        const LEVELS_PER_PAGE: usize = ROWS * COLUMNS;
        const BUTTON_OFFSET: f32 = 10.0;
        const BUTTON_SIZE: f32 = 0.09;
        const Y_POS: f32 = 0.25;
        set_default_camera();
        clear_background(BLACK);
        let (width, height) = screen_size();
        let mut buffer = String::with_capacity(2);
        let mut selected = None;
        let range = (self.page * LEVELS_PER_PAGE)
            ..((self.page + 1) * LEVELS_PER_PAGE).min(self.level_count);
        let button_size = height * BUTTON_SIZE;
        let x_start =
            (width - COLUMNS as f32 * button_size - BUTTON_OFFSET * (COLUMNS - 1) as f32) / 2.0;
        let y_start = Y_POS * height;

        for (index, level) in range.enumerate() {
            let y = y_start + (index / COLUMNS) as f32 * (button_size + BUTTON_OFFSET);
            let x = x_start + (index % COLUMNS) as f32 * (button_size + BUTTON_OFFSET);
            buffer.clear();
            let res = write!(&mut buffer, "{}", level + 1);
            if let Err(err) = res {
                error!("Error writing to buffer: {}", err);
            }
            if draw_button(Rect::new(x, y, button_size, button_size), &buffer) {
                self.is_in_menu = false;
                selected = Some(level);
            }
        }

        const NAV_BUTTON_OFFSET: f32 = 20.0;
        let nav_buttons_y =
            y_start + ROWS as f32 * (button_size + BUTTON_OFFSET) + NAV_BUTTON_OFFSET;
        let x_back = x_start;
        let mut page_change = 0;
        if self.page > 0 {
            if draw_button(
                Rect::new(x_back, nav_buttons_y, button_size, button_size),
                "<",
            ) {
                page_change = -1;
            }
        }

        let max_page = (self.level_count - 1) / LEVELS_PER_PAGE;
        let x_next = x_start + (COLUMNS - 1) as f32 * (button_size + BUTTON_OFFSET);
        if self.page < max_page {
            if draw_button(
                Rect::new(x_next, nav_buttons_y, button_size, button_size),
                ">",
            ) {
                page_change = 1;
            }
        }
        self.page = (self.page as i32 + page_change) as usize;

        selected
    }
}

/// returns true if clicked
fn draw_button(size: Rect, text: &str) -> bool {
    const BUTTON_COLOR: Color = Color::from_rgba(255, 255, 255, 120);
    const BUTTON_COLOR_HOVERED: Color = Color::from_rgba(255, 255, 255, 180);
    const BUTTON_BORDER_COLOR: Color = Color::from_rgba(255, 255, 255, 220);
    const BUTTON_BORDER_SIZE: f32 = 2.0;
    const BUTTON_TEXT_OFFSET: f32 = 4.0;

    let (mouse_x, mouse_y) = mouse_position();
    let is_hovered = size.contains(vec2(mouse_x, mouse_y));
    let button_color = if is_hovered {
        BUTTON_COLOR_HOVERED
    } else {
        BUTTON_COLOR
    };
    let font_size = size.h * 0.9;

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
        size.y + BUTTON_TEXT_OFFSET + font_size * 0.5,
        font_size,
        WHITE,
    );

    is_hovered && is_mouse_button_released(MouseButton::Left)
}
