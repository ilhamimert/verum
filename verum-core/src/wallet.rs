use k256::ecdsa::{SigningKey, VerifyingKey, Signature, signature::Signer};
use k256::ecdsa::signature::Verifier;
use rand::rngs::OsRng;
use sha2::{Digest, Sha256};
use ripemd::Ripemd160;
use serde::{Deserialize, Serialize};
use crate::transaction::{Transaction, TransactionType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletInfo {
    pub address: String,
    pub public_key: String,
}

pub struct Wallet {
    signing_key: SigningKey,
    pub info: WalletInfo,
}

impl Wallet {
    pub fn new() -> Self {
        let signing_key = SigningKey::random(&mut OsRng);
        let verifying_key = VerifyingKey::from(&signing_key);

        let address = Self::derive_address(&verifying_key);
        let public_key = hex::encode(verifying_key.to_encoded_point(true).as_bytes());

        Wallet {
            signing_key,
            info: WalletInfo { address, public_key },
        }
    }

    // Bitcoin tarzı adres türetme: SHA256 → RIPEMD160 → Base58Check
    fn derive_address(verifying_key: &VerifyingKey) -> String {
        let pub_bytes = verifying_key.to_encoded_point(true);

        // SHA256 hash
        let sha_hash = Sha256::digest(pub_bytes.as_bytes());

        // RIPEMD160 hash
        let ripe_hash = Ripemd160::digest(&sha_hash);

        // VRM prefix (0x56 = 'V')
        let mut payload = vec![0x56u8];
        payload.extend_from_slice(&ripe_hash);

        // Checksum (double SHA256 ilk 4 byte)
        let check1 = Sha256::digest(&payload);
        let check2 = Sha256::digest(&check1);
        payload.extend_from_slice(&check2[..4]);

        // Base58 encode
        let encoded = base58::ToBase58::to_base58(payload.as_slice());
        format!("VRM{}", encoded)
    }

    pub fn sign_transaction(&self, tx: &Transaction) -> String {
        let message = format!(
            "{}{}{}{}",
            tx.id, tx.from, tx.to, tx.amount
        );
        let signature: Signature = self.signing_key.sign(message.as_bytes());
        hex::encode(signature.to_bytes())
    }

    pub fn verify_signature(
        public_key_hex: &str,
        tx: &Transaction,
        signature_hex: &str,
    ) -> bool {
        let pub_bytes = match hex::decode(public_key_hex) {
            Ok(b) => b,
            Err(_) => return false,
        };

        let verifying_key = match VerifyingKey::from_sec1_bytes(&pub_bytes) {
            Ok(k) => k,
            Err(_) => return false,
        };

        let sig_bytes = match hex::decode(signature_hex) {
            Ok(b) => b,
            Err(_) => return false,
        };

        let signature = match Signature::from_slice(&sig_bytes) {
            Ok(s) => s,
            Err(_) => return false,
        };

        let message = format!(
            "{}{}{}{}",
            tx.id, tx.from, tx.to, tx.amount
        );

        verifying_key.verify(message.as_bytes(), &signature).is_ok()
    }

    pub fn create_transaction(&self, to: &str, amount: f64) -> Transaction {
        let mut tx = Transaction::new(
            &self.info.address,
            to,
            amount,
            TransactionType::Transfer,
        );
        let signature = self.sign_transaction(&tx);
        tx.signature = signature;
        tx
    }

    pub fn export_info(&self) -> String {
        serde_json::to_string_pretty(&self.info).unwrap_or_default()
    }
}

impl Default for Wallet {
    fn default() -> Self {
        Self::new()
    }
}
