use std::f32::consts::PI;

use macroquad::{
    color::{BLACK, Color, WHITE},
    math::{Vec2, vec2},
    miniquad::window::screen_size,
    shapes::{draw_circle, draw_rectangle, draw_rectangle_lines},
    text::{TextParams, draw_text_ex, measure_text},
    texture::{DrawTextureParams, draw_texture_ex},
    window::clear_background,
};

use crate::{
    model::{Arrow, Barier, Bow, Planet, Target, TargetFlip, UFO, UFOTemplate},
    resource_manager::ResourceManager,
};

pub fn draw_future_arrow_movements(positions: &[Vec2]) {
    const DOT_SIZE: f32 = 0.2;
    for pos in positions {
        draw_circle(pos.x, pos.y, DOT_SIZE, WHITE);
    }
}

pub fn draw_arrow(arrow: &Arrow, resource_manager: &ResourceManager) {
    let arrow_direction = arrow.velocity.normalize_or_zero();
    let rotation = PI / 2.0 - (arrow_direction.x).atan2(arrow_direction.y);
    let center = arrow.position - arrow_direction * (Arrow::SIZE / 2.0);
    let top_left = center - vec2(Arrow::SIZE / 2.0, Arrow::SIZE / 2.0);

    draw_texture_ex(
        &resource_manager.arrow,
        top_left.x,
        top_left.y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(Arrow::SIZE, Arrow::SIZE)),
            rotation,
            ..Default::default()
        },
    );
}

pub fn draw_bow(bow: &Bow, resource_manager: &ResourceManager) {
    let image_index = ((resource_manager.bow.len() as f32 * (bow.strength / Bow::MAX_STRENGTH))
        .floor() as usize)
        .min(resource_manager.bow.len() - 1);
    let texture = &resource_manager.bow[image_index];
    let rotation = bow.direction.y.asin();
    draw_texture_ex(
        texture,
        Bow::LOCATION.x - Bow::SIZE / 2.0,
        Bow::LOCATION.y - Bow::SIZE / 2.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(Bow::SIZE, Bow::SIZE)),
            rotation,
            ..Default::default()
        },
    );
}

pub fn draw_target(target: &Target, resource_manager: &ResourceManager) {
    let bb = target.bounding_box();
    let dest_size = Some(vec2(bb.w, bb.h));
    match target.template.flipped {
        TargetFlip::Right => {
            draw_texture_ex(
                &resource_manager.target,
                bb.x,
                bb.y,
                WHITE,
                DrawTextureParams {
                    dest_size,
                    ..Default::default()
                },
            );
        }
        TargetFlip::Top | TargetFlip::Bottom => {
            let flip_x = target.template.flipped == TargetFlip::Top;
            draw_texture_ex(
                &resource_manager.target,
                bb.x + bb.w / 4.0,
                bb.y - bb.h / 2.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(bb.h, bb.w)),
                    flip_x,
                    rotation: PI / 2.0,
                    ..Default::default()
                },
            );
        }
    };
    // draw_circle(bb.x, bb.y, 1.0, RED);
    // draw_rectangle(bb.x, bb.y, bb.w, bb.h, Color::from_rgba(255, 255, 255, 80));
}

pub fn draw_planet(planet: &Planet, resource_manager: &ResourceManager) {
    let top_left = planet.track.position - Vec2::splat(planet.size);
    let texture = resource_manager.get_planet_texture(planet.appearance);

    draw_texture_ex(
        texture,
        top_left.x,
        top_left.y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2::splat(planet.size * 2.0)),
            ..Default::default()
        },
    );
}

pub fn draw_background(resource_manager: &ResourceManager, brightness: f32) {
    let (width, height) = screen_size();
    clear_background(BLACK);
    draw_texture_ex(
        &resource_manager.background,
        0.0,
        0.0,
        Color::new(1.0, 1.0, 1.0, brightness.clamp(0.0, 1.0)),
        DrawTextureParams {
            dest_size: Some(vec2(width, height)),
            ..Default::default()
        },
    );
}

pub fn accuracy_to_int(accuracy: f32) -> i32 {
    (accuracy * 100.0).ceil() as i32
}

pub fn draw_win_text(accuracy: f32) {
    let display_accuracy = accuracy_to_int(accuracy);
    let text = format!("HIT! ACCURACY: {}%", display_accuracy);
    draw_centered_in_game_text(&text);
}

pub fn draw_miss_text() {
    draw_centered_in_game_text("MISSED!");
}

pub fn draw_barier(barier: &Barier, time: f32) {
    const BARIER_BASE_COLOR: Color = Color::from_rgba(130, 140, 255, 0);
    const LINES_THICKNESS: f32 = 2.0;
    const MAX_BRIGHTNESS: f32 = 0.9;
    const MIN_BRIGHTNESS: f32 = 0.7;
    const CHANGE_BRIGHTNESS_SPEED: f32 = 2.0;

    let rect = barier.get_rect();
    let time_sin = (time * CHANGE_BRIGHTNESS_SPEED).sin();
    let barier_brightness = time_sin * MAX_BRIGHTNESS + (1.0 - time_sin) * MIN_BRIGHTNESS;
    let color = Color {
        a: barier_brightness,
        ..BARIER_BASE_COLOR
    };

    draw_rectangle(rect.x, rect.y, rect.w, rect.h, color);
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, LINES_THICKNESS, WHITE);
}

pub fn draw_ufo(ufo: &UFO, resource_manager: &ResourceManager) {
    draw_texture_ex(
        &resource_manager.ufo,
        ufo.track.position.x,
        ufo.track.position.y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(UFOTemplate::UFO_SIZE),
            ..Default::default()
        },
    );

    let field_bb = ufo.field_bb();
    draw_texture_ex(
        &resource_manager.ufo_field,
        field_bb.x,
        field_bb.y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(field_bb.w, field_bb.h)),
            ..Default::default()
        },
    );

    //draw_rectangle_lines(field_bb.x, field_bb.y, field_bb.w, field_bb.h, 2.0, WHITE);
}

fn draw_centered_in_game_text(text: &str) {
    const Y: f32 = 0.25;
    const FONT_SIZE: f32 = 0.08;
    const SHADOW_OFFSET: f32 = 0.005;
    const MARGIN: f32 = 0.01;
    let (width, height) = screen_size();
    let font_size = (height * FONT_SIZE) as u16;
    let shadow_offset = height * SHADOW_OFFSET;
    let y = height * Y;
    let text_size = measure_text(text, None, font_size, 1.0);
    let x = (width - text_size.width) / 2.0;

    let rect_margin = MARGIN * height;
    let rect_x = x - rect_margin;
    let rect_y = y - rect_margin - text_size.height;
    let rect_w = text_size.width + 2.0 * rect_margin;
    let rect_h = text_size.height + 2.0 * rect_margin;
    draw_rectangle(
        rect_x,
        rect_y,
        rect_w,
        rect_h,
        Color::from_rgba(0, 0, 0, 220),
    );
    draw_text_ex(
        text,
        x + shadow_offset,
        y + shadow_offset,
        TextParams {
            font_size,
            color: Color::from_rgba(255, 255, 255, 60),
            ..Default::default()
        },
    );
    draw_text_ex(
        text,
        x,
        y,
        TextParams {
            font_size,
            color: WHITE,
            ..Default::default()
        },
    );
}
