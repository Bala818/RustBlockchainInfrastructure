
use crate::support::{Block, Extrinsic};
use crate::system::System;
use crate::balances::{self, Balances, Call as BalancesCall};


#[derive(Debug)]
pub struct Runtime<T: crate::system::Config + balances::Config> {
    pub system: System<T>,
    pub balances: Balances<T>,
}

impl<T: crate::system::Config + balances::Config> Runtime<T> {
    pub fn new() -> Self {
        Self {
            system: System::new(),
            balances: Balances::new(),
        }
    }

    pub fn execute_block(
        &mut self,
        block: Block<T::BlockNumber, Extrinsic<T::AccountId, BalancesCall<T>>>,
    ) {
        if block.header.block_number != self.system.block_number + T::BlockNumber::from(1) {
            panic!("Invalid block number!");
        }

        self.system.increment_block();

        for extrinsic in block.extrinsics {
            self.system.increment_nonce(extrinsic.caller.clone());
            if let Err(e) = self.balances.dispatch(extrinsic.caller, extrinsic.call) {
                eprintln!("Dispatch error: {}", e);
            }
        }
    }
}
