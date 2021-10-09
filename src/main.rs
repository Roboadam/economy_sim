use std::cmp::max;

use ::rand::SeedableRng;
use ai_person::HasPosition;
use components::Position;
use macroquad::prelude::*;
use quadtree::AABB;
use rand_chacha::ChaCha8Rng;
use rendering::*;
use spawners::*;
use sprites::Sprites;
use systems::*;
use world::W;

mod ai_person;
mod components;
mod quadtree;
mod rendering;
mod spawners;
mod sprites;
mod systems;
mod world;

#[macroquad::main("City Sim")]
async fn main() {
    let mut rng = ChaCha8Rng::seed_from_u64(2);
    let mut sprites = Sprites::default();
    let ai_texture_index = sprites.add_sprite_from_path("textures/ai_player.png").await;
    let ai_people = spawn_ai_people2(5, ai_texture_index, &mut rng);

    let building_texture = open_pixel_texture("textures/ai_travel_point.png").await;
    let ai_player_texture = open_pixel_texture("textures/ai_player.png").await;
    let half_dimension = max(get_screen_data().width(), get_screen_data().height()) as f32 / 2.;
    let center = Position {
        x: half_dimension,
        y: half_dimension,
    };
    let world_bounding_box = AABB::new(center, half_dimension);
    let mut world = W::new(&world_bounding_box, &2);
    let building_sprite = world.add_sprite_component(building_texture);
    let person_sprite = world.add_sprite_component(ai_player_texture);

    spawn_businesses(5, building_sprite, &mut world);
    spawn_homes(3, building_sprite, &mut world);
    spawn_ai_people(3, person_sprite, &mut world);

    loop {
        if is_key_pressed(KeyCode::F) {
            println!("FPS: {}", get_fps());
        }

        clear_background(LIGHTGRAY);
        ai_people.iter().for_each(|p| draw(p, &sprites));
        // draw_ai(&world);
        // draw_texture(ai_player_texture, 55., 137., WHITE);
        draw_businesses(&mut world, &world_bounding_box);
        // travel(&mut world, &world_bounding_box, get_frame_time(), &mut rng);

        next_frame().await
    }
}
