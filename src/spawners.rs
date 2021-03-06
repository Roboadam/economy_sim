use macroquad::prelude::get_screen_data;
use rand::Rng;
use rand_chacha::ChaCha8Rng;

use crate::{ai_person::AiPerson, building::Building, components::*, traits::*};

pub fn spawn_ai_people(num: i32, texture_index: usize, rng: &mut ChaCha8Rng) -> Vec<AiPerson> {
    let screen_data = get_screen_data();
    let x_max = screen_data.width() as f32;
    let y_max = screen_data.height() as f32;
    let mut result = Vec::new();
    for position in random_positions(num, x_max, y_max, rng) {
        let mut ai_person = AiPerson::default();
        ai_person.set_position(&position);
        ai_person.set_texture_index(texture_index);
        result.push(ai_person);
    }
    result
}

pub fn spawn_buildings(num: i32, texture_index: usize, rng: &mut ChaCha8Rng) -> Vec<Building> {
    let screen_data = get_screen_data();
    let x_max = screen_data.width() as f32;
    let y_max = screen_data.height() as f32;
    let mut result = Vec::new();
    for position in random_positions(num, x_max, y_max, rng) {
        let mut ai_person = Building::default();
        ai_person.set_position(&position);
        ai_person.set_texture_index(texture_index);
        result.push(ai_person);
    }
    result
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
