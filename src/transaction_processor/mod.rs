pub mod client_account;
pub mod transaction;
pub mod processor;

#[cfg(test)]
mod tests {
    use super::{
        processor::Proccessor, 
        transaction::{Transaction, TransactionType}
    };


    #[test]
    fn deposit_and_dispute_test() {
        let mut processor  = Proccessor::new();
        let deposit_transaction = Transaction {
            transaction_type: TransactionType::Deposit,
            client: 1,
            tx: 1,
            amount: Some(2.0)
        };
        let dispute_transaction = Transaction {
            transaction_type: TransactionType::Dispute,
            client: 1,
            tx: 1,
            amount: None
        };
        processor.process_transaction(deposit_transaction);
        processor.process_transaction(dispute_transaction);
        let client_account = processor.get_client_account(1);
        assert_eq!(client_account.available, 0.0);
        assert_eq!(client_account.held, 2.0);
    }

    #[test]
    fn deposit_dispute_resolve_test() {
        let mut processor  = Proccessor::new();
        let deposit_transaction = Transaction {
            transaction_type: TransactionType::Deposit,
            client: 1,
            tx: 1,
            amount: Some(2.0)
        };
        let dispute_transaction = Transaction {
            transaction_type: TransactionType::Dispute,
            client: 1,
            tx: 1,
            amount: None
        };
        let resolve_transaction = Transaction {
            transaction_type: TransactionType::Resolve,
            client: 1,
            tx: 1,
            amount: None
        };
        processor.process_transaction(deposit_transaction);
        processor.process_transaction(dispute_transaction);
        let client_account = processor.get_client_account(1);
        assert_eq!(client_account.available, 0.0);
        assert_eq!(client_account.held, 2.0);
        assert_eq!(client_account.total, 2.0);

        processor.process_transaction(resolve_transaction);
        let client_account = processor.get_client_account(1);
        assert_eq!(client_account.available, 2.0);
        assert_eq!(client_account.held, 0.0);
        assert_eq!(client_account.total, 2.0);
    }

    #[test]
    fn deposit_dispute_chargeback_test() {
        let mut processor  = Proccessor::new();
        let deposit_transaction = Transaction {
            transaction_type: TransactionType::Deposit,
            client: 1,
            tx: 1,
            amount: Some(2.0)
        };
        let dispute_transaction = Transaction {
            transaction_type: TransactionType::Dispute,
            client: 1,
            tx: 1,
            amount: None
        };
        let chargeback_transaction = Transaction {
            transaction_type: TransactionType::Chargeback,
            client: 1,
            tx: 1,
            amount: None
        };
        processor.process_transaction(deposit_transaction);
        processor.process_transaction(dispute_transaction);
        let client_account = processor.get_client_account(1);
        assert_eq!(client_account.available, 0.0);
        assert_eq!(client_account.held, 2.0);
        assert_eq!(client_account.total, 2.0);

        processor.process_transaction(chargeback_transaction);
        let client_account = processor.get_client_account(1);
        assert_eq!(client_account.available, 0.0);
        assert_eq!(client_account.held, 0.0);
        assert_eq!(client_account.total, 0.0);
        assert_eq!(client_account.locked, true);
    }

}
