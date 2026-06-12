use rand::Rng;
use crate::block::Block;

// PoAI — Proof of AI Consensus
// Dünyada ilk AI tabanlı konsensüs mekanizması
// Gerçek implementasyonda her node bir AI modeli çalıştırır
// Bu sürüm simülasyon tabanlıdır (testnet için)

pub struct AINode {
    pub id: String,
    pub stake: f64,
    pub reliability_score: f64,
}

impl AINode {
    pub fn new(id: &str, stake: f64) -> Self {
        AINode {
            id: id.to_string(),
            stake,
            reliability_score: 1.0,
        }
    }

    // Bloğu AI ile değerlendirir, skor döndürür (0.0 - 1.0)
    pub fn evaluate_block(&self, block: &Block) -> f64 {
        let mut rng = rand::thread_rng();

        // Gerçek implementasyonda burada AI modeli çalışır
        // Şimdilik: işlem geçerliliği + rastgele güvenilirlik skoru
        let tx_validity = block.transactions.iter().all(|tx| tx.is_valid());

        if !tx_validity {
            return 0.0;
        }

        // Simüle edilmiş AI skoru (0.7 - 1.0 arası güvenilir nodelar)
        let base_score: f64 = rng.gen_range(0.7..=1.0);
        base_score * self.reliability_score
    }
}

pub struct PoAIConsensus {
    pub nodes: Vec<AINode>,
    pub threshold: f64,      // Onay için gereken minimum ortalama skor
    pub quorum: usize,        // Kaç node'un oy vermesi gerekiyor
}

impl PoAIConsensus {
    pub fn new() -> Self {
        // İlk 7 AI node (testnet)
        let nodes = vec![
            AINode::new("VRM-NODE-001", 10_000.0),
            AINode::new("VRM-NODE-002", 10_000.0),
            AINode::new("VRM-NODE-003", 10_000.0),
            AINode::new("VRM-NODE-004", 10_000.0),
            AINode::new("VRM-NODE-005", 10_000.0),
            AINode::new("VRM-NODE-006", 10_000.0),
            AINode::new("VRM-NODE-007", 10_000.0),
        ];

        PoAIConsensus {
            nodes,
            threshold: 0.67, // %67 çoğunluk
            quorum: 5,         // En az 5 node oy vermeli
        }
    }

    pub fn validate_block(&self, block: &mut Block) -> ConsensusResult {
        let mut scores: Vec<f64> = Vec::new();
        let mut approvals = 0;

        for node in &self.nodes {
            let score = node.evaluate_block(block);
            scores.push(score);

            if score >= self.threshold {
                approvals += 1;
                block.add_ai_signature(&node.id, score);
            }
        }

        let avg_score = scores.iter().sum::<f64>() / scores.len() as f64;
        let approved = approvals >= self.quorum && avg_score >= self.threshold;

        ConsensusResult {
            approved,
            average_score: avg_score,
            approvals,
            total_nodes: self.nodes.len(),
        }
    }
}

impl Default for PoAIConsensus {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct ConsensusResult {
    pub approved: bool,
    pub average_score: f64,
    pub approvals: usize,
    pub total_nodes: usize,
}
