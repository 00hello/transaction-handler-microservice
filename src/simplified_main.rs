use std::collections::HashMap;

// Simple types for prototype
#[derive(Debug, Clone)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
    pub nonce: u64,
    // signature: String, // Omitted for simplicity in prototype
}

#[derive(Debug, Clone, PartialEq)]
pub struct Account {
    pub balance: u64,
    pub nonce: u64,
}

// Using a type alias for clarity
pub type AccountStore = HashMap<String, Account>;

/// Handles a single transaction, updating account balances and nonces.
///
/// Performs the following validations:
/// 1. Sender and receiver are not the same.
/// 2. Sender account exists.
/// 3. Transaction nonce is the sender's current_nonce + 1.
/// 4. Sender has sufficient funds.
///
/// If valid, it updates sender and receiver balances and increments the sender's nonce.
/// If the receiver account does not exist, it is created with 0 balance and 0 nonce before receiving funds.
pub fn handle_transaction(
    transaction: &Transaction,
    accounts: &mut AccountStore,
) -> Result<(), String> {
    // 1. Validate sender isn't receiver
    if transaction.sender == transaction.receiver {
        return Err(String::from("Sender cannot be the receiver."));
    }

    // Clone sender account data for validation and modification
    let mut sender_account_clone = accounts
        .get(&transaction.sender)
        .ok_or_else(|| format!("Sender account '{}' not found (must exist).", transaction.sender))?
        .clone();

    // Clone receiver account data for modification (validation primarily on sender)
    // We still need to ensure it exists as per our assumption.
    let mut receiver_account_clone = accounts
        .get(&transaction.receiver)
        .ok_or_else(|| format!("Receiver account '{}' not found (must exist).", transaction.receiver))?
        .clone();

    // Validate nonce for sender using the cloned data
    if transaction.nonce != sender_account_clone.nonce + 1 {
        return Err(format!(
            "Invalid nonce for sender {}. Expected {}, got {}.",
            transaction.sender,
            sender_account_clone.nonce + 1,
            transaction.nonce
        ));
    }

    // Validate sufficient funds for sender using the cloned data
    if sender_account_clone.balance < transaction.amount {
        return Err(format!(
            "Sender {} has insufficient funds. Balance: {}, Amount: {}.",
            transaction.sender, sender_account_clone.balance, transaction.amount
        ));
    }

    // Perform the transaction updates on the cloned accounts
    sender_account_clone.balance -= transaction.amount;
    sender_account_clone.nonce += 1;
    receiver_account_clone.balance += transaction.amount;

    // Put the modified clones back into the HashMap
    accounts.insert(transaction.sender.clone(), sender_account_clone);
    accounts.insert(transaction.receiver.clone(), receiver_account_clone);

    Ok(())
}

fn main() {
    let mut accounts: AccountStore = HashMap::new();

    // Initialize some accounts
    accounts.insert(
        String::from("Alice"),
        Account { balance: 1000, nonce: 0 },
    );
    accounts.insert(
        String::from("Bob"),
        Account { balance: 500, nonce: 0 },
    );

    println!("Initial accounts: {:?}", accounts);

    // --- Test Case 1: Valid transaction ---
    let tx1 = Transaction {
        sender: String::from("Alice"),
        receiver: String::from("Bob"),
        amount: 100,
        nonce: 1, // Alice's current nonce is 0, so next is 1
    };
    println!("\nProcessing transaction: {:?}", tx1);
    match handle_transaction(&tx1, &mut accounts) {
        Ok(_) => println!("Tx1 successful. Updated accounts: {:?}", accounts),
        Err(e) => println!("Tx1 failed: {}", e),
    }
    // Expected: Alice: { balance: 900, nonce: 1 }, Bob: { balance: 600, nonce: 0 }

    // --- Test Case 2: Sufficient funds (Bob sending to Alice) ---
    // After Tx1: Alice: { balance: 900, nonce: 1 }, Bob: { balance: 600, nonce: 0 }
    let tx2 = Transaction {
        sender: String::from("Bob"),
        receiver: String::from("Alice"),
        amount: 600, // Bob has 600
        nonce: 1,    // Bob's current nonce is 0, so next is 1
    };
    println!("\nProcessing transaction: {:?}", tx2);
    match handle_transaction(&tx2, &mut accounts) {
        Ok(_) => println!("Tx2 successful. Updated accounts: {:?}", accounts),
        Err(e) => println!("Tx2 failed: {}", e),
    }
    // Expected: Bob: { balance: 0, nonce: 1 }, Alice: { balance: 1500, nonce: 1 }

    println!("\nFinal accounts: {:?}", accounts);
} 