use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionInput {
    pub previous_output: String,  // Hash de transaction précédente  
    pub output_index: u32,        // Index de l'output utilisé
    pub signature: String,        // Signature de l'expéditeur
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionOutput {
    pub amount: u64,              // Montant en satoshis (1 AUR = 100M satoshis)
    pub recipient: String,        // Adresse du destinataire
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub inputs: Vec<TransactionInput>,
    pub outputs: Vec<TransactionOutput>,
    pub timestamp: i64,
    pub signature: String,        // Garde compatibilité avec l'existant
    pub fee: u64,                 // Nouveau : frais de transaction
}

impl Transaction {
    pub fn new_transfer(
        inputs: Vec<TransactionInput>,
        outputs: Vec<TransactionOutput>,
        fee: u64,
    ) -> Self {
        let mut tx = Transaction {
            id: String::new(),
            inputs,
            outputs,
            timestamp: chrono::Utc::now().timestamp(),
            signature: String::new(),
            fee,
        };
        
        tx.id = tx.calculate_hash();
        tx
    }
    
    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        let data = format!("{:?}{:?}{}{}", 
            self.inputs, self.outputs, self.timestamp, self.fee);
        hasher.update(data.as_bytes());
        format!("{:x}", hasher.finalize())
    }
    
    pub fn is_coinbase(&self) -> bool {
        self.inputs.len() == 1 && 
        self.inputs[0].previous_output == "0"
    }
    
    pub fn get_total_output(&self) -> u64 {
        self.outputs.iter().map(|o| o.amount).sum()
    }
}
