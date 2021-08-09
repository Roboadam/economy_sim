use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    money::{Money, TransferError::InsufficientFunds},
    person::People,
};

#[derive(Serialize, Deserialize)]
pub struct Business {
    pub num_widgets: i32,
    pub price: f32,
    pub name: String,
}

pub struct Businesses(HashMap<i32, Business>);

impl Businesses {
    pub fn get(&self, id: i32) -> &Business {
        self.0
            .get(&id)
            .expect(&format!("Unknown business id {}", id))
    }

    pub fn get_mut(&mut self, id: i32) -> &mut Business {
        self.0
            .get_mut(&id)
            .expect(&format!("Unknown business id {}", id))
    }

    pub fn new(data: HashMap<i32, Business>) -> Self {
        Self(data)
    }
}

pub fn widget_transaction(
    business_id: i32,
    person_id: i32,
    businesses: &mut Businesses,
    people: &mut People,
    money: &mut Money,
) {
    let business = businesses.get_mut(business_id);
    let person = people.get_mut(person_id);
    if business.num_widgets < 1 {
        println!("Not enough widgets to sell");
        return;
    }
    match money.transfer(business_id, person_id, business.price) {
        Ok(_) => {
            business.sell_widget();
            person.buy();
        }
        Err(InsufficientFunds) => println!("Insufficent funds!"),
    }
}

impl Business {
    pub fn sell_widget(&mut self) {
        self.num_widgets -= 1;
        let orig_price = self.price;
        self.price += self.price / 100.;
    }
}
