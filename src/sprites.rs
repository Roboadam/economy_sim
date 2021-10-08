use std::path::Path;

use macroquad::prelude::{load_texture, Texture2D};

#[derive(Default)]
pub struct Sprites(Vec<Texture2D>);

impl Sprites {
    pub async fn add_sprite_from_path(&mut self, path: &str) -> usize {
        let texture = load_texture(path).await.unwrap();
        let index = self.0.len();
        self.0.push(texture);
        index
    }

    pub fn texture(&self, texture_index: usize) -> &Texture2D {
        self.0.get(texture_index).expect("Oh no!")
    }
}
