// ZK-Shield — VERUM gizli işlem katmanı (eğitim amaçlı simülasyon)
//
// Gerçek bir ZK sisteminde (Groth16, PLONK, Bulletproofs) burada eliptik eğri
// tabanlı kanıtlar olurdu. Bu modül aynı akışı hash-tabanlı taahhütlerle
// simüle eder: miktar gizlenir, çift harcama nullifier ile engellenir.
//
// Akış:
//   shield()   → açık VRM'yi havuza kilitle, gizli not (note) al
//   transfer() → notu yeni sahibe devret, miktar hiç görünmez
//   unshield() → notu boz, açık VRM'ye geri dön

use serde::{Deserialize, Serialize};
use rand::RngCore;
use rand::rngs::OsRng;
use crate::crypto::sha256;

/// Gizli not: zincir üzerinde yalnızca commitment görünür.
/// Miktar ve sahibi yalnızca `secret`'ı bilen kişi açabilir.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShieldedNote {
    /// sha256(amount || owner || blinding) — zincirde görünen tek şey
    pub commitment: String,
    /// Çift harcamayı engelleyen tek kullanımlık değer
    pub nullifier: String,
}

/// Notu açabilmek için gereken gizli bilgi — sahibinde kalır, zincire yazılmaz.
#[derive(Debug, Clone)]
pub struct NoteSecret {
    pub amount: f64,
    pub owner: String,
    pub blinding: String,
}

impl NoteSecret {
    pub fn commitment(&self) -> String {
        sha256(&format!("{}|{}|{}", self.amount, self.owner, self.blinding))
    }

    pub fn nullifier(&self) -> String {
        sha256(&format!("NULLIFIER|{}|{}", self.blinding, self.owner))
    }
}

/// Simüle edilmiş sıfır-bilgi kanıtı: "commitment geçerli bir miktarı
/// gizliyor ve ben açabiliyorum" iddiasını miktarı ifşa etmeden temsil eder.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkProof {
    pub proof_hash: String,
    pub public_input: String, // doğrulayıcının gördüğü: commitment
}

#[derive(Debug)]
pub enum ShieldError {
    InsufficientBalance,
    UnknownCommitment,
    DoubleSpend,
    InvalidProof,
    InvalidAmount,
}

impl std::fmt::Display for ShieldError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            ShieldError::InsufficientBalance => "yetersiz bakiye",
            ShieldError::UnknownCommitment => "bilinmeyen commitment",
            ShieldError::DoubleSpend => "çift harcama girişimi",
            ShieldError::InvalidProof => "geçersiz ZK kanıtı",
            ShieldError::InvalidAmount => "geçersiz miktar",
        };
        write!(f, "{msg}")
    }
}

impl std::error::Error for ShieldError {}

/// Gizli işlem havuzu: kilitli VRM + geçerli commitment kümesi + harcanmış
/// nullifier kümesi. Gerçek sistemde bu durum zincir üzerinde tutulur.
pub struct ShieldedPool {
    pub total_locked: f64,
    commitments: Vec<String>,
    spent_nullifiers: Vec<String>,
}

impl ShieldedPool {
    pub fn new() -> Self {
        ShieldedPool {
            total_locked: 0.0,
            commitments: Vec::new(),
            spent_nullifiers: Vec::new(),
        }
    }

    /// Açık VRM'yi havuza kilitle, gizli not üret.
    pub fn shield(&mut self, amount: f64, owner: &str) -> Result<(ShieldedNote, NoteSecret), ShieldError> {
        if amount <= 0.0 {
            return Err(ShieldError::InvalidAmount);
        }

        let secret = NoteSecret {
            amount,
            owner: owner.to_string(),
            blinding: random_blinding(),
        };

        let note = ShieldedNote {
            commitment: secret.commitment(),
            nullifier: secret.nullifier(),
        };

        self.commitments.push(note.commitment.clone());
        self.total_locked += amount;

        Ok((note, secret))
    }

    /// Gizli transfer: eski not nullifier ile geçersizleşir, alıcı için yeni
    /// commitment eklenir. Miktar zincire hiç yazılmaz.
    pub fn transfer(
        &mut self,
        sender_secret: &NoteSecret,
        proof: &ZkProof,
        recipient: &str,
    ) -> Result<(ShieldedNote, NoteSecret), ShieldError> {
        self.consume_note(sender_secret, proof)?;

        let new_secret = NoteSecret {
            amount: sender_secret.amount,
            owner: recipient.to_string(),
            blinding: random_blinding(),
        };

        let new_note = ShieldedNote {
            commitment: new_secret.commitment(),
            nullifier: new_secret.nullifier(),
        };

        self.commitments.push(new_note.commitment.clone());

        Ok((new_note, new_secret))
    }

    /// Notu boz, miktarı açık VRM olarak serbest bırak.
    pub fn unshield(&mut self, secret: &NoteSecret, proof: &ZkProof) -> Result<f64, ShieldError> {
        self.consume_note(secret, proof)?;
        self.total_locked -= secret.amount;
        Ok(secret.amount)
    }

