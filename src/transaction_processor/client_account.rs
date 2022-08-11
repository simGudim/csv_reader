use serde::{Serialize, Serializer};


fn round_output_record<S: Serializer>(amount: &f64, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_f64((amount * 1000.0).round() / 1000.0)
}

#[derive(Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "PascalCase")]
pub struct ClientAccount {
    pub client: u16,
    #[serde(serialize_with = "round_output_record")]
    pub available: f64,
    #[serde(serialize_with = "round_output_record")]
    pub held: f64,
    #[serde(serialize_with = "round_output_record")]
    pub total: f64,
    pub locked: bool
}


impl ClientAccount {
    pub fn new(client: u16) -> Self {
        Self {
            client,
            available: 0.0,
            held: 0.0,
            total: 0.0,
            locked: false
        }
    }

    // increase the available and total funds of the client account
    pub fn deposit(&mut self, amount: f64) {
        self.total += amount;
        self.available += amount
    }

    // available: decrease the available and total funds of the client account,
    // if a client does not have sufficient available funds the withdrawal should fail
    pub fn withdrawl(&mut self, amount: f64) {
        if self.available >= amount {
            self.total =- amount;
            self.available = self.total - amount;
        } else {
            println!("insufficient funds for withdrawl");
        }
    }

    // available funds should decrease by the amount disputed, 
    // their held funds should increase by the amount disputed, 
    // while their total funds should remain the same
    pub fn dispute(&mut self, amount: f64) {
        if self.available >= amount {
            self.available -= amount;
            self.held += amount;
        } else {
            println!("insufficient funds for dispute");
        }
    }

    // held funds should decrease by the amount no longer disputed, 
    // their available funds should increase by the amount no longer disputed, 
    // and their total funds should remain the same.
    pub fn resolve(&mut self, amount: f64) {
        if self.held >= amount {
            self.held -= amount;
            self.available += amount;
        } else {
            println!("something is incorrect with disputed transaction")
        }
    }

    // held funds and total funds should decrease by the amount previously disputed. 
    // If a chargeback occurs the client's account should be immediately frozen.
    pub fn chargeback(&mut self, amount: f64) {
        if (self.held >= amount) & (self.total >= amount) {
            self.held -= amount;
            self.total -= amount;
            self.locked = true;
        } else {
            println!("an error occured during the transaction");
        }
    }
}