use macroquad::prelude::*;

use crate::{
    tile_map::{TileMap, TileType},
    tile_selector::TileSelector,
};

pub fn pixel_perfect_render_target() -> RenderTarget {
    let rt = render_target(2048, 2048);
    rt.texture.set_filter(FilterMode::Nearest);
    rt
}

pub fn screen_dimension_in_tiles(tile_width: f32) -> (i32, i32) {
    let height = screen_height() / tile_width;
    let width = screen_width() / tile_width;
    (width as i32, height as i32)
}

pub fn player_coords_to_target(coords: (f32, f32), tile_width: f32) -> Vec2 {
    vec2(coords.0 * tile_width, coords.1 * tile_width)
}

pub fn draw_tile_map(
    tile_map: &TileMap,
    tile_width: f32,
    texture_atlas: &Texture2D,
    player_coords: (f32, f32),
    screen_dimensions: (i32, i32),
) {
    let min_x = player_coords.0 as i32 - screen_dimensions.0 / 2;
    let min_y = player_coords.1 as i32 - screen_dimensions.1 / 2;
    let max_x = min_x + screen_dimensions.0 + 1;
    let max_y = min_y + screen_dimensions.1 + 1;
    // println!("{}, {}, {}, {}", min_x, min_y, max_x, max_y);

    let tile_selector = TileSelector::new();
    for y in min_y..max_y {
        for x in min_x..max_x {
            draw_texture_ex(
                *texture_atlas,
                x as f32 * tile_width,
                y as f32 * tile_width,
                WHITE,
                texture_params(x, y, tile_width, tile_map, &tile_selector),
            );
        }
    }
}

pub fn draw_to_texture(texture: RenderTarget, player_coords: (f32, f32), tile_width: f32) {
    set_camera(&Camera2D {
        // zoom: vec2(0.008, 0.008),
        target: player_coords_to_target(player_coords, tile_width),
        render_target: Some(texture),
        ..Default::default()
    });
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

pub fn draw_texture_to_screen(texture: RenderTarget) {
    set_default_camera();
    clear_background(WHITE);
    let max_width = if screen_height() > screen_width() {
        screen_height()
    } else {
        screen_width()
    };
    draw_texture_ex(
        texture.texture,
        0.,
        0.,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(max_width, max_width)),
            ..Default::default()
        },
    );
}

pub async fn open_pixel_texture(path: &str) -> Texture2D {
    let texture_atlas = load_texture(path).await.unwrap();
    texture_atlas.set_filter(FilterMode::Nearest);

    texture_atlas
}
