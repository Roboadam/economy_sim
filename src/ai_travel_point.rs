use crate::components::Position;

pub struct AiTravelPoint {
    pub position: Position,
    pub sprite: i32,
}

pub fn sample_travel_points(sprite: i32) -> Vec<AiTravelPoint> {
    vec![
        AiTravelPoint {
            position: Position(30., 20.),
            sprite: sprite,
        },
        AiTravelPoint {
            position: Position(40., 20.),
            sprite: sprite,
        },
        AiTravelPoint {
            position: Position(45., 30.),
            sprite: sprite,
        },
        AiTravelPoint {
            position: Position(35., 40.),
            sprite: sprite,
        },
        AiTravelPoint {
            position: Position(25., 30.),
            sprite: sprite,
        },
    ]
}

pub fn draw_travel_points(points: &Vec<AiTravelPoint>, sprites: &Sprites) {
    points.iter().for_each(|aitp| 
    draw_texture(
        player_texture,
        my_position.0 * TILE_WIDTH,
        my_position.1 * TILE_WIDTH,
        WHITE,
    );
}
