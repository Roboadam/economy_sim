use crate::building_generator::generate_buildings;
use crate::land_mass_generator::create_land_mass;
use macroquad::prelude::*;
use rendering::*;
use tile_map::{TileMap, TileType};
use TileType::Sea;

mod building_generator;
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
        if is_key_pressed(KeyCode::C) {
            for y in 0..tile_map.width as i32 {
                for x in 0..tile_map.width as i32 {
                    tile_map.set_tile(x, y, Sea);
                }
            }
            create_land_mass(&mut tile_map);
            generate_buildings(&mut tile_map);
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

        next_frame().await
    }
}
