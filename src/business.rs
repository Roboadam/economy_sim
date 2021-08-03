use serde::Serialize;

#[derive(Serialize)]
pub struct Business {
    pub cash: f32,
    pub num_widgets: i32,
    pub price: f32,
    pub building_id: i32,
    pub name: String,
}
