use crate::{components::*, entity_map::OneToOne, quadtree::AABB, world::W};
use ::rand::Rng;
use hecs::{Entity, World};
use macroquad::prelude::*;
use rand_chacha::ChaCha8Rng;

pub fn draw_ai(world: &mut W) {
    for (position, sprite) in world.ai_positions_and_sprites() {
        draw_texture(*sprite, position.x, position.y, WHITE);
    }
}

pub fn hunger(world: &mut World, seconds: f32) {
    for (_id, (hunger,)) in world.query_mut::<(&mut Hunger,)>() {
        hunger.0 -= seconds / 10.;
    }
}

pub fn buy_homes(world: &mut World, home_ownership: &mut OneToOne) {
    let people = world
        .query_mut::<&AiPersonTag>()
        .into_iter()
        .map(|(e, _)| e)
        .collect::<Vec<_>>();
    let homes = world
        .query_mut::<&BuildingType>()
        .into_iter()
        .filter(|(_, bt)| **bt == BuildingType::Home)
        .map(|(e, _)| e)
        .collect::<Vec<_>>();
    people
        .into_iter()
        .zip(homes.into_iter())
        .for_each(|(p, h)| home_ownership.insert(p, h).unwrap_or_default());
}

pub fn has_home(entity: Entity, home_ownership: &OneToOne) -> bool {
    home_ownership.contains_key(entity)
}

pub fn draw_businesses(world: &mut W, aabb: &AABB) {
    for (position, sprite) in world.business_positions_and_sprites(aabb) {
        draw_texture(*sprite, position.x, position.y, WHITE);
    }
}

pub fn travel(
    world: &mut W,
    seconds: f32,
) {
    for entity_id in world.traveling_to_and_positions() {

    }
    for (_, (traveling_to, from_position)) in world
        .query::<(&mut TravelingTo, &mut Position)>()
        .into_iter()
    {
        match traveling_to {
            TravelingTo::Nowhere => {
                let index = rng.gen_range(0..building_entities.len());
                if let Ok(to_position) = world.get::<Position>(building_entities[index]) {
                    *traveling_to = TravelingTo::TravelPosition(*to_position);
                }
            }
            TravelingTo::TravelPosition(to_position) => {
                let move_result = move_ai_people(from_position, to_position, seconds);
                if move_result == MoveResult::Done {
                    *traveling_to = TravelingTo::Nowhere;
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
    from_position.x += dx * seconds * 50. / len;
    from_position.y += dy * seconds * 50. / len;
    if len < 0.1 {
        return MoveResult::Done;
    }
    MoveResult::Moving
}
