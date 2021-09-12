use hecs::World;
use macroquad::prelude::*;
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::{self, ChaCha8Rng};
use rendering::*;
use spawners::*;
use systems::*;

mod components;
mod rendering;
mod spawners;
mod systems;

#[macroquad::main("City Sim")]
async fn main() {
    let building_texture = open_pixel_texture("textures/ai_travel_point.png").await;
    let ai_player_texture = open_pixel_texture("textures/ai_player.png").await;
    let mut rng = ChaCha8Rng::seed_from_u64(2);

    let mut world = World::new();
    let building_entities = spawn_buildings(5, &mut world, &mut rng);
    spawn_ai_people(3, &mut world, &mut rng);

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
