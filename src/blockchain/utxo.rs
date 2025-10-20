use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TransactionOutput {
    pub amount: u64,
    pub recipient: String,
}

pub struct UTXOSet {
    pub utxos: HashMap<String, TransactionOutput>,
}

impl UTXOSet {
    pub fn new() -> Self {
        Self {
            utxos: HashMap::new(),
        }
    }
    
    pub fn get_balance(&self, address: &str) -> u64 {
        self.utxos.values()
            .filter(|output| output.recipient == address)
            .map(|output| output.amount)
            .sum()
    }
}
