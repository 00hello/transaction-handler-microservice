use std::collections::HashMap;

#[derive(Debug)]
pub struct Transaction {
    pub sender_id: String,
    pub receiver_id: String,
    pub amount: u64,
    pub nonce: u32
}

#[derive(Debug, Clone)]
pub struct Account {
    pub id: String,
    pub balance: u64,
    pub current_nonce: u32,
}

#[derive(Debug)]
pub enum TransactionError {
    InsufficientFunds,
    InvalidNonce,
    AccountNotFound,
    SenderIsReceiver,
    AmountIsZero
}


pub mod core_logic {
    use super::Account;
    use super::Transaction;
    use super::TransactionError;
    use super::HashMap;

    pub fn validate_transaction(tx: &Transaction, sender_account: &Account) -> Result<(), TransactionError> {
    

        // 1. account exists
        // // handleed by the code that calls this function


        // 2. sender is not receiver
        if tx.sender_id == tx.receiver_id {
            return Err(TransactionError::SenderIsReceiver);
        }
        // 3. amount is greater than zero
        if tx.amount == 0 {
            return Err(TransactionError::AmountIsZero);
        }
        // 4. sender has enough funds
        if sender_account.balance < tx.amount {
            return Err(TransactionError::InsufficientFunds);
        }
        // 5. nonce is correct (same as "current_nonce" in sender account)
        if tx.nonce != sender_account.current_nonce {
            return Err(TransactionError::InvalidNonce);
        }
        
        
        // all checks pass
        Ok(())
                
    }

    // pub fn execute_transaction(tx: &Transaction, sender: &mut Account, receiver: &mut Account) -> Result<(), TransactionError> {
    //    Ok(())
    // } 
    pub fn execute_transaction(tx: &Transaction, accts: &mut HashMap<String, Account>) -> Result<(), TransactionError> {

        
        // now get mutable references for transaction execution
        // // clone the ID's from tx1 to avoid borrowing issues with accts and tx1 simultaneously
        let sender_id_for_execution = tx.sender_id.clone();
        let receiver_id_for_execution = tx.receiver_id.clone();

        // get mutable copy of sender account and receiver account
        // NOTE: .unwrap() will panic if key not found. For MVP this might be acceptable,
        // but for production, handle Option properly (e.g., return TransactionError::AccountNotFound)
        let mut sender_account = accts.get(&sender_id_for_execution).unwrap().clone();
        let mut receiver_account = accts.get(&receiver_id_for_execution).unwrap().clone();

        // subtract money from sender account
        // add money to receiver account
        // adjust sender account nonce += 1

        // overwrite the origingal accounts in the hash map with the clones
        // Example (if you were to implement it here):
        // sender_account.balance -= tx.amount;
        // receiver_account.balance += tx.amount;
        // sender_account.current_nonce += 1;
        // accts.insert(sender_id_for_execution, sender_account);
        // accts.insert(receiver_id_for_execution, receiver_account);

        Ok(())

  
     } 
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

    let (tx1, sender_account_for_validation_ref) = match alice_account_option_ref {
        Some(acct_ref) => 
        (
            Transaction {
            sender_id: acct_ref.id.clone(),
            receiver_id: "Bob".to_string(),
            amount: 4,
            nonce: acct_ref.current_nonce,
            }, 
            acct_ref
        ),
        None => {
            panic!("{} Account not found", "Alice");
        }
    };
    

    println!("Transaction 1 is {:#?}", tx1);
    println!("Account for validation (Alice) is {:#?}", sender_account_for_validation_ref);

    // validate transaction
    match core_logic::validate_transaction(&tx1, sender_account_for_validation_ref) {
        Ok(_) => {
            println!("Transaction validated successfully!");

           

           
            println!("Atttempting to execute transaction...");

            // Execute transaction
            
            match core_logic::execute_transaction(&tx1, &mut accts) {
                Ok(_) => {
                    println!("Tranasction executed successfully!");
                    // Optionally, print updated account states
                    println!("State of accounts after transaction:");
                    // Use get() for printing as we only need immutable access here.
                    println!("Account '{}': {:#?}", tx1.sender_id, accts.get(&tx1.sender_id).unwrap());
                    println!("Account '{}': {:#?}", tx1.receiver_id, accts.get(&tx1.receiver_id).unwrap());
                },
                Err(e) => {
                    println!("Transaction execution failed: {:#?}",e);
                }
            }
        },
        Err(e) => {
            println!("Transaction validation failed: {:#?}", e);
        }
    }

    println!("...Transaction processing finished.");

}
