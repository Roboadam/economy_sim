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

pub fn travel2<T: HasTravelingTo + HasPosition>(movers: &mut Vec<T>, seconds: f32) {
    for m in movers.iter_mut() {
        if let Some(to_position) = m.traveling_to() {
            let mut from_position = m.position().clone();
            let move_result = move_ai_people(&mut from_position, to_position, seconds);
            match move_result {
                MoveResult::Moving => m.set_position(&from_position),
                MoveResult::Done => m.done_traveling(),
            };
        }
    }
}

#[derive(PartialEq)]
enum MoveResult {
    Moving,
    Done,
}

fn move_ai_people(
    from_position: &mut Position,
    to_position: &Position,
    seconds: f32,
) -> MoveResult {
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
