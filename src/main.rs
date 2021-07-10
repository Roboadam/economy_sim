use crate::land_mass_generator::create_land_mass;
use macroquad::prelude::*;
use rendering::*;
use tile_map::{TileMap, TileType};
use TileType::Sea;

mod land_mass_generator;
mod rendering;
mod tile_map;
mod tile_selector;

#[macroquad::main("Texture")]
async fn main() {
    const MAP_WIDTH_IN_TILES: usize = 160;
    const SPEED: f32 = 10.;
    const TILE_WIDTH: f32 = 16.;
    const TILES_ON_SCREEN: i32 = 10;

    let screen_dimensions = screen_dimension_in_tiles(TILES_ON_SCREEN);
    println!("screen_dimensions: {:?} - {}", screen_dimensions, 10./screen_dimensions.1 as f32);
    let rt = pixel_perfect_render_target(screen_dimensions, TILE_WIDTH);
    let texture_atlas = open_pixel_texture("textures/land_tilemap.png").await;
    // let player_texture = open_pixel_texture("textures/player.png").await;
    let mut tile_map = TileMap::new(MAP_WIDTH_IN_TILES);
    create_land_mass(&mut tile_map);

    let mut player_coords: (f32, f32) = (10., 10.);

    loop {
        if is_key_pressed(KeyCode::F) {
            println!("FPS: {}", get_fps());
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
        }

        draw_to_texture(rt, player_coords, TILE_WIDTH);
        clear_background(LIGHTGRAY);
        draw_tile_map(
            &tile_map,
            16.,
            &texture_atlas,
            player_coords,
            screen_dimensions,
        );
        // draw_texture_ex(
        //     player_texture,
        //     target.x,
        //     target.y,
        //     WHITE,
        //     DrawTextureParams {
        //         dest_size: Some(vec2(
        //             player_texture.width() / 2.,
        //             player_texture.height() / 2.,
        //         )),
        //         ..Default::default()
        //     },
        // );
        // draw_rectangle(target.x, target.y, 2., 2., RED);
        draw_texture_to_screen(rt);

        next_frame().await
    }
}
