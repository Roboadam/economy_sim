use crate::{
    ai_person::{AiPerson, IsDrawable},
    components::*,
    quadtree::AABB,
    sprites::Sprites,
    world::W,
};
use ::rand::Rng;
use macroquad::prelude::*;
use rand_chacha::ChaCha8Rng;

pub fn draw_ai(world: &mut W) {
    for (position, sprite) in world.ai_positions_and_sprites() {
        draw_texture(*sprite, position.x, position.y, WHITE);
    }
}

pub fn draw<T: IsDrawable>(drawable: &T, sprites: &Sprites) {
    let (texture_index, position) = drawable.render_info();
    let texture = sprites.texture(texture_index);
    println!("drawing {:?}", position);
    draw_texture(*texture, position.x, position.y, WHITE);
}

pub fn draw_businesses(world: &mut W, aabb: &AABB) {
    for (position, sprite) in world.business_positions_and_sprites(aabb) {
        draw_texture(*sprite, position.x, position.y, WHITE);
    }
}

pub fn travel(w: &mut W, range: &AABB, seconds: f32, rng: &mut ChaCha8Rng) {
    for entity_id in w.traveling_to_and_positions() {
        if let Some(traveling_to) = w.traveling_to_for_entity_id(entity_id) {
            match traveling_to {
                TravelingTo::Nowhere => {
                    let business_positions = w.business_positions(range);
                    let index = rng.gen_range(0..business_positions.len());
                    let business_position = *business_positions
                        .get(index)
                        .expect("Index generated in range");
                    w.update_traveling_to(
                        entity_id,
                        TravelingTo::TravelPosition(business_position),
                    );
                }
                TravelingTo::TravelPosition(to_position) => {
                    if let Some(mut from_position) = w.position_for_entity_id(entity_id) {
                        let move_result = move_ai_people(&mut from_position, to_position, seconds);
                        match move_result {
                            MoveResult::Moving => w.update_position(entity_id, from_position),
                            MoveResult::Done => {
                                w.update_traveling_to(entity_id, TravelingTo::Nowhere)
                            }
                        };
                    }
                }
            }
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
