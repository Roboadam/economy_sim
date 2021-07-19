use crate::business::Business;
use crate::land_mass_generator::create_land_mass;
use crate::tile_map::Building;
use crate::{building_generator::generate_buildings};
use macroquad::prelude::*;
use rendering::*;
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
    let buildings = generate_buildings(&mut tile_map);
    let businesses: Vec<Business> = buildings.iter().filter(|building| {
        match building {
            Building { id: _, building_type: tile_map::BuildingType::Business } => true,
            _ => false,
        }
    }).map(|building| {
        let building_id = building.id;
        Business {
            cash: 0.,
            num_widgets: 0,
            price: 0.,
            building_id,
            name: format!("Building #{}", building_id),
        }
    }).collect();
    let mut player_coords: (f32, f32) = (10., 10.);
    let mut curr_screen_width = screen_width() as i32;
    let mut curr_screen_height = screen_height() as i32;

    loop {
        if is_key_pressed(KeyCode::F) {
            println!("FPS: {}, player_coords: {:?}", get_fps(), player_coords);
        }
        if is_key_down(KeyCode::W) {
            player_coords.1 -= SPEED * get_frame_time();
        }
        if is_key_down(KeyCode::S) {
            player_coords.1 += SPEED * get_frame_time();
        }
        if is_key_down(KeyCode::A) {
            player_coords.0 -= SPEED * get_frame_time();
        }
        if is_key_down(KeyCode::D) {
            player_coords.0 += SPEED * get_frame_time();
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
        draw_text_ex("Some text here", 20.0, 20.0, TextParams::default());

        next_frame().await
    }
}
