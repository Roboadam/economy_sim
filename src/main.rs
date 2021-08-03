use std::{collections::HashMap, fs::File};

use crate::building_generator::generate_buildings;
use crate::business::Business;
use crate::land_mass_generator::create_land_mass;
use macroquad::prelude::*;
use rendering::*;
use ron::{
    de::from_reader,
    ser::{to_writer_pretty, PrettyConfig},
};
use tile_map::TileMap;

mod building_generator;
mod business;
mod land_mass_generator;
mod rendering;
mod tile_map;
mod tile_selector;

#[macroquad::main("City Sim")]
async fn main() {
    const MAP_WIDTH_IN_TILES: usize = 50;
    const SPEED: f32 = 5.;
    const TILE_WIDTH: f32 = 16.;
    const TILES_ON_SCREEN: i32 = 10;

    let mut screen_data =
        ScreenData::new(TILES_ON_SCREEN, TILE_WIDTH, screen_width(), screen_height());
    let texture_atlas = open_pixel_texture("textures/land_tilemap.png").await;
    let player_texture = open_pixel_texture("textures/player.png").await;
    let mut tile_map = TileMap::new(MAP_WIDTH_IN_TILES);
    create_land_mass(&mut tile_map);
    generate_buildings(&mut tile_map);

    let buffer = File::open("foo.txt").unwrap();
    let businesses_by_id: HashMap<i32, Business> = from_reader(buffer).unwrap();
    let buffer = File::create("foo.txt").unwrap();
    let _result = to_writer_pretty(buffer, &businesses_by_id, PrettyConfig::new()).unwrap();

    let mut player_coords: (f32, f32) = (10., 10.);
    let mut curr_screen_width = screen_width() as i32;
    let mut curr_screen_height = screen_height() as i32;
    let mut status_text = None;

    loop {
        if is_key_pressed(KeyCode::F) {
            println!("FPS: {}, player_coords: {:?}", get_fps(), player_coords);
        }
        let mut moved = false;
        if is_key_down(KeyCode::W) {
            player_coords.1 -= SPEED * get_frame_time();
            moved = true;
        }
        if is_key_down(KeyCode::S) {
            player_coords.1 += SPEED * get_frame_time();
            moved = true;
        }
        if is_key_down(KeyCode::A) {
            player_coords.0 -= SPEED * get_frame_time();
            moved = true;
        }
        if is_key_down(KeyCode::D) {
            player_coords.0 += SPEED * get_frame_time();
            moved = true;
        }

        if moved {
            if let Some(building_id) = tile_map.close_building(player_coords) {
                if let Some(business) = businesses_by_id.get(&building_id) {
                    // TODO - setting this every frame is time consuming
                    status_text = Some(format!(
                        "{} - widgets:{}",
                        business.name, business.num_widgets
                    ));
                } else {
                    status_text = None;
                }
            } else {
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

        draw_to_texture(player_coords, &screen_data);
        clear_background(LIGHTGRAY);
        draw_tile_map(&tile_map, &texture_atlas, player_coords, &screen_data);
        draw_texture(
            player_texture,
            player_coords.0 * TILE_WIDTH,
            player_coords.1 * TILE_WIDTH,
            WHITE,
        );
        draw_texture_to_screen(&screen_data);
        if let Some(ref text) = status_text {
            draw_text_ex(text, 20.0, 20.0, TextParams::default());
        }

        next_frame().await
    }
}
