# Basic Transaction Handler Microservice in Rust

A Rust-based transaction handler microservice for DeFi applications.

## Iterative Development Plan

This project will be developed in iterations, starting with core logic and progressively adding web capabilities and other features.

### Iteration 1: Core Logic (Command-Line Application)

*   **Goal:** Implement the fundamental transaction processing logic as a simple command-line program.
*   **Key Components & Learnings:**
    *   **Structs:** Define `Transaction`, `Account` (or `UserState`), and `TransactionError` enum.
        *   `Transaction`: `sender_id: String`, `receiver_id: String`, `amount: u64`, `nonce: u32`
        *   `Account`: `id: String`, `balance: u64`, `current_nonce: u32`
        *   `TransactionError`: e.g., `InsufficientFunds`, `InvalidNonce`, `AccountNotFound`
    *   **Enums & Pattern Matching:** Use `TransactionError` with `Result` for function return types. Use `match` for handling outcomes.
    *   **Functions & Modules:**
        *   `validate_transaction(tx: &Transaction, sender_account: &Account) -> Result<(), TransactionError>`
        *   `execute_transaction(tx: &Transaction, sender: &mut Account, receiver: &mut Account) -> Result<(), TransactionError>`
        *   Organize into a `core_logic` module (e.g., `src/core_logic.rs`).
    *   **State Management (Simple):** Use `std::collections::HashMap` to store `Account` states in memory.
    *   **`main.rs` (CLI version):** Create sample transactions, fetch/update accounts from the `HashMap`, call validation/execution functions, and print results.
    *   **Testing:** Write basic unit tests for `validate_transaction` and `execute_transaction` using `cargo test`.

### Iteration 2: Introducing the Web Server & JSON Handling

*   **Goal:** Expose the core logic from Iteration 1 via an HTTP endpoint.
*   **Key Learnings & Technologies:**
    *   **Refactor to Library (Prerequisite):** Before adding web server code, ensure the core transaction logic (structs, enums, validation, execution functions from Iteration 1) is organized into a library crate structure. This typically means moving it from `src/main.rs` to `src/lib.rs` and modules like `src/core_logic.rs`. `src/main.rs` will then become the entry point for the web server and will use the library.
    *   **Async/Await, Tokio:** Basics of `async fn` and `#[tokio::main]`.
    *   **Web Framework (`actix-web` or `axum`):**
        *   Set up a basic server.
        *   Define a route (e.g., `/submit_transaction`).
        *   Write an `async` handler function for the route.
    *   **`serde` (Serialization/Deserialization):**
        *   Add `#[derive(Serialize, Deserialize)]` to `Transaction`, `Account`, and any request/response structs.
        *   Deserialize JSON from request body into `Transaction` struct.
        *   Serialize `Result` or a dedicated response struct into JSON for the response.
    *   **Shared State:** Address safe sharing of the `HashMap` of accounts across `async` requests (e.g., using `Arc<Mutex<HashMap<...>>>` or framework-specific data extractors).

### Iteration 3 & Beyond: Refinements & Further Features

*   **Concurrent State Management:** Implement robust sharing of state (e.g., `Arc<Mutex<...>>`).
*   **Logging:** Integrate `tracing` or `log` for observing application flow.
*   **Error Handling:** Map `TransactionError` variants and other errors to appropriate HTTP status codes and response bodies.
*   **Event Ingestion (Simulated):** Add an endpoint like `/submit_event` to simulate ingestion of smart contract events, potentially updating application state based on these events.
*   **Frontend Integration Sketch:** Document or sketch how a frontend would interact with the defined API endpoints.
*   **Browser-Based IDE Dry Run:** Ensure the project can be run and tested in a constrained environment like GitHub Codespaces.
*   **README Polish:** Keep this README updated with progress, setup instructions, and API documentation as it evolves.
*   **Edge Case Handling:** Consider and implement handling for various edge cases in transaction processing.
