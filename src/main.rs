use ::rand::SeedableRng;
use macroquad::prelude::*;
use rand_chacha::ChaCha8Rng;
use spawners::*;
use sprites::Sprites;
use systems::*;

mod ai_person;
mod building;
mod components;
mod spawners;
mod sprites;
mod systems;
mod traits;

#[macroquad::main("City Sim")]
async fn main() {
    let mut rng = ChaCha8Rng::seed_from_u64(2);
    let mut sprites = Sprites::default();
    let ai_texture_index = sprites.add_sprite_from_path("textures/ai_player.png").await;
    let building_texture_index = sprites
        .add_sprite_from_path("textures/ai_travel_point.png")
        .await;
    let mut ai_people = spawn_ai_people(5, ai_texture_index, &mut rng);
    let buildings = spawn_buildings(5, building_texture_index, &mut rng);

    loop {
        if is_key_pressed(KeyCode::F) {
            println!("FPS: {}", get_fps());
        }

        clear_background(LIGHTGRAY);
        ai_people.iter().for_each(|p| draw(p, &sprites));
        buildings.iter().for_each(|p| draw(p, &sprites));
        assign_travel_to_randomly(&mut ai_people, &buildings, &mut rng);
        travel2(&mut ai_people, get_frame_time());
        // travel(&mut world, &world_bounding_box, get_frame_time(), &mut rng);

        next_frame().await
    }
}
