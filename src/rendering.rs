use macroquad::prelude::*;

use crate::{tile_map::{TileMap, TileType}, tile_selector::TileSelector};

pub fn pixel_perfect_render_target() -> RenderTarget {
    let rt = render_target(2048, 2048);
    rt.texture.set_filter(FilterMode::Nearest);
    rt
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

pub fn draw_to_texture(texture: RenderTarget, target: Vec2) {
    set_camera(&Camera2D {
        zoom: vec2(0.008, 0.008),
        target: target,
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

pub async fn open_texture_atlas(path: &str) -> Texture2D {
    let texture_atlas = load_texture(path).await.unwrap();
    texture_atlas.set_filter(FilterMode::Nearest);

    texture_atlas
}
