use crate::business::Business;

pub struct AiPerson {
    pub cash: f32,
    pub hunger: f32,
    pub food_from_meal: f32,
}

impl AiPerson {
    pub fn will_buy(&self, business: &Business) -> bool {
        let cost_of_meal = business.price;
        if cost_of_meal > self.cash {
            return false;
        }

        let buy_it_price = self.cash / 2000.;
        if business.price < buy_it_price {
            return true;
        }

        let max_price = self.cash / 21.;
        
        if self.hunger < 0.2  && business.price < max_price {
            return true;
        }

        false
    }
}
