use std::collections::HashMap;

use axum::{
    routing::post,
    Json, Router,
    extract::State,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::net::TcpListener;

use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)] 
struct Account {
    balance: u64,
    nonce: u32, 
}

#[derive(Debug, Clone, Deserialize)]
struct Transaction {
    sender: String,
    receiver: String,
    amount: u64,
    nonce: u32,
    // signature: String, // Omitted for simplicity in prototype.
}

#[derive(Debug)]
enum TransactionError {
    AccountNotFound, // Sender account doesn't exist
    AmountIsZero, // Transcation amount is zero
    SenderIsReceiver, // Sender and receiver are the same 
    InsufficientFunds, //  Sender has sufficient funds
    InvalidNonce, // Transaction's nonce isn't the sender's current nonce
}

#[derive(Debug, Serialize)]
struct TxResponse {
    status: String,
    message: String,
}

type AccountStore = HashMap<String, Account>;
type SharedAccountStore = Arc<Mutex<AccountStore>>;


// Function handles a single transaction, validating then updating account balances and nonces
// if valid, it updates the sender and receiver balances and increments the sender's nonce
// if the recewiver account doesn't exist, it's created with 0 balance and 0 nonce before receiving funds

fn handle_transaction(
    tx: &Transaction,
    accts: &mut AccountStore,
) -> Result<(), TransactionError> {

    // 1. Verify sender account exists by using get and unwrap before cloning it
   let mut sender_account_clone = accts.get(&tx.sender).unwrap().clone();

    // 2. Transaction amount is not zero
    if tx.amount == 0 {
        return Err(TransactionError::AmountIsZero);
    }

    // 3. validate sender isn't receiver
    if tx.sender == tx.receiver {
        return Err(TransactionError::SenderIsReceiver);
    }

    // 4. Sender has sufficient funds
    if sender_account_clone.balance < tx.amount {
        return Err(TransactionError::InsufficientFunds);
    }

    // 5. Transaction's nonce is the sender's current nonce
    if sender_account_clone.nonce != tx.nonce {
        return Err(TransactionError::InvalidNonce);
    }

    // It's Valid. 
    // // Update Sender bal
    sender_account_clone.balance -= tx.amount;
    // // Increment Sender Nonce
    sender_account_clone.nonce += 1;
    
    // // Update Receiver Bal. If receiver account, doesn't exist, create it.
    let receiver_account = accts.entry(tx.receiver.clone()).or_insert(Account {balance: 0, nonce: 0 });
    receiver_account.balance += tx.amount;

    // put the modified sender back into the AccountStore
    accts.insert(tx.sender.clone(), sender_account_clone);
    
    println!("Updated accounts {:#?}", accts);

    Ok(())
}

async fn submit_transaction(
    State(accounts): State<SharedAccountStore>,
    Json(tx): Json<Transaction>,
) -> Json<TxResponse> {
    
    let mut accts = accounts.lock().unwrap();

    match handle_transaction(&tx,&mut accts) {
        Ok(_) => Json(TxResponse {
            status: "ok".to_string(),
            message: format!("Processed transaction from {} to {} for {}", tx.sender, tx.receiver, tx.amount),
        }),
        Err(e) => Json(TxResponse {
            status: "error".to_string(),
            message: format!("{:?}", e),
        }),
    }
    
}

#[tokio::main]
async fn main() {

    let accounts: SharedAccountStore = Arc::new(Mutex::new({
        let mut accts: AccountStore = HashMap::new();
        // Populate with some initial accounts
        accts.insert("Alice".to_string(), Account { balance: 1000, nonce: 0 });
        accts.insert("Bob".to_string(), Account { balance: 500, nonce: 0 });
        println!("initial accounts {:?}", accts.keys());
        accts
    }));
    
    let app = Router::new()
        .route("/submit_transaction", post(submit_transaction))
        .with_state(accounts);
   
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

   // After starting this server, test it by sending a transaction using the following curl command in a separate terminal window
   // curl -X POST -H "Content-Type: application/json" -d '{"sender": "Alice", "receiver":"Bob", "amount":100, "nonce":0}' http://127.0.0.1:3000/submit_transaction

}