    /// Ortak doğrulama: commitment havuzda mı, nullifier harcanmamış mı,
    /// kanıt geçerli mi? Geçerliyse notu harcanmış say.
    fn consume_note(&mut self, secret: &NoteSecret, proof: &ZkProof) -> Result<(), ShieldError> {
        let commitment = secret.commitment();

        if !self.commitments.contains(&commitment) {
            return Err(ShieldError::UnknownCommitment);
        }

        let nullifier = secret.nullifier();
        if self.spent_nullifiers.contains(&nullifier) {
            return Err(ShieldError::DoubleSpend);
        }

        if !verify_proof(proof, &commitment) {
            return Err(ShieldError::InvalidProof);
        }

        self.spent_nullifiers.push(nullifier);
        Ok(())
    }

    pub fn commitment_count(&self) -> usize {
        self.commitments.len()
    }

    pub fn spent_count(&self) -> usize {
        self.spent_nullifiers.len()
    }
}

impl Default for ShieldedPool {
    fn default() -> Self {
        Self::new()
    }
}

/// Kanıt üretimi: gizli bilgiden, miktarı ifşa etmeyen bir kanıt türet.
/// Gerçek sistemde burada bir SNARK devresi çalışır.
pub fn generate_proof(secret: &NoteSecret) -> ZkProof {
    let commitment = secret.commitment();
    ZkProof {
        proof_hash: sha256(&format!("ZKPROOF|{}|{}", commitment, secret.blinding)),
        public_input: commitment,
    }
}

/// Kanıt doğrulama: doğrulayıcı yalnızca commitment'ı görür, miktarı asla.
/// Simülasyonda kanıtın commitment'a bağlı olduğunu kontrol ediyoruz.
fn verify_proof(proof: &ZkProof, commitment: &str) -> bool {
    proof.public_input == commitment && !proof.proof_hash.is_empty()
}

fn random_blinding() -> String {
    let mut bytes = [0u8; 32];
    OsRng.fill_bytes(&mut bytes);
    hex::encode(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pool_with_note(amount: f64, owner: &str) -> (ShieldedPool, ShieldedNote, NoteSecret) {
        let mut pool = ShieldedPool::new();
        let (note, secret) = pool.shield(amount, owner).unwrap();
        (pool, note, secret)
    }

    #[test]
    fn shield_locks_amount_and_registers_commitment() {
        let (pool, note, secret) = pool_with_note(100.0, "VRM_ALICE");
        assert_eq!(pool.total_locked, 100.0);
        assert_eq!(pool.commitment_count(), 1);
        assert_eq!(note.commitment, secret.commitment());
    }

    #[test]
    fn rejects_zero_or_negative_amount() {
        let mut pool = ShieldedPool::new();
        assert!(matches!(pool.shield(0.0, "X"), Err(ShieldError::InvalidAmount)));
        assert!(matches!(pool.shield(-5.0, "X"), Err(ShieldError::InvalidAmount)));
    }

    #[test]
    fn transfer_moves_note_without_revealing_amount() {
        let (mut pool, _note, secret) = pool_with_note(250.0, "VRM_ALICE");
        let proof = generate_proof(&secret);

        let (new_note, new_secret) = pool.transfer(&secret, &proof, "VRM_BOB").unwrap();

        assert_eq!(new_secret.owner, "VRM_BOB");
        assert_eq!(new_secret.amount, 250.0);
        assert_ne!(new_note.commitment, secret.commitment());
        assert_eq!(pool.total_locked, 250.0); // kilitli miktar değişmez
    }

    #[test]
    fn double_spend_is_rejected() {
        let (mut pool, _note, secret) = pool_with_note(50.0, "VRM_ALICE");
        let proof = generate_proof(&secret);

        pool.transfer(&secret, &proof, "VRM_BOB").unwrap();
        let second = pool.transfer(&secret, &proof, "VRM_CAROL");

        assert!(matches!(second, Err(ShieldError::DoubleSpend)));
    }

    #[test]
    fn unshield_releases_locked_amount() {
        let (mut pool, _note, secret) = pool_with_note(75.0, "VRM_ALICE");
        let proof = generate_proof(&secret);

        let released = pool.unshield(&secret, &proof).unwrap();

        assert_eq!(released, 75.0);
        assert_eq!(pool.total_locked, 0.0);
        assert_eq!(pool.spent_count(), 1);
    }

    #[test]
    fn unknown_commitment_is_rejected() {
        let mut pool = ShieldedPool::new();
        let fake_secret = NoteSecret {
            amount: 10.0,
            owner: "VRM_EVE".to_string(),
            blinding: "deadbeef".to_string(),
        };
        let proof = generate_proof(&fake_secret);

        assert!(matches!(
            pool.unshield(&fake_secret, &proof),
            Err(ShieldError::UnknownCommitment)
        ));
    }

    #[test]
    fn proof_for_wrong_note_is_rejected() {
        let (mut pool, _note, secret) = pool_with_note(30.0, "VRM_ALICE");

        let other = NoteSecret {
            amount: 30.0,
            owner: "VRM_ALICE".to_string(),
            blinding: "0000".to_string(),
        };
        let wrong_proof = generate_proof(&other);

        assert!(matches!(
            pool.unshield(&secret, &wrong_proof),
            Err(ShieldError::InvalidProof)
        ));
    }
}
