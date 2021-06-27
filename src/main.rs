use crate::land_mass_generator::create_land_mass;
use macroquad::prelude::*;
use selection::Selection;
use tile_map::{TileMap, TileType};
use TileType::{Land, Sea};

mod land_mass_generator;
mod selection;
mod tile_map;

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
    for y in 0..tile_map.width as i32 {
        for x in 0..tile_map.width as i32 {
            draw_texture_ex(
                *texture,
                x as f32 * tile_len,
                y as f32 * tile_len,
                WHITE,
                texture_params(x, y, tile_len, tile_map),
            );
        }
    }
}

fn texture_coordinates(
    me: &TileType,
    up: &TileType,
    down: &TileType,
    left: &TileType,
    right: &TileType,
) -> (i32, i32) {
    match me {
        Land => match up {
            Land => match down {
                Land => match left {
                    Land => match right {
                        // me land up land down land left land right land
                        Land => (2, 2),
                        // me land up land down land left land right sea
                        Sea => (3, 2),
                    },
                    Sea => match right {
                        // me land up land down land left sea right land
                        Land => (1, 2),
                        // me land up land down land left sea right sea
                        Sea => (9, 2),
                    },
                },
                Sea => match left {
                    Land => match right {
                         // me land up land down sea left land right land
                        Land => (2, 3),
                         // me land up land down sea left land right sea
                        Sea => (3, 3),
                    },
                    Sea => match right {
                        // me land up land down sea left sea right land
                        Land => (1, 3),
                        // me land up land down sea left sea right sea
                        Sea => (6, 3),
                    },
                },
            },
            Sea => match down {
                Land => match left {
                    Land => match right {
                        // me land up sea down land left land right land
                        Land => (2, 1),
                        // me land up sea down land left land right sea
                        Sea => (3, 1),
                    },
                    Sea => match right {
                        // me land up sea down land left sea right land
                        Land => (1, 1),
                        // me land up sea down land left sea right sea
                        Sea => (6, 1),
                    },
                },
                Sea => match left {
                    Land => match right {
                        // me land up sea down sea left land right land
                        Land => (6, 5),
                        // me land up sea down sea left land right sea
                        Sea => (7, 5),
                    },
                    Sea => match right {
                        // me land up sea down sea left sea right land
                        Land => (5, 5),
                        // me land up sea down sea left sea right sea
                        Sea => (2, 2),
                    },
                },
            },
        },
        Sea => match up {
            Land => match down {
                Land => (0, 0),
                Sea => match left {
                    Land => match right {
                         // me sea up land down sea left land right land
                        Land => (0, 0),
                         // me sea up land down sea left land right sea
                        Sea => (7, 3),
                    },
                    Sea => match right {
                        // me sea up land down sea left sea right land
                        Land => (5, 3),
                        // me sea up land down sea left sea right sea
                        Sea => (0, 0),
                    },
                },
            },
            Sea => match down {
                Land => match left {
                    Land => match right {
                        // me sea up sea down land left land right land
                        Land => (0, 0),
                        // me sea up sea down land left land right sea
                        Sea => (7, 1),
                    },
                    Sea => match right {
                        // me sea up sea down land left sea right land
                        Land => (5, 1),
                        // me sea up sea down land left sea left land
                        Sea => (0, 0),
                    },
                },
                Sea => (0, 0),
            },
        },
    }
}

fn texture_params(x: i32, y: i32, tile_len: f32, tile_map: &TileMap) -> DrawTextureParams {
    const TILE_PIXEL_LEN: f32 = 16f32;
    let my_tile_type = tile_map.get_tile(x, y).unwrap();
    if *my_tile_type == Land {
        return DrawTextureParams {
            source: Some(Rect {
                x: TILE_PIXEL_LEN,
                y: 0.,
                w: TILE_PIXEL_LEN,
                h: TILE_PIXEL_LEN,
            }),
            dest_size: Some(vec2(tile_len, tile_len)),
            ..Default::default()
        };
    }

    let up_tile_land = {
        if let Some(tile_type) = tile_map.get_tile(x, y - 1) {
            *tile_type == Land
        } else {
            false
        }
    };

    let down_tile_land = {
        if let Some(tile_type) = tile_map.get_tile(x, y + 1) {
            *tile_type == Land
        } else {
            false
        }
    };

    let left_tile_land = {
        if let Some(tile_type) = tile_map.get_tile(x - 1, y) {
            *tile_type == Land
        } else {
            false
        }
    };

    let right_tile_land = {
        if let Some(tile_type) = tile_map.get_tile(x + 1, y) {
            *tile_type == Land
        } else {
            false
        }
    };

    if !up_tile_land && !down_tile_land && !left_tile_land && !right_tile_land {
        return DrawTextureParams {
            source: Some(Rect {
                x: 0.,
                y: 0.,
                w: TILE_PIXEL_LEN,
                h: TILE_PIXEL_LEN,
            }),
            dest_size: Some(vec2(tile_len, tile_len)),
            ..Default::default()
        };
    }

    if !up_tile_land && !down_tile_land && !left_tile_land && right_tile_land {
        return DrawTextureParams {
            source: Some(Rect {
                x: TILE_PIXEL_LEN * 2.,
                y: 0.,
                w: TILE_PIXEL_LEN,
                h: TILE_PIXEL_LEN,
            }),
            dest_size: Some(vec2(tile_len, tile_len)),
            rotation: 3. * std::f32::consts::PI / 2.,
            ..Default::default()
        };
    }

    DrawTextureParams {
        source: Some(Rect {
            x: 0.,
            y: 0.,
            w: TILE_PIXEL_LEN,
            h: TILE_PIXEL_LEN,
        }),
        dest_size: Some(vec2(tile_len, tile_len)),
        rotation: 3. * std::f32::consts::PI / 2.,
        ..Default::default()
    }
}
