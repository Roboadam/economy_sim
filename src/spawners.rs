use macroquad::prelude::get_screen_data;
use rand::Rng;
use rand_chacha::ChaCha8Rng;

use crate::{components::*, world::W};

pub fn spawn_ai_people(num: i32, world: &mut World, rng: &mut ChaCha8Rng) {
    let screen_data = get_screen_data();
    let x_max = screen_data.width() as f32;
    let y_max = screen_data.height() as f32;
    for position in random_positions(num, x_max, y_max, rng) {
        world.spawn((AiPersonTag, Hunger(100.), position, TravelingTo::Nowhere));
    }
}

pub fn spawn_businesses(num: i32, sprite: usize, w: &mut W, rng: &mut ChaCha8Rng) {
    let screen_data = get_screen_data();
    let x_max = screen_data.width() as f32;
    let y_max = screen_data.height() as f32;

    for position in random_positions(num, x_max, y_max, rng) {
        w.add_business_entity(sprite, position);
    }
}

pub fn spawn_homes(num: i32, sprite: usize, w: &mut W, rng: &mut ChaCha8Rng) {
    let screen_data = get_screen_data();
    let x_max = screen_data.width() as f32;
    let y_max = screen_data.height() as f32;

    for position in random_positions(num, x_max, y_max, rng) {
        w.add_home_entity(sprite, position);
    }
}

fn random_positions(num: i32, x_max: f32, y_max: f32, rng: &mut ChaCha8Rng) -> Vec<Position> {
    (0..num)
        .into_iter()
        .map(|_i| {
            let x = rng.gen_range(0f32..x_max);
            let y = rng.gen_range(0f32..y_max);
            Position { x, y }
        })
        .collect()
}
