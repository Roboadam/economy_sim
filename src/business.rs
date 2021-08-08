use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Business {
    pub cash: f32,
    pub num_widgets: i32,
    pub price: f32,
    pub name: String,
}

pub struct Purchase {
    pub cash: f32,
    pub num_items: i32,
}

impl Business {
    pub fn buy_widget(&mut self) -> Purchase {
        if self.num_widgets <= 0 {
            return Purchase {
                cash: 0.,
                num_items: 0,
            };
        }
        self.num_widgets -= 1;
        self.cash += self.price;
        let orig_price = self.price;
        self.price += self.price / 100.;
        Purchase {
            cash: orig_price,
            num_items: 1,
        }
    }
}
