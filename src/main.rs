use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Transaction {
    pub sender: String, // simplify naming convention, removing _id
    pub receiver: String, // simplify naming convention, removing _id
    pub amount: u64,
    pub nonce: u32,
    // signature: String, // Omitted for simplicity in prototype.
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


fn main() {

    println!("Transaction Handler CLI - Starting...");
    
    let mut accts: AccountStore = HashMap::new();

    // Populate with some initial accounts
    accts.insert(
        "Alice".to_string(), 
        Account {
            balance: 1000, 
            nonce: 0 
        }
    );
    accts.insert(
        "Bob".to_string(), 
        Account { 
            balance: 500, 
            nonce: 0 
        }
    );
    
    println!("initial accounts {:?}", accts.keys());


    let tx1 = Transaction {
        sender: String::from("Alice"),
        receiver: String::from("Bob"),
        amount: 100,
        nonce: 0,
    };

    println!("\n processing transaction {:?}", tx1);

    handle_transaction(&tx1, &mut accts).unwrap();

   

}
