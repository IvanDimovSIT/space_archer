use macroquad::{
    audio::play_sound_once,
    color::{Color, WHITE},
    input::{MouseButton, is_mouse_button_released, mouse_position},
    math::{Rect, vec2},
    shapes::{draw_rectangle, draw_rectangle_lines},
    text::draw_text,
};

use crate::resource_manager::ResourceManager;

const BUTTON_BORDER_SIZE: f32 = 2.0;
const BUTTON_TEXT_OFFSET: f32 = 4.0;
const BUTTON_FONT_SIZE: f32 = 0.9;

/// returns true if clicked
pub fn draw_button(
    resource_manager: &ResourceManager,
    size: Rect,
    text: &str,
    subtext: &str,
) -> bool {
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

pub fn draw_unselectable_button(size: Rect, text: &str) {
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
