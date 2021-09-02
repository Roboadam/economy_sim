use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    money::{Money, TransferError::InsufficientFunds},
    person::{People, PersonId},
};

#[derive(Serialize, Deserialize)]
pub struct Business {
    pub num_widgets: i32,
    pub price: f32,
    pub name: String,
}

#[derive(PartialEq, Eq, Hash, Debug, Serialize, Deserialize, Clone, Copy)]
pub struct BusinessId(pub i32);

pub struct Businesses(HashMap<BusinessId, Business>);

impl Businesses {
    pub fn get_mut(&mut self, id: BusinessId) -> &mut Business {
        self.0
            .get_mut(&id)
            .expect(&format!("Unknown business id {:?}", id))
    }

    pub fn new(data: HashMap<BusinessId, Business>) -> Self {
        Self(data)
    }
}

pub fn widget_transaction(
    business_id: BusinessId,
    person_id: PersonId,
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
    match money.transfer(person_id, business_id, business.price) {
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
        self.price += self.price / 100.;
    }
}
