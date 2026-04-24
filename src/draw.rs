use std::f32::consts::PI;

use macroquad::{
    color::WHITE,
    math::{vec2, Vec2},
    shapes::{draw_circle, draw_rectangle},
    texture::{draw_texture_ex, DrawTextureParams},
};

use crate::{
    model::{Arrow, Bow, Planet, Target},
    resource_manager::ResourceManager,
};

pub fn draw_future_arrow_movements(positions: &[Vec2]) {
    const DOT_SIZE: f32 = 0.2;
    for pos in positions {
        draw_circle(pos.x, pos.y, DOT_SIZE, WHITE);
    }
}

pub fn draw_arrow(arrow: &Arrow, resource_manager: &ResourceManager) {
    let arrow_direction =  arrow.velocity.normalize_or_zero();
    let rotation = PI/2.0 - (arrow_direction.x).atan2(arrow_direction.y);
    let center = arrow.position - arrow_direction * (Arrow::SIZE / 2.0);
    let top_left = center - vec2(Arrow::SIZE/ 2.0, Arrow::SIZE/ 2.0);

    draw_texture_ex(
        &resource_manager.arrow,
        top_left.x,
        top_left.y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(Arrow::SIZE, Arrow::SIZE)),
            rotation,
            ..Default::default()
        });
}

pub fn draw_bow(bow: &Bow, resource_manager: &ResourceManager) {
    let image_index = ((resource_manager.bow.len() as f32 * (bow.strength / Bow::MAX_STRENGTH))
        .floor() as usize)
        .min(resource_manager.bow.len() - 1);
    let texture = &resource_manager.bow[image_index];
    let rotation = bow.direction.y.asin();
    draw_texture_ex(
        texture,
        -Bow::SIZE / 2.0,
        -Bow::SIZE / 2.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(Bow::SIZE, Bow::SIZE)),
            rotation,
            ..Default::default()
        },
    );
}

pub fn draw_target(target: &Target) {
    let bb = target.bounding_box();
    draw_rectangle(bb.x, bb.y, bb.w, bb.h, WHITE);
}

pub fn draw_planet(planet: &Planet, resource_manager: &ResourceManager) {
    let top_left = planet.track.position - Vec2::splat(planet.size);

    draw_texture_ex(
        &resource_manager.planets[0],
        top_left.x,
        top_left.y,
        WHITE,
     DrawTextureParams { dest_size: Some(Vec2::splat(planet.size*2.0)), ..Default::default() }
    );
}
