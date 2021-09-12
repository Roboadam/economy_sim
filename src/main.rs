use crate::ai_travel_point::building_positions;
use crate::components::{AiPersonTag, Hunger, Position};
use ::rand::Rng;
use components::{BuildingTag, TravelingTo};
use hecs::{Entity, With, World};
use macroquad::prelude::*;
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::{self, ChaCha8Rng};
use rendering::*;

mod ai_travel_point;
mod components;
mod rendering;

#[macroquad::main("City Sim")]
async fn main() {
    let building_texture = open_pixel_texture("textures/ai_travel_point.png").await;
    let ai_player_texture = open_pixel_texture("textures/ai_player.png").await;
    let mut rng = ChaCha8Rng::seed_from_u64(1);
    let mut building_entities = vec![];

    let mut world = World::new();
    world.spawn((
        AiPersonTag,
        Hunger(100.),
        Position(20., 20.),
        TravelingTo::Nowhere,
    ));
    let screen_data = get_screen_data();
    for position in building_positions(
        5,
        screen_data.width() as f32,
        screen_data.height() as f32,
        &mut rng,
    ) {
        let entity = world.spawn((BuildingTag, position));
        building_entities.push(entity);
    }

    loop {
        if is_key_pressed(KeyCode::F) {
            println!("FPS: {}", get_fps());
        }

        clear_background(LIGHTGRAY);
        draw_ai(&mut world, &ai_player_texture);
        draw_buildings(&mut world, &building_texture);
        hunger(&mut world, get_frame_time());
        travel(&mut world, &building_entities, get_frame_time(), &mut rng);

        next_frame().await
    }
}

fn draw_ai(world: &mut World, texture: &Texture2D) {
    for (_, position) in world.query_mut::<With<AiPersonTag, &Position>>() {
        draw_texture(*texture, position.0, position.1, WHITE);
    }
}

fn hunger(world: &mut World, seconds: f32) {
    for (_id, (hunger,)) in world.query_mut::<(&mut Hunger,)>() {
        hunger.0 -= seconds / 10.;
    }
}

fn draw_buildings(world: &mut World, texture: &Texture2D) {
    for (_, position) in world.query_mut::<&mut Position>().with::<BuildingTag>() {
        draw_texture(*texture, position.0, position.1, WHITE);
    }
}

fn travel(world: &mut World, building_entities: &Vec<Entity>, seconds: f32, rng: &mut ChaCha8Rng) {
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
    from_position.0 += dx * seconds * 10. / len;
    from_position.1 += dy * seconds * 10. / len;
    if len < 0.1 {
        return MoveResult::Done;
    }
    MoveResult::Moving
}
