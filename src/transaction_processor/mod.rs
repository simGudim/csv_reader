pub mod client_account;
pub mod transaction;
pub mod transaction_state;
pub mod processor;



#[cfg(test)]
mod tests {
    use super::{
        processor::Proccessor, 
        transaction::{Transaction, TransactionType}
    };

    #[test]
    #[ignore]
    fn execute_test() {
        assert_eq!(1+1 , 2);
    }

}
