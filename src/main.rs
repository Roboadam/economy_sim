use crate::ai_travel_point::{draw_travel_points, sample_travel_points};
use crate::components::{AiPersonTag, Hunger, Position};
use crate::person::{AiPerson, People, Person, PersonId};
use crate::sprites::SpritePool;
use ::rand::{thread_rng, Rng};
use ai_travel_point::AiTravelPoint;
use components::TravelingTo;
use hecs::{Entity, World};
use macroquad::prelude::*;
use rendering::*;

mod ai_travel_point;
mod components;
mod person;
mod rendering;
mod sprites;

#[macroquad::main("City Sim")]
async fn main() {
    const SPEED: f32 = 50.;
    let my_id = PersonId(0);
    let player_texture = open_pixel_texture("textures/player.png").await;
    let ai_player_texture = open_pixel_texture("textures/ai_player.png").await;

    let mut people = People::new();
    people.add(
        my_id,
        Person {
            hunger: 100.,
            position: Position(0., 0.),
        },
    );

    let mut ai_person = AiPerson {
        person: Person {
            hunger: 100.,
            position: Position(20., 20.),
        },
        travel_to: None,
    };

    let mut curr_screen_width = screen_width() as i32;
    let mut curr_screen_height = screen_height() as i32;
    let mut sprite_pool = SpritePool::new();
    let travel_point_sprite = sprite_pool.add("textures/ai_travel_point.png").await;
    let travel_points = sample_travel_points(travel_point_sprite);

    let mut world = World::new();
    world.spawn((AiPersonTag, Hunger(100.), Position(20., 20.)));

    loop {
        if is_key_pressed(KeyCode::F) {
            println!(
                "FPS: {}, player_coords: {:?}",
                get_fps(),
                people.get(my_id).position
            );
        }
        if is_key_down(KeyCode::W) {
            people.get_mut(my_id).position.1 -= SPEED * get_frame_time();
        }
        if is_key_down(KeyCode::S) {
            people.get_mut(my_id).position.1 += SPEED * get_frame_time();
        }
        if is_key_down(KeyCode::A) {
            people.get_mut(my_id).position.0 -= SPEED * get_frame_time();
        }
        if is_key_down(KeyCode::D) {
            people.get_mut(my_id).position.0 += SPEED * get_frame_time();
        }

        if curr_screen_height != screen_height() as i32
            || curr_screen_width != screen_width() as i32
        {
            curr_screen_width = screen_width() as i32;
            curr_screen_height = screen_height() as i32;
        }

        clear_background(LIGHTGRAY);
        let my_position = people.get(my_id).position;
        draw_travel_points(&travel_points, &sprite_pool);
        draw_texture(player_texture, my_position.0, my_position.1, WHITE);
        draw_texture(
            ai_player_texture,
            ai_person.person.position.0,
            ai_person.person.position.1,
            WHITE,
        );

        people.update(get_frame_time());
        ai_person.update(get_frame_time(), &travel_points);
        next_frame().await
    }
}

fn hunger(world: &mut World, seconds: f32) {
    for (_id, (hunger,)) in world.query_mut::<(&mut Hunger,)>() {
        hunger.0 -= seconds / 10.;
    }
}

fn give_place_to_go(world: &mut World, num_places: i32) {
    let mut rng = thread_rng();

    let still_guys: Vec<_> = world.query::<&AiPerson>().without::<TravelingTo>().iter().collect();

    still_guys.iter().for_each(|(entity,_)| {
        let to = rng.gen_range(0..num_places);
        world.insert_one(*entity, TravelingTo(to));
    });
}

fn move_ai_people(world: &mut World, time: f32, travel_points: &Vec<AiTravelPoint>, seconds: f32) {
    for (entity, (position, _, &traveling_to)) in
        world.query_mut::<(&mut Position, &AiPerson, &TravelingTo)>()
    {
        if let Some(aitp) = travel_points.get(traveling_to.0 as usize) {
            let dx = aitp.position.0 - position.0;
            let dy = aitp.position.1 - position.1;
            let len = (dx * dx + dy * dy).sqrt();
            position.0 += dx * seconds * 10. / len;
            position.1 += dy * seconds * 10. / len;
            if len < 0.1 {
                world.remove_one::<TravelingTo>(entity);
            }
        }
    }
}
