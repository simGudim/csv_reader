use super::transaction::TransactionType;

#[derive(Debug)]
pub struct TransactionState {
    pub client: u16,
    pub transaction_type: TransactionType,
    pub amount: Option<f64>
}