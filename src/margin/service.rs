use rust_decimal::Decimal;
use serde::Serialize;
use std::collections::HashMap;
use uuid::Uuid;

pub struct MarginService {
    balances: HashMap<Uuid, Balance>,
}

struct Balance {
    available: Decimal,
    locked: Decimal,
}

impl MarginService {
    pub fn new() -> Self {
        Self {
            balances: HashMap::new(),
        }
    }

    pub fn deposit(&mut self, user_id: Uuid, amount: Decimal) -> Result<(), UserBalanceError> {
        if amount <= Decimal::ZERO {
            return Err(UserBalanceError::InvalidAmount);
        }
        let balance = self.balances.entry(user_id).or_insert(Balance {
            available: Decimal::ZERO,
            locked: Decimal::ZERO,
        });
        balance.available += amount;
        Ok(())
    }

    pub fn reserve(&mut self, user_id: Uuid, amount: Decimal) -> Result<(), UserBalanceError> {
        if amount <= Decimal::ZERO {
            return Err(UserBalanceError::InvalidAmount);
        }
        let user = self
            .balances
            .get_mut(&user_id)
            .ok_or(UserBalanceError::UserNotFound)?;
        if user.available < amount {
            return Err(UserBalanceError::InsufficientAvailableAmount);
        }
        user.available -= amount;
        user.locked += amount;
        Ok(())
    }

    pub fn release(&mut self, user_id: Uuid, amount: Decimal) -> Result<(), UserBalanceError> {
        if amount <= Decimal::ZERO {
            return Err(UserBalanceError::InvalidAmount);
        }
        let user = self
            .balances
            .get_mut(&user_id)
            .ok_or(UserBalanceError::UserNotFound)?;
        if user.locked < amount {
            return Err(UserBalanceError::InsufficientLockedAmount);
        }
        user.locked -= amount;
        user.available += amount;
        Ok(())
    }

    pub fn get_balance(&self, user_id: Uuid) -> Result<BalanceView, UserBalanceError> {
        let user = self
            .balances
            .get(&user_id)
            .ok_or(UserBalanceError::UserNotFound)?;
        Ok(BalanceView {
            available: user.available,
            locked: user.locked,
            total: user.available + user.locked,
        })
    }

    pub fn check_balance(&self, user_id: Uuid, amount: Decimal) -> Result<bool, &str> {
        let user = self.balances.get(&user_id);
        match user {
            Some(balance) => {
                if balance.available >= amount {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            None => Err("Invalid User ID"),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct BalanceView {
    pub available: Decimal,
    pub locked: Decimal,
    pub total: Decimal,
}

#[derive(Serialize, Debug)]
pub enum UserBalanceError {
    InsufficientAvailableAmount,
    InsufficientLockedAmount,
    UserNotFound,
    InvalidAmount,
}
