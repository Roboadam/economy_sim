use std::fs::File;

use crate::ai_travel_point::{draw_travel_points, sample_travel_points};
use crate::business::{BusinessId, Businesses};
use crate::components::Position;
use crate::money::Money;
use crate::person::{AiPerson, People, Person, PersonId};
use crate::sprites::SpritePool;
use macroquad::prelude::*;
use rendering::*;
use ron::de::from_reader;

mod ai_travel_point;
mod business;
mod components;
mod money;
mod person;
mod rendering;
mod sprites;
mod tile_map;
mod tile_selector;

#[macroquad::main("City Sim")]
async fn main() {
    const SPEED: f32 = 5.;
    const TILE_WIDTH: f32 = 16.;
    const TILES_ON_SCREEN: i32 = 10;

    let my_id = PersonId(0);
    let mut screen_data =
        ScreenData::new(TILES_ON_SCREEN, TILE_WIDTH, screen_width(), screen_height());
    let player_texture = open_pixel_texture("textures/player.png").await;
    let ai_player_texture = open_pixel_texture("textures/ai_player.png").await;

    let buffer = File::open("foo.txt").unwrap();
    let mut businesses = Businesses::new(from_reader(buffer).unwrap());
    let mut people = People::new();
    people.add(
        my_id,
        Person {
            hunger: 100.,
            position: Position(0., 0.),
        },
    );
    let mut money = Money::new();
    money.create_cash(0, 100.3);

    let mut ai_person = AiPerson {
        person: Person {
            hunger: 100.,
            position: Position(20., 20.),
        },
        travel_to: None,
    };

    let mut curr_screen_width = screen_width() as i32;
    let mut curr_screen_height = screen_height() as i32;
    let close_business: Option<BusinessId> = None;
    let mut sprite_pool = SpritePool::new();
    let travel_point_sprite = sprite_pool.add("textures/ai_travel_point.png").await;
    let travel_points = sample_travel_points(travel_point_sprite);

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
        if is_key_pressed(KeyCode::B) {
            if let Some(business_id) = close_business {
                // let business = businesses.get(business_id);
                business::widget_transaction(
                    business_id,
                    my_id,
                    &mut businesses,
                    &mut people,
                    &mut money,
                );
            }
        }

        if curr_screen_height != screen_height() as i32
            || curr_screen_width != screen_width() as i32
        {
            screen_data.update_with_screen_size(screen_width(), screen_height());
            curr_screen_width = screen_width() as i32;
            curr_screen_height = screen_height() as i32;
        }

        clear_background(LIGHTGRAY);
        let my_position = people.get(my_id).position;
        draw_travel_points(&travel_points, &sprite_pool, TILE_WIDTH);
        draw_texture(
            player_texture,
            my_position.0 * TILE_WIDTH,
            my_position.1 * TILE_WIDTH,
            WHITE,
        );
        draw_texture(
            ai_player_texture,
            ai_person.person.position.0 * TILE_WIDTH,
            ai_person.person.position.1 * TILE_WIDTH,
            WHITE,
        );

        people.update(get_frame_time());
        ai_person.update(get_frame_time(), &travel_points);
        next_frame().await
    }
}
