
# RustBlockchainInfrastructure Executing Blocks and Dispatching Calls

## Overview
This project implements a blockchain infrastructure in Rust, focusing on modularity, flexibility, and dynamic transaction management. It includes components for managing system state, user balances, and transaction dispatching. The project showcases a step-by-step implementation of Rust's generics, traits, and runtime configuration.

---

## Features

### 1. **Support Module**
- Defines blockchain structures:
  - **Header**: Stores metadata like block numbers.
  - **Block**: Encapsulates headers and extrinsics (transactions).
  - **Extrinsic**: Represents individual transactions with a caller and call details.
- Provides a foundation for blockchain functionality.

### 2. **System Pallet**
- Manages:
  - **Block Numbers**: Tracks the current blockchain state.
  - **Nonces**: Ensures unique transaction identifiers for each account.
- Implements generics for flexible configuration:
  - `AccountId`: User identifier.
  - `BlockNumber`: Represents blockchain blocks.
  - `Nonce`: Transaction counters for accounts.

### 3. **Balances Pallet**
- Manages:
  - **User Balances**: Tracks balances for each account.
  - **Transfers**: Securely transfers funds between accounts with error handling.
- Supports generics for type flexibility:
  - `AccountId`: User identifier.
  - `Balance`: Numeric balance for each account.

### 4. **Runtime**
- Integrates:
  - **System Pallet**: For state management (blocks and nonces).
  - **Balances Pallet**: For user account management and transfers.
- Provides a single interface to:
  - Execute blocks.
  - Dispatch transactions to appropriate pallets.
- Implements block validation, dispatching, and execution.

### 5. **Dynamic Dispatching**
- Implements nested dispatch for managing runtime calls dynamically.
- Routes calls to appropriate pallets using enums and match statements.
- Supports flexible, modular function execution within the runtime.

---

## How to Run

### Prerequisites
- Install Rust: [Rust Installation](https://www.rust-lang.org/tools/install)

### Steps
1. **Clone the Repository**:
   ```
   git clone <repository_url>
   cd RustBlockchainInfrastructure
   ```

2. **Build and Run**:
   ```
   cargo run
   ```

3. **Run Tests**:
   ```
   cargo test
   ```

---

## Example Output

### Running the Program
The program simulates blockchain transactions with block validation and dynamic dispatching.

```yaml
Transaction successful! Alice sent 300 to Bob.
Current Runtime State:
Runtime {
    system: System {
        block_number: 1,
        transaction_count: {"Alice": 1},
    },
    balances: Balances {
        balances: {"Alice": 700, "Bob": 800},
    },
}
Transaction successful! Bob sent 200 to Alice.
Current Runtime State:
Runtime {
    system: System {
        block_number: 2,
        transaction_count: {"Alice": 1, "Bob": 1},
    },
    balances: Balances {
        balances: {"Alice": 900, "Bob": 600},
    },
}
```

## Highlights

### **Modularity**
- Separate modules for `System`, `Balances`, and `Support` ensure clear and reusable code.

### **Type Flexibility**
- Uses Rust's generics and traits for configurable types like `AccountId`, `BlockNumber`, and `Balance`.

### **Dynamic Dispatching**
- Supports runtime-level transaction routing for efficient and extensible function execution.

### **Scalability**
- Structured to add new pallets and functionalities with minimal changes.

