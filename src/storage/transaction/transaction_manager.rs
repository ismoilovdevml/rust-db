// src/storage/transaction/transaction_manager.rs

use crate::storage::transaction::transaction::Transaction;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use uuid::Uuid;

pub struct TransactionManager {
    transactions: Arc<Mutex<HashMap<Uuid, Transaction>>>,
}

impl TransactionManager {
    pub fn new() -> Self {
        Self {
            transactions: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn create_transaction(&self) -> Transaction {
        let mut transaction = Transaction::new();
        let transaction_id = transaction.id;
        self.transactions.lock().unwrap().insert(transaction_id, transaction.clone());
        transaction
    }

    pub fn commit_transaction(&self, transaction: Transaction) -> Result<(), String> {
        // Implement your commit logic here
        // Apply the changes of the transaction to the storage
        // and remove the transaction from the active transactions
        self.transactions.lock().unwrap().remove(&transaction.id);
        Ok(())
    }

    pub fn rollback_transaction(&self, transaction: Transaction) -> Result<(), String> {
        // Implement your rollback logic here
        // Undo the changes of the transaction and remove it from the active transactions
        self.transactions.lock().unwrap().remove(&transaction.id);
        Ok(())
    }
}
