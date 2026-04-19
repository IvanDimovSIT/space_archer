use macroquad::{
    camera::Camera2D,
    color::WHITE,
    math::{Vec2, vec2},
    prelude::camera,
    shapes::{draw_circle, draw_line},
    texture::{DrawTextureParams, draw_texture_ex},
};

use crate::{
    model::{Arrow, Bow},
    resource_manager::{self, ResourceManager},
};

pub fn draw_arrow(arrow: &Arrow) {
    const ARROW_SIZE: f32 = 2.0;
    draw_circle(arrow.position.x, arrow.position.y, ARROW_SIZE, WHITE);
    let arrow_tail = arrow.position - arrow.direction * 10.0;

    draw_line(
        arrow.position.x,
        arrow.position.y,
        arrow_tail.x,
        arrow_tail.y,
        1.0,
        WHITE,
    );
}

pub fn draw_bow(bow: &Bow, resource_manager: &ResourceManager) {
    const BOW_SIZE: Vec2 = vec2(20.0, 20.0);

    let image_index =
        (resource_manager.bow.len() as f32 * (bow.strength / Bow::MAX_STRENGTH)).floor() as usize;
    let texture = &resource_manager.bow[image_index];
    let rotation = bow.direction.y.asin();
    draw_texture_ex(
        texture,
        -BOW_SIZE.x / 2.0,
        -BOW_SIZE.y / 2.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(BOW_SIZE),
            rotation,
            ..Default::default()
        },
    );
}
