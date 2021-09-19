use std::cmp::max;

use components::{BuildingType, Position};
use entity_map::OneToOne;
// use hecs::World;
use macroquad::prelude::*;
use quadtree::AABB;
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::{self, ChaCha8Rng};
use rendering::*;
use spawners::*;
use systems::*;
use world::W;

mod components;
mod entity_map;
mod quadtree;
mod rendering;
mod spawners;
mod systems;
mod world;

#[macroquad::main("City Sim")]
async fn main() {
    let building_texture = open_pixel_texture("textures/ai_travel_point.png").await;
    let ai_player_texture = open_pixel_texture("textures/ai_player.png").await;
    let mut rng = ChaCha8Rng::seed_from_u64(2);

    let half_dimension = max(get_screen_data().width(), get_screen_data().height()) as f32 / 2.;
    let center = Position {
        x: half_dimension,
        y: half_dimension,
    };
    let mut world = W::new(AABB::new(center, half_dimension));
    let building_sprite = world.add_sprite_component(building_texture);

    let resturant_entities = spawn_resturants(5, building_sprite, &mut world, &mut rng);
    let home_entities = spawn_buildings(3, &mut world, &mut rng, BuildingType::Resturant);
    let home_ownership = OneToOne::new();

    spawn_ai_people(3, &mut world, &mut rng);

    loop {
        if is_key_pressed(KeyCode::F) {
            println!("FPS: {}", get_fps());
        }

        clear_background(LIGHTGRAY);
        draw_ai(&mut world, &ai_player_texture);
        draw_buildings(&mut world, &building_texture);
        hunger(&mut world, get_frame_time());
        travel(&mut world, &resturant_entities, get_frame_time(), &mut rng);

        next_frame().await
    }
}
