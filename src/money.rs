use std::collections::HashMap;

pub struct Money(HashMap<i32, f32>);

pub enum TransferError {
    InsufficientFunds,
}

impl Money {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn transfer(&mut self, from: i32, to: i32, cash: f32) -> Result<(), TransferError> {
        let from_funds = self.funds(from);
        if from_funds >= cash {
            return Err(TransferError::InsufficientFunds);
        }

        let to_funds = self.funds(to);

        self.0.insert(from, from_funds - cash);
        self.0.insert(to, to_funds + cash);

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
