use serde::{Deserialize, Serialize};
use chrono::Utc;
use uuid::Uuid;
use crate::crypto::sha256;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionType {
    Transfer,
    Stake,
    Unstake,
    RealAnchor,
    SmartContract,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub from: String,
    pub to: String,
    pub amount: f64,
    pub fee: f64,
    pub tx_type: TransactionType,
    pub timestamp: i64,
    pub signature: String,
    pub zk_proof: Option<String>,
}

impl Transaction {
    pub fn new(from: &str, to: &str, amount: f64, tx_type: TransactionType) -> Self {
        let id = Uuid::new_v4().to_string();
        let timestamp = Utc::now().timestamp();
        let fee = amount * 0.001; // %0.1 işlem ücreti

        let signature = sha256(&format!("{}{}{}{}{}", id, from, to, amount, timestamp));

        Transaction {
            id,
            from: from.to_string(),
            to: to.to_string(),
            amount,
            fee,
            tx_type,
            timestamp,
            signature,
            zk_proof: None,
        }
    }

    pub fn with_zk_proof(mut self, proof: &str) -> Self {
        self.zk_proof = Some(proof.to_string());
        self
    }

    pub fn hash(&self) -> String {
        let data = format!(
            "{}{}{}{}{}{}",
            self.id, self.from, self.to, self.amount, self.fee, self.timestamp
        );
        sha256(&data)
    }

    pub fn is_valid(&self) -> bool {
        !self.from.is_empty()
            && !self.to.is_empty()
            && self.amount > 0.0
            && self.fee >= 0.0
    }
}
