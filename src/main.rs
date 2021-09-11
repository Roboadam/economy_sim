use crate::ai_travel_point::{draw_travel_points, sample_travel_points};
use crate::components::{AiPersonTag, Hunger, Position};
use crate::sprites::SpritePool;
use ::rand::Rng;
use ai_travel_point::AiTravelPoint;
use components::TravelingTo;
use hecs::World;
use macroquad::prelude::*;
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::{self, ChaCha8Rng};
use rendering::*;

mod ai_travel_point;
mod components;
mod rendering;
mod sprites;

#[macroquad::main("City Sim")]
async fn main() {
    let ai_player_texture = open_pixel_texture("textures/ai_player.png").await;
    let mut rng = ChaCha8Rng::seed_from_u64(1);

    let mut sprite_pool = SpritePool::new();
    let travel_point_sprite = sprite_pool.add("textures/ai_travel_point.png").await;
    let travel_points = sample_travel_points(travel_point_sprite);

    let mut world = World::new();
    world.spawn((
        AiPersonTag,
        Hunger(100.),
        Position(20., 20.),
        TravelingTo::Nowhere,
    ));

    loop {
        if is_key_pressed(KeyCode::F) {
            println!("FPS: {}", get_fps());
        }

        clear_background(LIGHTGRAY);
        draw_travel_points(&travel_points, &sprite_pool);
        hunger(&mut world, get_frame_time());
        travel(&mut world, &travel_points, get_frame_time(), &mut rng);
        draw_ai(&mut world, &ai_player_texture);

        next_frame().await
    }
}

fn draw_ai(world: &mut World, texture: &Texture2D) {
    for (_, position) in world.query_mut::<&Position>() {
        draw_texture(*texture, position.0, position.1, WHITE);
    }
}

fn hunger(world: &mut World, seconds: f32) {
    for (_id, (hunger,)) in world.query_mut::<(&mut Hunger,)>() {
        hunger.0 -= seconds / 10.;
    }
}

fn travel(
    world: &mut World,
    travel_points: &Vec<AiTravelPoint>,
    seconds: f32,
    rng: &mut ChaCha8Rng,
) {
    for (_, (traveling_to, position)) in world.query_mut::<(&mut TravelingTo, &mut Position)>() {
        match traveling_to {
            TravelingTo::Nowhere => {
                *traveling_to =
                    TravelingTo::TravelPoint(rng.gen_range(0..travel_points.len() as i32))
            }
            TravelingTo::TravelPoint(i) => {
                let i = *i as usize;
                let move_result = move_ai_people(&travel_points[i], position, seconds);
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
    travel_point: &AiTravelPoint,
    position: &mut Position,
    seconds: f32,
) -> MoveResult {
    let dx = travel_point.position.0 - position.0;
    let dy = travel_point.position.1 - position.1;
    let len = (dx * dx + dy * dy).sqrt();
    position.0 += dx * seconds * 10. / len;
    position.1 += dy * seconds * 10. / len;
    if len < 0.1 {
        return MoveResult::Done;
    }
    MoveResult::Moving
}
