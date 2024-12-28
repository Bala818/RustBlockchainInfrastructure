use crate::system::Config as SystemConfig;
use num::traits::{CheckedAdd, CheckedSub, Zero};
use std::collections::BTreeMap;
use std::fmt::Debug;

pub trait Config: SystemConfig {
    type Balance: Zero + CheckedSub + CheckedAdd + Copy + Debug;
}

#[derive(Debug)]
pub struct Balances<T: Config> {
    balances: BTreeMap<T::AccountId, T::Balance>,
}

impl<T: Config> Balances<T> {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    pub fn dispatch(
        &mut self,
        caller: T::AccountId,
        call: Call<T>,
    ) -> Result<(), &'static str> {
        match call {
            Call::Transfer { to, amount } => self.transfer(caller, to, amount),
        }
    }

    pub fn transfer(
        &mut self,
        caller: T::AccountId,
        to: T::AccountId,
        amount: T::Balance,
    ) -> Result<(), &'static str> {
        let caller_balance = self.get_balance(&caller);
        let to_balance = self.get_balance(&to);

        let new_caller_balance = caller_balance.checked_sub(&amount).ok_or("Not enough funds.")?;
        let new_to_balance = to_balance.checked_add(&amount).ok_or("Overflow")?;

        self.balances.insert(caller, new_caller_balance);
        self.balances.insert(to, new_to_balance);

        Ok(())
    }

    pub fn get_balance(&self, account: &T::AccountId) -> T::Balance {
        *self.balances.get(account).unwrap_or(&T::Balance::zero())
    }
}

#[derive(Debug)]
pub enum Call<T: Config> {
    Transfer {
        to: T::AccountId,
        amount: T::Balance,
    },
}
