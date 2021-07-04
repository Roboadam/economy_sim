use crate::land_mass_generator::create_land_mass;
use macroquad::prelude::*;
use selection::Selection;
use tile_map::{TileMap, TileType};
use tile_selector::TileSelector;
use TileType::{Land, Sea};

mod land_mass_generator;
mod selection;
mod tile_map;
mod tile_selector;

#[macroquad::main("Texture")]
async fn main() {
    let texture_atlas: Texture2D = load_texture("textures/land_tilemap.png").await.unwrap();
    texture_atlas.set_filter(FilterMode::Nearest);
    const MAP_WIDTH_IN_TILES: usize = 16;

    let mut selection = Selection::new(MAP_WIDTH_IN_TILES);

    let mut tile_map = TileMap::new(MAP_WIDTH_IN_TILES);
    create_land_mass(&mut tile_map);

    loop {
        clear_background(LIGHTGRAY);
        let screen_width_in_pixels = if screen_height() > screen_width() {
            screen_height()
        } else {
            screen_width()
        };
        let tile_width_in_screen_pixels = screen_width_in_pixels / MAP_WIDTH_IN_TILES as f32;

        if is_key_pressed(KeyCode::W) || is_key_pressed(KeyCode::Up) {
            selection.up();
        }
        if is_key_pressed(KeyCode::S) || is_key_pressed(KeyCode::Down) {
            selection.down();
        }
        if is_key_pressed(KeyCode::A) || is_key_pressed(KeyCode::Left) {
            selection.left();
        }
        if is_key_pressed(KeyCode::D) || is_key_pressed(KeyCode::Right) {
            selection.right();
        }
        if is_key_pressed(KeyCode::C) {
            for y in 0..tile_map.width as i32 {
                for x in 0..tile_map.width as i32 {
                    tile_map.set_tile(x, y, Sea);
                }
            }
            create_land_mass(&mut tile_map);
        }
        if is_key_pressed(KeyCode::Space) {
            if let Some(tile_type) = tile_map.get_tile(selection.x, selection.y) {
                let new_tile_type = match tile_type {
                    Land => Sea,
                    Sea => Land,
                };
                tile_map.set_tile(selection.x, selection.y, new_tile_type)
            }
        }

        draw_tile_map(&tile_map, tile_width_in_screen_pixels, &texture_atlas);
        draw_selection(&selection, tile_width_in_screen_pixels);

        next_frame().await
    }
}

pub fn draw_selection(selection: &Selection, tile_len: f32) {
    draw_rectangle_lines(
        selection.x as f32 * tile_len + 2.5,
        selection.y as f32 * tile_len + 2.5,
        tile_len - 5.,
        tile_len - 5.,
        5.,
        WHITE,
    );
    draw_rectangle_lines(
        selection.x as f32 * tile_len + 2.,
        selection.y as f32 * tile_len + 2.,
        tile_len - 4.,
        tile_len - 4.,
        3.,
        BLACK,
    );
}

pub fn draw_tile_map(tile_map: &TileMap, tile_width_in_screen_pixels: f32, texture_atlas: &Texture2D) {
    let tile_selector = TileSelector::new();
    for y in 0..tile_map.width as i32 {
        for x in 0..tile_map.width as i32 {
            draw_texture_ex(
                *texture_atlas,
                x as f32 * tile_width_in_screen_pixels,
                y as f32 * tile_width_in_screen_pixels,
                WHITE,
                texture_params(x, y, tile_width_in_screen_pixels, tile_map, &tile_selector),
            );
        }
    }
}

fn texture_params(
    x: i32,
    y: i32,
    tile_len: f32,
    tile_map: &TileMap,
    tile_selector: &TileSelector,
) -> DrawTextureParams {
    const TILE_PIXEL_LEN: f32 = 16f32;
    let nw = tile_map.get_tile(x, y).unwrap_or(&TileType::Sea);
    let sw = tile_map.get_tile(x, y + 1).unwrap_or(&TileType::Sea);
    let ne = tile_map.get_tile(x + 1, y).unwrap_or(&TileType::Sea);
    let se = tile_map.get_tile(x + 1, y + 1).unwrap_or(&TileType::Sea);

    let (x_coord, y_coord) = tile_selector.select_tile(*nw, *ne, *sw, *se);

    DrawTextureParams {
        source: Some(Rect {
            x: TILE_PIXEL_LEN * x_coord as f32,
            y: TILE_PIXEL_LEN * y_coord as f32,
            w: TILE_PIXEL_LEN,
            h: TILE_PIXEL_LEN,
        }),
        dest_size: Some(vec2(tile_len, tile_len)),
        ..Default::default()
    }
}
