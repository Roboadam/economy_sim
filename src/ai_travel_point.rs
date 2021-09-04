use macroquad::prelude::WHITE;
use macroquad::texture::draw_texture;

use crate::components::Position;
use crate::sprites::SpritePool;

pub struct AiTravelPoint {
    pub position: Position,
    pub sprite: i32,
}

pub fn sample_travel_points(sprite: i32) -> Vec<AiTravelPoint> {
    vec![
        AiTravelPoint {
            position: Position(200., 20.),
            sprite: sprite,
        },
        AiTravelPoint {
            position: Position(300., 20.),
            sprite: sprite,
        },
        AiTravelPoint {
            position: Position(245., 130.),
            sprite: sprite,
        },
        AiTravelPoint {
            position: Position(135., 240.),
            sprite: sprite,
        },
        AiTravelPoint {
            position: Position(25., 130.),
            sprite: sprite,
        },
    ]
}

pub fn draw_travel_points(points: &Vec<AiTravelPoint>, sprites: &SpritePool) {
    points.iter().for_each(|aitp| {
        if let Some(texture) = sprites.get(aitp.sprite) {
            draw_texture(texture.0, aitp.position.0, aitp.position.1, WHITE);
        }
    });
}
