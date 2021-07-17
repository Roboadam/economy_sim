pub struct Business {
    pub cash: f32,
    pub num_widgets: i32,
    pub price: f32,
    pub building_id: i32,
    pub name: String,
}

impl Business {
    pub fn new(name: &str, building_id: i32) -> Self {
        Business {
            cash: 0.,
            num_widgets: 0,
            price: 0.,
            building_id: building_id,
            name: name.to_owned(),
        }
    }
}
