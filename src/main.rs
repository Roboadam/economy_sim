use std::fs::File;

use crate::ai_travel_point::{draw_travel_points, sample_travel_points};
use crate::building_generator::generate_buildings;
use crate::business::{BusinessId, Businesses};
use crate::components::Position;
use crate::land_mass_generator::create_land_mass;
use crate::money::Money;
use crate::person::{AiPerson, People, Person, PersonId};
use crate::sprites::SpritePool;
use macroquad::prelude::*;
use rendering::*;
use ron::de::from_reader;
use tile_map::TileMap;

mod ai_travel_point;
mod building_generator;
mod business;
mod components;
mod land_mass_generator;
mod money;
mod person;
mod rendering;
mod sprites;
mod tile_map;
mod tile_selector;

#[macroquad::main("City Sim")]
async fn main() {
    const MAP_WIDTH_IN_TILES: usize = 50;
    const SPEED: f32 = 5.;
    const TILE_WIDTH: f32 = 16.;
    const TILES_ON_SCREEN: i32 = 10;

    let my_id = PersonId(0);
    let mut screen_data =
        ScreenData::new(TILES_ON_SCREEN, TILE_WIDTH, screen_width(), screen_height());
    let texture_atlas = open_pixel_texture("textures/land_tilemap.png").await;
    let player_texture = open_pixel_texture("textures/player.png").await;
    let ai_player_texture = open_pixel_texture("textures/ai_player.png").await;
    let mut tile_map = TileMap::new(MAP_WIDTH_IN_TILES);
    create_land_mass(&mut tile_map);
    generate_buildings(&mut tile_map);

    let buffer = File::open("foo.txt").unwrap();
    let mut businesses = Businesses::new(from_reader(buffer).unwrap());
    let mut people = People::new();
    people.add(
        my_id,
        Person {
            hunger: 100.,
            position: Position(10., 10.),
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
    let mut status_text = None;
    let mut close_business: Option<BusinessId> = None;
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
        let mut moved = false;
        if is_key_down(KeyCode::W) {
            people.get_mut(my_id).position.1 -= SPEED * get_frame_time();
            moved = true;
        }
        if is_key_down(KeyCode::S) {
            people.get_mut(my_id).position.1 += SPEED * get_frame_time();
            moved = true;
        }
        if is_key_down(KeyCode::A) {
            people.get_mut(my_id).position.0 -= SPEED * get_frame_time();
            moved = true;
        }
        if is_key_down(KeyCode::D) {
            people.get_mut(my_id).position.0 += SPEED * get_frame_time();
            moved = true;
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
                moved = true;
            }
        }

        if moved {
            if let Some(building_id) = tile_map.close_building(people.get(my_id).position) {
                let business = businesses.get(building_id);
                // TODO - setting this every frame is time consuming
                close_business = Some(building_id);
                let my_cash = money.funds(my_id.0);
                let my_hunger = people.get(my_id).hunger;
                status_text = Some(format!(
                    "{} - widgets:{}\nmycash: {}, my_hunger: {}",
                    business.name, business.num_widgets, my_cash, my_hunger
                ));
            } else {
                close_business = None;
                status_text = None;
            }
        }

        if curr_screen_height != screen_height() as i32
            || curr_screen_width != screen_width() as i32
        {
            screen_data.update_with_screen_size(screen_width(), screen_height());
            curr_screen_width = screen_width() as i32;
            curr_screen_height = screen_height() as i32;
        }

        draw_to_texture(people.get(my_id).position, &screen_data);
        clear_background(LIGHTGRAY);
        let my_position = people.get(my_id).position;
        draw_tile_map(
            &tile_map,
            &texture_atlas,
            people.get(my_id).position,
            &screen_data,
        );
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
        draw_texture_to_screen(&screen_data);
        if let Some(ref text) = status_text {
            draw_text_ex(text, 20.0, 20.0, TextParams::default());
        }

        people.update(get_frame_time());
        ai_person.update(get_frame_time(), &travel_points);
        next_frame().await
    }
}
