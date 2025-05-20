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

#[derive(Debug, Clone, Deserialize)]
pub struct Transaction {
    pub sender: String, // simplify naming convention, removing _id
    pub receiver: String, // simplify naming convention, removing _id
    pub amount: u64,
    pub nonce: u32,
    // signature: String, // Omitted for simplicity in prototype.
}

#[derive(Debug, Serialize)]
struct TxResponse {
    status: String,
    message: String,
}

// Simple Account struct. No 'id' field 
// More efficient as the id is implied since it's the key in the hash map
#[derive(Debug, Clone, PartialEq)] // So we can '==' instances of this struct with each other
pub struct Account {
    pub balance: u64,
    pub nonce: u32, // simplify current_nonce to 'nonce'
}

#[derive(Debug)]
pub enum TransactionError {
    AmountIsZero,
    SenderIsReceiver,
    AccountNotFound,
    InsufficientFunds,
    InvalidNonce,
}

pub type AccountStore = HashMap<String, Account>;


pub type SharedAccountStore = Arc<Mutex<AccountStore>>;



// Comprehewnsive function documentation
// Handles a single transaction, updating account balances and nonces

// performs the following validation

// 1. Transcation amount is not zero
// 2. Sender and receiver are not the same 
// 3. Sender account exists
// 4. Sender has sufficient funds
// 5. Transaction's nonce is the sender's current nonce. Incremented after the transaction

// if valid, it updates the sender and receiver balances and increments the sender's nonce
// if the recewiver account doesn't exist, it's created with 0 balance and 0 nonce before receiving funds

pub fn handle_transaction(
    tx: &Transaction,
    accts: &mut AccountStore,
) -> Result<(), TransactionError> {
    // 1 Transaction amount is not zero

    if tx.amount == 0 {
        return Err(TransactionError::AmountIsZero);
    }


    // 2 validate sender isn't receiver
    if tx.sender == tx.receiver {
        return Err(TransactionError::SenderIsReceiver);
    }

    
   // 3. Very Sender account exists by using get and unwrap before cloning it
   let mut sender_account_clone = accts.get(&tx.sender).unwrap().clone();

    // 4 has sufficient funds
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
