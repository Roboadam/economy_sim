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
    rng: &mut ChaCha8Rng,
) {
    for tt in tts.iter_mut() {
        if tt.traveling_to().is_none() {
            let i = rng.gen_range(0..dests.len());
            let dest = dests.get(i).expect("Random within range").position();
            tt.set_traveling_to(dest);
        }
    }
}

pub fn travel<T: HasTravelingTo + HasPosition>(movers: &mut Vec<T>, seconds: f32) {

    for m in movers.iter_mut() {
        if let Some(to_position) = m.traveling_to() {
            let mut from_position = m.position().clone();
            let move_result = move_position(&mut from_position, to_position, seconds);
            match move_result {
                MoveResult::Moving(delta) => m.move_delta(&delta, seconds),
                MoveResult::Done => m.done_traveling(),
            };
        }
    }
}

pub fn idle_calorie_burn<T: NeedsFood>(eaters: &mut Vec<T>, seconds: f32) {
    for e in eaters.iter_mut() {
        if e.hunger_index() < 1. {
            e.burn_calories(seconds / 10000.);
        }
    }
}

#[derive(PartialEq)]
enum MoveResult {
    Moving(Position),
    Done,
}

fn move_position(from_position: &Position, to_position: &Position, seconds: f32) -> MoveResult {
    let dx = to_position.x - from_position.x;
    let dy = to_position.y - from_position.y;
    let len = (dx * dx + dy * dy).sqrt();

    if len < 5. {
        return MoveResult::Done;
    }

    MoveResult::Moving(Position {
        x: dx * seconds * 500. / len,
        y: dy * seconds * 500. / len,
    })
}
