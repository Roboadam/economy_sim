use crate::land_mass_generator::create_land_mass;
use macroquad::prelude::*;
use selection::Selection;
use tile_map::{TileMap, TileType};
use TileType::Sea;
use rendering::*;

mod land_mass_generator;
mod selection;
mod tile_map;
mod tile_selector;
mod rendering;

#[macroquad::main("Texture")]
async fn main() {
    let rt = pixel_perfect_render_target();
    let texture_atlas = open_texture_atlas("textures/land_tilemap.png").await;
    const MAP_WIDTH_IN_TILES: usize = 160;
    const TILE_SIZE: i32 = 16;

    let mut selection = Selection::new(MAP_WIDTH_IN_TILES);

    let mut tile_map = TileMap::new(MAP_WIDTH_IN_TILES);
    create_land_mass(&mut tile_map);

    let mut target = vec2(200., 200.);

    loop {
        if is_key_pressed(KeyCode::W) {
            selection.up();
            target.y = ((selection.y - 5) * TILE_SIZE + 126) as f32;
        }
        if is_key_pressed(KeyCode::S) {
            selection.down();
            target.y = ((selection.y - 5) * TILE_SIZE + 126) as f32;
        }
        if is_key_pressed(KeyCode::A) {
            selection.left();
            target.x = ((selection.x - 5) * TILE_SIZE + 126) as f32;
        }
        if is_key_pressed(KeyCode::D) {
            selection.right();
            target.x = ((selection.x - 5) * TILE_SIZE + 126) as f32;
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
        draw_tile_map(&tile_map, 16., &texture_atlas);
        draw_selection(&selection, 16.);
        draw_texture_to_screen(rt);

        next_frame().await
    }
}
