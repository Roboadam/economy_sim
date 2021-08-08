use crate::business::Business;

pub struct AiPerson {
    pub cash: f32,
    pub hunger: f32,
    pub expected_price: f32,
    pub food_from_meal: f32,
}

impl AiPerson {
    pub fn will_buy(&self, business: &Business) -> bool {
        let cost_of_meal = business.price;
        if cost_of_meal > self.cash {
            return false;
        }

        if business.price < self.expected_price {
            return true;
        }

        if self.hunger < 0.2 {
            return true;
        }

        false
    }

    pub fn buy(&mut self, business: &mut Business) {
        if self.will_buy(business) {
            business.buy_widget();
            self.cash -= business.price;
            self.hunger += 1. / 3.;
            let price_diff = business.price - self.expected_price;
            self.expected_price += price_diff / 3.;
        }
    }
}
