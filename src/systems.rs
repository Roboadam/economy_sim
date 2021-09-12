use crate::{components::*, entity_map::OneToOne};
use ::rand::Rng;
use hecs::{Entity, With, World};
use macroquad::prelude::*;
use rand_chacha::ChaCha8Rng;

pub fn draw_ai(world: &mut World, texture: &Texture2D) {
    for (_, position) in world.query_mut::<With<AiPersonTag, &Position>>() {
        draw_texture(*texture, position.0, position.1, WHITE);
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

pub fn draw_buildings(world: &mut World, texture: &Texture2D) {
    for (_, position) in world.query_mut::<&mut Position>().with::<BuildingType>() {
        draw_texture(*texture, position.0, position.1, WHITE);
    }
}

pub fn travel(
    world: &mut World,
    building_entities: &Vec<Entity>,
    seconds: f32,
    rng: &mut ChaCha8Rng,
) {
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
    let dx = to_position.0 - from_position.0;
    let dy = to_position.1 - from_position.1;
    let len = (dx * dx + dy * dy).sqrt();
    from_position.0 += dx * seconds * 50. / len;
    from_position.1 += dy * seconds * 50. / len;
    if len < 0.1 {
        return MoveResult::Done;
    }
    MoveResult::Moving
}
