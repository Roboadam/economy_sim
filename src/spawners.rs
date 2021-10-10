use macroquad::prelude::get_screen_data;
use rand::Rng;
use rand_chacha::ChaCha8Rng;

use crate::{
    ai_person::{AiPerson, HasPosition, IsDrawable},
    components::*,
    world::W,
};

pub fn spawn_ai_people(num: i32, sprite: usize, w: &mut W) {
    let screen_data = get_screen_data();
    let x_max = screen_data.width() as f32;
    let y_max = screen_data.height() as f32;
    for position in random_positions(num, x_max, y_max, &mut w.rng) {
        w.add_ai_person_entity(sprite, position);
    }
}

pub fn spawn_ai_people2(num: i32, texture_index: usize, rng: &mut ChaCha8Rng) -> Vec<AiPerson> {
    let screen_data = get_screen_data();
    let x_max = screen_data.width() as f32;
    let y_max = screen_data.height() as f32;
    let mut result = Vec::new();
    for position in random_positions2(num, x_max, y_max, rng) {
        let mut ai_person = AiPerson::default();
        ai_person.set_position(&position);
        ai_person.set_texture_index(texture_index);
        result.push(ai_person);
    }
    result
}

pub fn spawn_businesses(num: i32, sprite: usize, w: &mut W) {
    let screen_data = get_screen_data();
    let x_max = screen_data.width() as f32;
    let y_max = screen_data.height() as f32;

    for position in random_positions(num, x_max, y_max, &mut w.rng) {
        w.add_business_entity(sprite, position);
    }
}

pub fn spawn_homes(num: i32, sprite: usize, w: &mut W) {
    let screen_data = get_screen_data();
    let x_max = screen_data.width() as f32;
    let y_max = screen_data.height() as f32;

    for position in random_positions(num, x_max, y_max, &mut w.rng) {
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

fn random_positions2(num: i32, x_max: f32, y_max: f32, rng: &mut ChaCha8Rng) -> Vec<Position> {
    vec![
        Position { x: 55., y: 137. },
        Position { x: 0., y: 0. },
        // Position {
        //     x: 0.,
        //     y: y_max - margin,
        // },
        // Position {
        //     x: x_max - margin,
        //     y: 0.,
        // },
        // Position {
        //     x: x_max - margin,
        //     y: y_max - margin,
        // },
    ]
}
