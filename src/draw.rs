use std::f32::consts::PI;

use macroquad::{
    color::{Color, BLACK, WHITE}, input::mouse_position, math::{vec2, Rect, Vec2}, miniquad::window::screen_size, shapes::{draw_circle, draw_rectangle, draw_rectangle_lines}, text::{draw_text, draw_text_ex, measure_text, TextParams}, texture::{draw_texture_ex, DrawTextureParams, Texture2D}, window::clear_background
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

pub fn draw_medals(resource_manager: &ResourceManager, completed_levels: &[i32], total_levels: usize) {
    const REQUIRED_AVERAGE: i32 = 80;
    const X_REL: f32 = 0.01;
    const Y_BOTTOM: f32 = 0.01;
    const MEDAL_SIZE: f32 = 0.2;
    const MEDAL_RATIO: f32 = 0.5;
    const FONT_SIZE: f32 = 0.05;
    let average_accuracy = completed_levels.iter().sum::<i32>() / completed_levels.len() as i32;
    let bronze_medal_text = ["Bronze Medal", "Complete all levels"];
    let silver_medal_text_owned = [
        "Silver Medal".to_owned(),
        format!("Complete all levels with an average of {REQUIRED_AVERAGE}% accuracy"),
        format!("Current: {average_accuracy}%")
    ];
    let silver_medal_text = silver_medal_text_owned.iter()
        .map(|s| s.as_ref())
        .collect::<Vec<_>>();
    let gold_medal_text = ["Gold Medal", "Complete all levels with 100% accuracy"];

    let medals_count = if completed_levels.len() != total_levels {
        0
    } else if completed_levels.iter().all(|acc| *acc == 100) {
        3
    } else if completed_levels.iter().sum::<i32>() / total_levels as i32 >= REQUIRED_AVERAGE {
        2
    } else {
        1
    };

    let (width, height) = screen_size();
    let size_y = height * MEDAL_SIZE;
    let size_x = size_y * MEDAL_RATIO;
    let margin = size_y * 0.2;
    let y = (height - Y_BOTTOM * height) - size_y;
    let x_start = X_REL * width;
    let font_size = FONT_SIZE * height;

    let rect_bronze_medal = Rect::new(x_start, y, size_x, size_y);
    draw_medal(&resource_manager.medals[0], rect_bronze_medal, medals_count >= 1);

    let rect_silver_medal = Rect::new(x_start + size_x + margin, y, size_x, size_y);
    draw_medal(&resource_manager.medals[1], rect_silver_medal, medals_count >= 2);

    let rect_gold_medal = Rect::new(x_start + 2.0*(size_x + margin), y, size_x, size_y);
    draw_medal(&resource_manager.medals[2], rect_gold_medal, medals_count >= 3);

    if draw_medal_text(rect_bronze_medal, font_size, &bronze_medal_text) {
        return;
    }
    if draw_medal_text(rect_silver_medal, font_size, &silver_medal_text) {
        return;
    }
    if draw_medal_text(rect_gold_medal, font_size, &gold_medal_text) {
        return;
    }
}

fn draw_medal(texture: &Texture2D, rect: Rect, active: bool) {
    //draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1.0, WHITE);
    let color = if active {
        WHITE
    } else {
        Color::from_rgba(30, 30, 30, 255)
    };
    draw_texture_ex(texture, rect.x, rect.y, color, DrawTextureParams { dest_size: Some(vec2(rect.w, rect.h)), ..Default::default()});
}

/// returns true if drawn
fn draw_medal_text(medal_rect: Rect, font_size: f32, hover_text: &[&str]) -> bool {
    let (mouse_x, mouse_y) = mouse_position();
    let should_draw_text = medal_rect.contains(vec2(mouse_x, mouse_y));

    if should_draw_text {
        draw_text_box(mouse_x, mouse_y, font_size, hover_text);
    }

    should_draw_text
}

fn draw_text_box(x: f32, y: f32, font_size: f32, text: &[&str]) {
    const MARGIN: f32 = 0.25;
    if text.is_empty() {
        return;
    }
    let width = text.iter()
        .map(|line| measure_text(line, None, font_size as u16, 1.0).width.ceil() as i32)
        .max()
        .unwrap_or(0) as f32;
    let text_height = measure_text(text[0], None, font_size as u16, 1.0).height;
    let margin = text_height * MARGIN;
    let height = (text_height + margin) * text.len() as f32;
    let y_start = y - height;

    draw_rectangle(x, y_start - text_height, width, height, Color::from_rgba(0, 0, 0, 200));
    for (i, line) in text.iter().enumerate() {
        let current_y = y_start + i as f32 * (text_height + margin);
        draw_text(line, x, current_y, font_size, WHITE);
    }
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
