use macroquad::prelude::*;
use selection::Selection;
use tile_map::{TileMap, TileType};
use crate::land_mass_generator::create_land_mass;

mod selection;
mod tile_map;
mod land_mass_generator;

#[macroquad::main("Texture")]
async fn main() {
    let texture: Texture2D = load_texture("textures/land_tilemap.png").await.unwrap();
    texture.set_filter(FilterMode::Nearest);
    const TILES_PER_SIDE: usize = 16;

    let mut selection = Selection::new(TILES_PER_SIDE);

    let mut tile_map = TileMap::new(TILES_PER_SIDE);
    create_land_mass(&mut tile_map);

    loop {
        clear_background(LIGHTGRAY);
        let screen_min_len = if screen_height() < screen_width() {
            screen_height()
        } else {
            screen_width()
        };
        let tile_len = screen_min_len / TILES_PER_SIDE as f32;

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
                    tile_map.set_tile(x, y, TileType::Sea);
                }
            }
            create_land_mass(&mut tile_map);
        }
        if is_key_pressed(KeyCode::Space) {
            if let Some(tile_type) = tile_map.get_tile(selection.x, selection.y) {
                let new_tile_type = match tile_type {
                    TileType::Land => TileType::Sea,
                    TileType::Sea => TileType::Beach,
                    TileType::Beach => TileType::Land,
                };
                tile_map.set_tile(selection.x, selection.y, new_tile_type)
            }
        }

        draw_tile_map(&tile_map, tile_len, &texture);
        draw_selection(&selection, tile_len);

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

pub fn draw_tile_map(tile_map: &TileMap, tile_len: f32, texture: &Texture2D) {
    const TILE_PIXEL_LEN: f32 = 16f32;

    let water_tile = Rect {
        x: 0.,
        y: 0.,
        w: TILE_PIXEL_LEN,
        h: TILE_PIXEL_LEN,
    };
    let land_tile = Rect {
        x: TILE_PIXEL_LEN,
        y: 0.,
        w: TILE_PIXEL_LEN,
        h: TILE_PIXEL_LEN,
    };
    let beach_tile = Rect {
        x: TILE_PIXEL_LEN * 2.,
        y: 0.,
        w: TILE_PIXEL_LEN,
        h: TILE_PIXEL_LEN,
    };

    for y in 0..tile_map.width as i32 {
        for x in 0..tile_map.width as i32 {
            if let Some(tile_type) = tile_map.get_tile(x, y) {
                let rect = match tile_type {
                    TileType::Land => land_tile,
                    TileType::Sea => water_tile,
                    TileType::Beach => beach_tile,
                };
                draw_texture_ex(
                    *texture,
                    x as f32 * tile_len,
                    y as f32 * tile_len,
                    WHITE,
                    DrawTextureParams {
                        source: Some(rect),
                        dest_size: Some(vec2(tile_len, tile_len)),
                        ..Default::default()
                    },
                );
            }
        }
    }
}
