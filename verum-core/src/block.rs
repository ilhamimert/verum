use serde::{Deserialize, Serialize};
use chrono::Utc;
use crate::transaction::Transaction;
use crate::crypto::sha256;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    pub index: u64,
    pub timestamp: i64,
    pub previous_hash: String,
    pub merkle_root: String,
    pub ai_consensus_score: f64,
    pub version: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
    pub hash: String,
    pub ai_signatures: Vec<String>,
}

impl Block {
    pub fn new(
        index: u64,
        previous_hash: &str,
        transactions: Vec<Transaction>,
    ) -> Self {
        let timestamp = Utc::now().timestamp();
        let merkle_root = Self::compute_merkle_root(&transactions);

        let header = BlockHeader {
            index,
            timestamp,
            previous_hash: previous_hash.to_string(),
            merkle_root,
            ai_consensus_score: 0.0,
            version: 1,
        };

        let hash = Self::compute_hash(&header);

        Block {
            header,
            transactions,
            hash,
            ai_signatures: Vec::new(),
        }
    }

    /// Genesis bloğu deterministik olmalı: tüm node'lar aynı hash'i üretsin
    /// diye id, timestamp ve imza sabit değerlerden oluşturulur.
    pub fn genesis() -> Self {
        const GENESIS_TIMESTAMP: i64 = 1_735_689_600; // 2025-01-01 00:00:00 UTC
        const ZERO_HASH: &str =
            "0000000000000000000000000000000000000000000000000000000000000000";

        let amount = 8_400_000.0; // Topluluk & Ekosistem payı
        let genesis_tx = Transaction {
            id: "VERUM-GENESIS-TX-0000".to_string(),
            from: "VERUM_GENESIS".to_string(),
            to: "VERUM_FOUNDATION".to_string(),
            amount,
            fee: 0.0,
            tx_type: crate::transaction::TransactionType::Transfer,
            timestamp: GENESIS_TIMESTAMP,
            signature: sha256(&format!(
                "VERUM-GENESIS-TX-0000VERUM_GENESISVERUM_FOUNDATION{}{}",
                amount, GENESIS_TIMESTAMP
            )),
            zk_proof: None,
        };

        let merkle_root = Self::compute_merkle_root(std::slice::from_ref(&genesis_tx));
        let header = BlockHeader {
            index: 0,
            timestamp: GENESIS_TIMESTAMP,
            previous_hash: ZERO_HASH.to_string(),
            merkle_root,
            ai_consensus_score: 1.0,
            version: 1,
        };
        let hash = Self::compute_hash(&header);

        Block {
            header,
            transactions: vec![genesis_tx],
            hash,
            ai_signatures: Vec::new(),
        }
    }

    fn compute_merkle_root(transactions: &[Transaction]) -> String {
        if transactions.is_empty() {
            return sha256("empty");
        }

        let hashes: Vec<String> = transactions.iter().map(|tx| tx.hash()).collect();

        let combined = hashes.join("");
        sha256(&combined)
    }

    fn compute_hash(header: &BlockHeader) -> String {
        let data = format!(
            "{}{}{}{}{}{}",
            header.index,
            header.timestamp,
            header.previous_hash,
            header.merkle_root,
            header.ai_consensus_score,
            header.version,
        );
        sha256(&data)
    }

    pub fn recalculate_hash(&mut self) {
        self.hash = Self::compute_hash(&self.header);
    }

    pub fn add_ai_signature(&mut self, node_id: &str, score: f64) {
        let sig = sha256(&format!("{}{}{}", node_id, self.hash, score));
        self.ai_signatures.push(sig);
        self.header.ai_consensus_score = score;
        self.recalculate_hash();
    }

    pub fn transaction_count(&self) -> usize {
        self.transactions.len()
    }
}
