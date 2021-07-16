pub struct Business {
    cash: f32,
    num_widgets: i32,
    price: f32,
    building_id: i32,
    name: String,
}

impl Business {
    pub fn default() -> Self {
        Business {
            cash: 0.,
            num_widgets: 0,
            price: 0.,
            building_id: 0,
            name: "".to_string(),
        }
    }
}