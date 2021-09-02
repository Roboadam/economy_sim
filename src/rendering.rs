use macroquad::prelude::*;

pub struct ScreenData {
    screen_dimensions: (i32, i32),
    render_target: RenderTarget,
    tiles_on_screen: i32,
    tile_width: f32,
}

impl ScreenData {
    pub fn new(
        tiles_on_screen: i32,
        tile_width: f32,
        screen_width: f32,
        screen_height: f32,
    ) -> Self {
        let mut result = Self {
            tiles_on_screen,
            tile_width,
            screen_dimensions: (0, 0),
            render_target: pixel_perfect_render_target((0, 0), tile_width),
        };
        result.update_with_screen_size(screen_width, screen_height);
        result
    }

    pub fn update_with_screen_size(&mut self, screen_width: f32, screen_height: f32) {
        self.screen_dimensions =
            screen_dimension_in_tiles(self.tiles_on_screen, screen_width, screen_height);
        self.render_target = pixel_perfect_render_target(self.screen_dimensions, self.tile_width);
    }
}

fn pixel_perfect_render_target(screen_dimensions: (i32, i32), tile_width: f32) -> RenderTarget {
    let width = screen_dimensions.0 as u32 * tile_width as u32;
    let height = screen_dimensions.1 as u32 * tile_width as u32;
    let rt = render_target(width, height);
    rt.texture.set_filter(FilterMode::Nearest);
    rt
}

fn screen_dimension_in_tiles(
    tiles_on_screen: i32,
    screen_width: f32,
    screen_height: f32,
) -> (i32, i32) {
    let aspect_ratio = screen_width / screen_height;
    if aspect_ratio > 1. {
        let width = (aspect_ratio * tiles_on_screen as f32).ceil();
        (width as i32, tiles_on_screen)
    } else {
        let height = (1. / aspect_ratio * tiles_on_screen as f32).ceil();
        (tiles_on_screen, height as i32)
    }
}

pub async fn open_pixel_texture(path: &str) -> Texture2D {
    let texture_atlas = load_texture(path).await.unwrap();
    texture_atlas.set_filter(FilterMode::Nearest);

    texture_atlas
}
