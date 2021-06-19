use std::collections::HashSet;

use macroquad::prelude::*;
use rules::{Direction, Rule};
use selection::Selection;
use super_position::SuperPosition;
use tile_map::{TileMap, TileType};

mod rules;
mod selection;
mod super_position;
mod tile_map;

#[macroquad::main("Texture")]
async fn main() {
    let texture: Texture2D = load_texture("textures/land_tilemap.png").await.unwrap();
    texture.set_filter(FilterMode::Nearest);
    const TILES_PER_SIDE: i32 = 16;

    let mut selection = Selection::new(TILES_PER_SIDE);

    let mut tile_map = TileMap::new(TILES_PER_SIDE);

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
            let (rule_set, super_position) = collect_rules_and_super_position(&tile_map);
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

    for y in 0..tile_map.width {
        for x in 0..tile_map.width {
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

fn collect_rules_and_super_position(tile_map: &TileMap) -> (HashSet<Rule>, SuperPosition) {
    let mut rule_set = HashSet::new();
    let mut super_position = SuperPosition::new();
    for y in 0..tile_map.width {
        for x in 0..tile_map.width {
            if let Some(current_tile) = tile_map.get_tile(x, y) {
                if let Some(value) = super_position.get_mut(current_tile) {
                    *value += 1;
                } else {
                    super_position.insert(current_tile.clone(), 1);
                }
                if let Some(up_tile) = tile_map.get_tile(x, y - 1) {
                    rule_set.insert(Rule(*current_tile, *up_tile, Direction::Up));
                }
                if let Some(down_tile) = tile_map.get_tile(x, y + 1) {
                    rule_set.insert(Rule(*current_tile, *down_tile, Direction::Down));
                }
                if let Some(left_tile) = tile_map.get_tile(x - 1, y) {
                    rule_set.insert(Rule(*current_tile, *left_tile, Direction::Left));
                }
                if let Some(right_tile) = tile_map.get_tile(x + 1, y) {
                    rule_set.insert(Rule(*current_tile, *right_tile, Direction::Right));
                }
            }
        }
    }
    (rule_set, super_position)
}
