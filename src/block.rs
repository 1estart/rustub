/// src/block.rs
/// Basic block structure for StubChain
use crate::transaction::Transaction;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// Block structure - the fundamental building element of the blockchain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    /// Position in the chain (0 = genesis block)
    pub index: u64,

    /// Creation time (milliseconds since UNIX epoch)
    pub timestamp: u128,

    /// Hash of the previous block - this creates the "chain"
    /// If the previous block changes, this hash becomes invalid
    pub previous_hash: String,

    /// Transactions
    pub transactions: Vec<Transaction>,

    /// Number changed during mining to find a valid hash
    pub nonce: u64,

    /// Hash of the current block (computed from all fields above)
    hash: String,
}

impl Block {
    /// Creates a new block with automatic hash computation
    pub fn new(index: u64, transactions: Vec<Transaction>, previous_hash: String) -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();

        let mut block = Block {
            index,
            timestamp,
            previous_hash,
            transactions,
            nonce: 0,
            hash: String::new(), // Empty initially
        };

        // Compute hash immediately
        block.hash = block.calculate_hash();
        block
    }

    /// Computes SHA-256 hash of the block content
    /// Important: We hash everything except the `hash` field itself
    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();

        let txs = serde_json::to_string(&self.transactions).unwrap_or_default();
        // Combine all data into a single string
        let input = format!(
            "{}{}{}{}{}",
            self.index, self.timestamp, self.previous_hash, txs, self.nonce
        );

        hasher.update(input.as_bytes());
        let result = hasher.finalize();

        // Convert bytes to hex string (readable format)
        hex::encode(result)
    }

    /// Proof-of-Work: finds nonce so hash starts with zeros
    /// `difficulty` - number of leading zeros required (e.g., 4)
    pub fn mine_block(&mut self, difficulty: usize) {
        let prefix = "0".repeat(difficulty);

        // Increment nonce until hash starts with the required prefix
        while !self.hash.starts_with(&prefix) {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }

        println!("⛏️ Block mined! Nonce: {}, Hash: {}", self.nonce, self.hash);
    }

    /// Creates the first block in the chain (Genesis)
    pub fn genesis() -> Self {
        let mut genesis = Block::new(0, vec![], "0".to_string());
        genesis.mine_block(4); // Easy difficulty for startup
        genesis
    }

    pub fn get_hash(&self) -> &String {
        &self.hash
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_hash_is_consistent() {
        let block = Block::new(1, vec![], "0".to_string());
        let hash1 = block.get_hash().clone();
        let hash2 = block.calculate_hash();
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_mining_produces_valid_hash() {
        let mut block = Block::new(1, vec![], "0".to_string());
        block.mine_block(3);
        assert!(block.get_hash().starts_with("000"));
    }
}
