use std::collections::HashMap;

use super::client_account::ClientAccount;
use super::transaction::{
    Transaction, 
    TransactionType, 
    TransactionState
};


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

    // inserts transaction history for deposits and withdrawls
    fn insert_transaction_history(&mut self, transaction: &Transaction) {
        self.transactions_map
            .entry(transaction.tx)
            .or_insert_with(|| {
                TransactionState {
                    client: transaction.client,
                    amount: transaction.amount
                }
            }
        );
    }

    // checks if it is a valid amount within the Option<f64>
    fn check_valid_amount(&self, amount: Option<f64>) -> f64 {
        if let Some(value) = amount {
           value
        } else {
            0.0
        }
    }

    // makes a depist transaction for a particlar account within the HashMap, 
    // otherwise inserts a default account and makes a deposit into the account
    fn insert_deposit_transaction_into_account(&mut self, transaction: &Transaction) {
        let amount = self.check_valid_amount(transaction.amount);
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

    // makes a depist transaction for a particlar account within the HashMap, 
    // otherwise inserts a default account
    fn insert_withdrawl_transaction_into_account(&mut self, transaction: &Transaction) {
        let amount = self.check_valid_amount(transaction.amount);
        self.accounts_map
            .entry(transaction.client)
            .and_modify(|account| {
                account.withdrawl(amount)
            }).or_insert_with(|| {
                let account: ClientAccount = ClientAccount::new(transaction.client);
                account
            });
    }

    // handle all the dispute transactions, such as dipiuste, resolve, chargeback
    fn handle_dipsute_transactions(&mut self, transaction: &Transaction) {
        let transaction_state = self.transactions_map.get(&transaction.tx);
        match transaction_state {
            Some(state) => {
                let amount = self.check_valid_amount(state.amount);
                self.accounts_map
                    .entry(transaction.client)
                    .and_modify(|client| {
                        match transaction.transaction_type {
                            TransactionType::Dispute => client.dispute(amount),
                            // ASSUMPTION: if a transaction is resolved, we no longer need it in memory
                            TransactionType::Resolve => {
                                client.resolve(amount);
                                self.transactions_map.remove(&transaction.tx);
                            },
                            TransactionType::Chargeback => client.chargeback(amount),
                            _ => eprintln!("something is terribly wrong with your code....")
                        }
                    }
                ).or_insert_with(|| ClientAccount::new(transaction.client));
            },
            None => eprintln!("transaction doesn't exist, maybe something went wrong....")
        }
    }

    // main processor fucntions as an interface for the processing work 
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
            _  => self.handle_dipsute_transactions(&transaction)
        }
    }


    // only used in tests
    pub fn get_client_account(&self, client: u16) -> &ClientAccount {
        self.accounts_map.get(&client).unwrap()   
    }

}
