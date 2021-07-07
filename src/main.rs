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
    let rt = pixel_perfect_render_target();
    let texture_atlas = open_texture_atlas("textures/land_tilemap.png").await;
    const MAP_WIDTH_IN_TILES: usize = 160;
    const SPEED: f32 = 1.;

    let mut tile_map = TileMap::new(MAP_WIDTH_IN_TILES);
    create_land_mass(&mut tile_map);

    let mut target = vec2(200., 200.);

    loop {
        if is_key_down(KeyCode::W) {
            target.y -= SPEED;
        }
        if is_key_down(KeyCode::S) {
            target.y += SPEED;
        }
        if is_key_down(KeyCode::A) {
            target.x -= SPEED;
        }
        if is_key_down(KeyCode::D) {
            target.x += SPEED;
        }
        if is_key_pressed(KeyCode::C) {
            for y in 0..tile_map.width as i32 {
                for x in 0..tile_map.width as i32 {
                    tile_map.set_tile(x, y, Sea);
                }
            }
            create_land_mass(&mut tile_map);
        }

        draw_to_texture(rt, target);
        clear_background(LIGHTGRAY);
        draw_tile_map(&tile_map, 16., &texture_atlas, target);
        draw_texture_to_screen(rt);

        next_frame().await
    }
}
