use macroquad::math::Vec2;

use crate::model::{Arrow, Bow, Planet};

#[derive(Debug)]
pub struct Track<'a> {
    pub points: &'a [Vec2],
    pub index: usize,
    pub position: Vec2,
    pub speed: f32,
}

pub fn simulate_future_arrow_movement(
    mut arrow: Arrow,
    planets: &[Planet],
    bow: &Bow,
    samples: u8,
) -> Vec<Vec2> {
    const DELTA: f32 = 1.0 / 60.0;
    const STEPS_PER_SAMPLE: usize = 4;
    arrow.velocity = bow.direction * bow.strength;
    let mut movements = Vec::with_capacity(samples as usize);
    for _ in 0..samples {
        for _ in 0..STEPS_PER_SAMPLE {
            move_arrow(&mut arrow, planets, DELTA);
        }
        movements.push(arrow.position);
    }

    movements
}

pub fn move_arrow(arrow: &mut Arrow, planets: &[Planet], delta: f32) {
    const GRAVITY: f32 = 60_000.0;
    for planet in planets {
        let line_to_planet = planet.track.position - arrow.position;
        let direction_to_planet = line_to_planet.normalize_or_zero();
        let distance_to_planet = line_to_planet.length();
        let gravity_force = delta * GRAVITY / (distance_to_planet * distance_to_planet);
        arrow.velocity += gravity_force * direction_to_planet;
    }

    let delta_pos = arrow.velocity * delta;
    arrow.position += delta_pos;
}

pub fn calculate_static_movement(track: &mut Track, delta: f32) {
    debug_assert!(!track.points.is_empty());
    if track.points.len() <= 1 {
        return;
    }

    debug_assert!(track.speed > 0.001);
    let mut distance_to_travel = track.speed * delta;
    while distance_to_travel > 0.001 {
        let line_to_next_point = track.points[track.index] - track.position;
        let line_to_next_point_length = line_to_next_point.length();
        let current_distance_to_travel = if distance_to_travel > line_to_next_point_length {
            track.index = (track.index + 1) % track.points.len();
            line_to_next_point_length
        } else {
            distance_to_travel
        };
        distance_to_travel -= current_distance_to_travel;
        let movement_direction = line_to_next_point.normalize_or_zero();
        let new_position = track.position + movement_direction * current_distance_to_travel;
        track.position = new_position;
    }
}
