mod balances;
mod runtime;
mod support;
mod system;

use crate::balances::Call as BalancesCall;
use crate::runtime::Runtime;
use crate::support::{Block, Extrinsic};

fn main() {
    let mut runtime = Runtime::<String, u128, u32, u32>::new();

    runtime.balances.add_user("Alice".to_string(), 1000);
    runtime.balances.add_user("Bob".to_string(), 500);

    let extrinsics = vec![
        Extrinsic {
            caller: "Alice".to_string(),
            call: BalancesCall::Transfer {
                to: "Bob".to_string(),
                amount: 300,
            },
        },
        Extrinsic {
            caller: "Alice".to_string(),
            call: BalancesCall::Transfer {
                to: "Bob".to_string(),
                amount: 200,
            },
        },
    ];

    let block = Block::new(1, extrinsics);

    runtime.execute_block(block);

    println!("{:#?}", runtime);
}
