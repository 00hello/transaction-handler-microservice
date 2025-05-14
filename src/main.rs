use std::collections::HashMap;

#[derive(Debug)]
struct Transaction {
    sender_id: String,
    receiver_id: String,
    amount: u64,
    nonce: u32
}

struct Account {
    id: String,
    balance: u64,
    current_nonce: u32,
}

enum TransactionError {
    InsufficientFunds,
    InvalidNonce,
    AccountNotFound
}


pub mod core_logic {
    use super::Account;
    use super::Transaction;
    use super::TransactionError;

    // fn validate_transaction(tx: &Transaction, sender_account: &Account) -> Result<(), TransactionError> {
        // Checklist
        // //  Enough funds
        // // Nonce is correct


    // }

    // fn execute_transaction(tx: &Transaction, sender: &mut Account, receiver: &mut Account) -> Result<(), TransactionError> {

    // } 
}

pub mod state_management {

}

fn main() {

    println!("Transaction Handler CLI - Starting...");
    
    let mut accts: HashMap<String, Account> = HashMap::new();

    // Populate with some initial accounts
    accts.insert(
        "Alice".to_string(), 
        Account {
            id: "Alice".to_string(), 
            balance: 100, 
            current_nonce: 0 
        }
    );
    accts.insert(
        "Bob".to_string(), 
        Account {
            id: "Bob".to_string(), 
            balance: 50, 
            current_nonce: 0 
        }
    );
    
    println!("initial accounts {:?}", accts.keys());

    // ... (your logic for creating and processing a sample transaction) ...

    let alice_account_option_ref = accts.get("Alice");

    let tx1 = match alice_account_option_ref {
        Some(acct) => Transaction {
            sender_id: acct.id.clone(),
            receiver_id: "Bob".to_string(),
            amount: 4,
            nonce: acct.current_nonce,
        },
        None => {
            panic!("{} account not found", "Alice");
        }
    };
    


    println!("Transaction is {:#?}", tx1);

    println!("...Transaction processing finished.");

}
