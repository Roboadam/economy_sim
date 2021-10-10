use crate::{components::*, sprites::Sprites, traits::*};
use ::rand::Rng;
use macroquad::prelude::*;
use rand_chacha::ChaCha8Rng;

pub fn draw<T: IsDrawable>(drawable: &T, sprites: &Sprites) {
    let (texture_index, position) = drawable.render_info();
    let texture = sprites.texture(texture_index);
    draw_texture(*texture, position.x, position.y, WHITE);
}

pub fn assign_travel_to_randomly<T: HasTravelingTo, U: HasPosition>(
    tts: &mut Vec<T>,
    dests: &Vec<U>,
    seconds: f32,
    rng: &mut ChaCha8Rng,
) {
    for tt in tts.iter_mut() {
        if tt.traveling_to().is_none() {
            let i = rng.gen_range(0..dests.len());
            let dest = dests.get(i).expect("Random within range").position();
            tt.set_traveling_to(dest, seconds);
        }
    }
}

pub fn travel<T: HasTravelingTo + HasPosition>(movers: &mut Vec<T>, seconds: f32) {
    for m in movers.iter_mut() {
        if let Some(to_position) = m.traveling_to() {
            let mut from_position = m.position().clone();
            let move_result = move_position(&mut from_position, to_position, seconds);
            match move_result {
                MoveResult::Moving => m.set_position(&from_position),
                MoveResult::Done => m.done_traveling(),
            };
        }
    }
}

pub fn idle_calorie_burn<T: NeedsFood>(eaters: &mut Vec<T>, seconds: f32) {
    let mut dead_eater_indexes = Vec::new();
    let mut i = 0;
    for e in eaters.iter_mut() {
        e.burn_calories(seconds / 10000.);
        println!("{}", e.hunger_index());
        if e.hunger_index() > 1. {
            dead_eater_indexes.push(i);
        }
        i += 1;
    }

    for i in dead_eater_indexes.iter().rev() {
        eaters.remove(*i);
    }
}

#[derive(PartialEq)]
enum MoveResult {
    Moving,
    Done,
}

fn move_position(from_position: &mut Position, to_position: &Position, seconds: f32) -> MoveResult {
    let dx = to_position.x - from_position.x;
    let dy = to_position.y - from_position.y;
    let len = (dx * dx + dy * dy).sqrt();
    from_position.x += dx * seconds * 500. / len;
    from_position.y += dy * seconds * 500. / len;
    if len < 0.1 {
        return MoveResult::Done;
    }
    MoveResult::Moving
}
