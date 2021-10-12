use ::rand::SeedableRng;
use kiss3d::{
    light::Light,
    nalgebra::{UnitQuaternion, Vector3},
    window::Window,
};
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
    let mut window = Window::new("Kiss3d: cube");
    let mut c = window.add_cube(1.0, 1.0, 1.0);

    c.set_color(1.0, 0.0, 0.0);

    window.set_light(Light::StickToCamera);

    let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);

    while window.render() {
        c.prepend_to_local_rotation(&rot);
    }

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
        let frame_time = get_frame_time();
        ai_people.iter().for_each(|p| draw(p, &sprites));
        buildings.iter().for_each(|p| draw(p, &sprites));
        assign_travel_to_randomly(&mut ai_people, &buildings, &mut rng);
        travel(&mut ai_people, frame_time);
        idle_calorie_burn(&mut ai_people, frame_time);

        next_frame().await
    }
}
