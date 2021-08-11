use std::collections::HashMap;

use crate::{business::BusinessId, person::PersonId};

pub struct Money(HashMap<i32, f32>);

pub enum TransferError {
    InsufficientFunds,
}

impl Money {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn transfer(
        &mut self,
        from: PersonId,
        to: BusinessId,
        cash: f32,
    ) -> Result<(), TransferError> {
        let from_funds = self.funds(from.0);
        if from_funds >= cash {
            return Err(TransferError::InsufficientFunds);
        }

        let to_funds = self.funds(to.0);

        self.0.insert(from.0, from_funds - cash);
        self.0.insert(to.0, to_funds + cash);

        Ok(())
    }

    pub fn funds(&self, entity: i32) -> f32 {
        *self.0.get(&entity).unwrap_or(&0.)
    }

    pub fn create_cash(&mut self, entity: i32, cash: f32) {
        let entity_funds = self.funds(entity);
        self.0.insert(entity, entity_funds + cash);
    }
}
