use std::collections::BTreeMap;
use std::fmt::Debug;

pub trait Config {
    type AccountId: Clone + Ord + Debug;
    type BlockNumber: Default
        + Copy
        + PartialEq
        + std::ops::Add<Output = Self::BlockNumber>
        + From<u32>
        + Debug;
    type Nonce: Default + Copy + std::ops::AddAssign + From<u32> + Debug;
}

#[derive(Debug)]
pub struct System<T: Config> {
    pub block_number: T::BlockNumber,
    pub nonces: BTreeMap<T::AccountId, T::Nonce>,
}

impl<T: Config> System<T> {
    pub fn new() -> Self {
        Self {
            block_number: T::BlockNumber::default(),
            nonces: BTreeMap::new(),
        }
    }

    pub fn increment_block(&mut self) {
        self.block_number = self.block_number + T::BlockNumber::from(1);
    }

    pub fn get_nonce(&self, account: &T::AccountId) -> T::Nonce {
        *self.nonces.get(account).unwrap_or(&T::Nonce::default())
    }

    pub fn increment_nonce(&mut self, account: T::AccountId) {
        let nonce = self.nonces.entry(account).or_insert(T::Nonce::default());
        *nonce += T::Nonce::from(1);
    }
}
