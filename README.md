# Basic Transaction Handler Microservice in Rust

A Rust-based transaction handler microservice for DeFi applications.

> Project Context: I created this project as a refresher after spending over a year focused on AI development in Python. It serves as a way to get back into Rust programming while building something practical and interesting in the Web3 space.

> Development Philosophy: As noted on corrode.dev - "The beauty of prototyping in Rust is that your 'rough drafts' have the same memory safety and performance as polished code. Even when I liberally use unwrap(), stick everything in main.rs, and reach for owned types everywhere, the resulting code is on-par with a Python prototype in reliability, but outperforms it easily. This makes it perfect for experimenting with real-world workloads, even before investing time in proper error handling."

## Iterative Development Plan

This project will be developed in iterations, starting with core logic and progressively adding web capabilities and other features.

### ✅ Iteration 1: Core Logic (Command-Line Application)

*   **Goal:** Implement the fundamental transaction processing logic as a simple command-line program.
*   **Key Components & Learnings:**
    *   **Structs:** Define `Transaction`, `Account`, and `TransactionError` enum.
        *   `Transaction`: `sender: String`, `receiver: String`, `amount: u64`, `nonce: u32`
            > Note: For this MVP, we've simplified the transaction format to focus on the core logic. In a production environment, this would be extended to handle standard Ethereum transaction objects from web3.js or ethers.js, including proper hex addresses, signatures, and gas parameters.
        *   `Account`: `balance: u64`, `nonce: u32` (ID stored as HashMap key)
        *   `TransactionError`: `AmountIsZero`, `SenderIsReceiver`, `AccountNotFound`, `InsufficientFunds`, `InvalidNonce`
    *   **Transaction Validation Rules:**
        *   Amount must be greater than zero
        *   Sender and receiver must be different accounts
        *   Sender account must exist
        *   Sender must have sufficient funds
        *   Transaction nonce must match sender's current nonce
    *   **Auto Account Creation:** Receiver accounts are automatically created if they don't exist
    *   **Function:**
        *   `handle_transaction(tx: &Transaction, accts: &mut AccountStore) -> Result<(), TransactionError>`
            - Validates transaction
            - Updates sender and receiver balances
            - Increments sender's nonce
    *   **State Management:** Use `AccountStore` type alias for `HashMap<String, Account>` to store account states in memory.

### ✅ Iteration 2: Introducing the Web Server & JSON Handling

*   **Goal:** Expose the core logic from Iteration 1 via an HTTP endpoint.
*   **Key Learnings & Technologies:**
    *   **Async/Await, Tokio:** Basics of `async fn` and `#[tokio::main]`.
    *   **Web Framework (`axum`):**
        *   Set up a basic server on `127.0.0.1:3000`
        *   Define POST route `/submit_transaction`
        *   Write an `async` handler function for the route
    *   **`serde` (Serialization/Deserialization):**
        *   Add `#[derive(Serialize, Deserialize)]` to `Transaction`, `Account`, and response structs
        *   Deserialize JSON from request body into `Transaction` struct
        *   Serialize `TxResponse` (with status and message) for responses
    *   **Shared State:** Using `Arc<Mutex<HashMap>>` for thread-safe account storage
    *   **Testing:** Use curl to test the endpoint:
        ```bash
        curl -X POST -H "Content-Type: application/json" \
        -d '{"sender": "Alice", "receiver":"Bob", "amount":100, "nonce":0}' \
        http://127.0.0.1:3000/submit_transaction
        ```

### ⬜ Iteration 3 & Beyond: Refinements & Further Features

*   **Concurrent State Management:** Implement robust sharing of state (e.g., `Arc<Mutex<...>>`).
*   **Logging:** Integrate `tracing` or `log` for observing application flow.
*   **Error Handling:** Map `TransactionError` variants and other errors to appropriate HTTP status codes and response bodies.
*   **Event Ingestion (Simulated):** Add an endpoint like `/submit_event` to simulate ingestion of smart contract events, potentially updating application state based on these events.
*   **Frontend Integration Sketch:** Document or sketch how a frontend would interact with the defined API endpoints.
*   **Browser-Based IDE Dry Run:** Ensure the project can be run and tested in a constrained environment like GitHub Codespaces.
*   **README Polish:** Keep this README updated with progress, setup instructions, and API documentation as it evolves.
*   **Edge Case Handling:** Consider and implement handling for various edge cases in transaction processing.
