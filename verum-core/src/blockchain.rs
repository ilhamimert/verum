use crate::block::Block;
use crate::transaction::Transaction;
use crate::consensus::PoAIConsensus;

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub pending_transactions: Vec<Transaction>,
    consensus: PoAIConsensus,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis = Block::genesis();
        println!("🔵 VERUM Genesis Bloğu oluşturuldu");
        println!("   Hash: {}", genesis.hash);

        Blockchain {
            chain: vec![genesis],
            pending_transactions: Vec::new(),
            consensus: PoAIConsensus::new(),
        }
    }

    pub fn add_transaction(&mut self, tx: Transaction) -> bool {
        if !tx.is_valid() {
            println!("❌ Geçersiz işlem reddedildi: {}", tx.id);
            return false;
        }
        println!("📥 İşlem havuzuna eklendi: {} → {} ({} VRM)", tx.from, tx.to, tx.amount);
        self.pending_transactions.push(tx);
        true
    }

    pub fn mine_pending_transactions(&mut self) -> Option<&Block> {
        if self.pending_transactions.is_empty() {
            println!("⚠️  İşlem havuzu boş.");
            return None;
        }

        let previous_hash = self.chain.last().unwrap().hash.clone();
        let index = self.chain.len() as u64;
        let transactions = self.pending_transactions.drain(..).collect();

        let mut new_block = Block::new(index, &previous_hash, transactions);

        println!("\n🤖 PoAI Konsensüs başlatıldı — {} AI node oyluyor...", self.consensus.nodes.len());

        let result = self.consensus.validate_block(&mut new_block);

        if result.approved {
            println!("✅ Blok #{} onaylandı", new_block.header.index);
            println!("   Ortalama AI Skoru: {:.3}", result.average_score);
            println!("   Onay: {}/{}", result.approvals, result.total_nodes);
            println!("   Hash: {}", new_block.hash);
            self.chain.push(new_block);
            self.chain.last()
        } else {
            println!("❌ Blok reddedildi — AI skoru yetersiz ({:.3})", result.average_score);
            None
        }
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            if current.header.previous_hash != previous.hash {
                return false;
            }
        }
        true
    }

    pub fn get_balance(&self, address: &str) -> f64 {
        let mut balance = 0.0;

        for block in &self.chain {
            for tx in &block.transactions {
                if tx.to == address {
                    balance += tx.amount;
                }
                if tx.from == address {
                    balance -= tx.amount + tx.fee;
                }
            }
        }
        balance
    }

    pub fn block_count(&self) -> usize {
        self.chain.len()
    }

    pub fn print_chain(&self) {
        println!("\n╔══════════════════════════════════════╗");
        println!("║        VERUM Blockchain Durumu        ║");
        println!("╚══════════════════════════════════════╝");
        println!("Toplam Blok: {}", self.chain.len());
        println!("Zincir Geçerli: {}\n", self.is_valid());

        for block in &self.chain {
            println!("─── Blok #{} ───────────────────────────", block.header.index);
            println!("  Hash     : {}", &block.hash[..16]);
            println!("  Önceki   : {}", &block.header.previous_hash[..16]);
            println!("  İşlem    : {}", block.transaction_count());
            println!("  AI Skoru : {:.3}", block.header.ai_consensus_score);
        }
        println!();
    }
}

impl Default for Blockchain {
    fn default() -> Self {
        Self::new()
    }
}
