use macroquad::prelude::*;

pub async fn open_pixel_texture(path: &str) -> Texture2D {
    let texture_atlas = load_texture(path).await.unwrap();
    texture_atlas.set_filter(FilterMode::Nearest);

    texture_atlas
}
