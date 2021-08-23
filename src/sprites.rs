use macroquad::prelude::Texture2D;

use crate::rendering::open_pixel_texture;

pub struct Sprite(Texture2D);

pub struct SpritePool(Vec<Sprite>);

impl SpritePool {
    pub async fn add(&mut self, file_path: &str) -> i32 {
        let texture = open_pixel_texture(file_path).await;
        self.0.push(Sprite(texture));
        (self.0.len() - 1) as i32
    }

    pub fn get(&self, index: i32) -> Option<&Sprite> {
        self.get(index)
    }
}
