use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Business {
    pub cash: f32,
    pub num_widgets: i32,
    pub price: f32,
    pub name: String,
}
