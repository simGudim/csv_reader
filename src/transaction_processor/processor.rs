use std::collections::HashMap;

use super::client_account::ClientAccount;
use super::transaction_state::{TransactionState};
use super::transaction::{Transaction, TransactionType};

pub struct Proccessor {
    pub transactions_map: HashMap<u32, TransactionState>,
    pub accounts_map: HashMap<u16, ClientAccount> 
}


impl Proccessor {
    pub fn new() -> Self {
        Self { 
            transactions_map: HashMap::new(),
            accounts_map: HashMap::new()
         }
    }

    pub fn insert_transaction_history(&mut self, transaction: &Transaction) {
        self.transactions_map
            .entry(transaction.tx)
            .or_insert_with(|| {
                TransactionState {
                    client: transaction.client,
                    transaction_type: transaction.transaction_type,
                    amount: transaction.amount
                }
            });
    }

    fn check_valid_amount(&self, transaction: &Transaction) -> f64 {
        if let Some(amount) = transaction.amount {
           amount
        } else {
            0.0
        }
    }

    fn insert_deposit_transaction_into_account(&mut self, transaction: &Transaction) {
        let amount = self.check_valid_amount(&transaction);
        self.accounts_map
            .entry(transaction.client)
            .and_modify(|account| {
                account.deposit(amount)
            }).or_insert_with(|| {
                let mut account: ClientAccount = ClientAccount::new(transaction.client);
                account.deposit(amount);
                account
            });
    }

    fn insert_withdrawl_transaction_into_account(&mut self, transaction: &Transaction) {
        let amount = self.check_valid_amount(&transaction);
        self.accounts_map
            .entry(transaction.client)
            .and_modify(|account| {
                account.withdrawl(amount)
            }).or_insert_with(|| {
                let mut account: ClientAccount = ClientAccount::new(transaction.client);
                account.withdrawl(amount);
                account
            });
    }


    fn handle_dipsute_transactions(&mut self, transaction_state: Option<&TransactionState>, transaction: &mut Transaction) {
        // let mut transaction_state = self.accounts_map.get_mut(&transaction.client);
        self.get_transaction_state(&transaction.tx);
        let amount = self.check_valid_amount(&transaction);
        match transaction_state {
            Some(state) => {
                let client = self.get_client_account(&state.client);
                if let Some(cl) = client {
                    match transaction.transaction_type {
                        TransactionType::Dispute => cl.dispute(amount),
                        TransactionType::Withdrawal => cl.withdrawl(amount),
                        TransactionType::Chargeback => cl.chargeback(amount),
                        _ => {}
                    }
                } else {
                    self.accounts_map
                        .entry(transaction.client)
                        .or_insert_with(|| ClientAccount::new(transaction.client));
                }
            },
            None => {
                println!("transaction doesn't exist")
            }
        }
    }

    fn get_transaction_state(&self, tx: &u32) -> Option<&TransactionState> {
        self.transactions_map.get(tx)  
    }

    fn get_client_account(&mut self, client: &u16) -> Option<&mut ClientAccount> {
        self.accounts_map.get_mut(client)
    }

    pub fn process_transaction(&mut self, transaction: Transaction) {
        match transaction.transaction_type {
            TransactionType::Deposit => {
                self.insert_transaction_history(&transaction);
                self.insert_deposit_transaction_into_account(&transaction);
            },
            TransactionType::Withdrawal => {
                self.insert_transaction_history(&transaction);
                self.insert_withdrawl_transaction_into_account(&transaction);
            },
            _  => {
                let transaction_state = self.get_transaction_state(&transaction.tx);
                self.handle_dipsute_transactions(transaction_state, &mut transaction);

            }
        }

    }
}