/// src/chain.rs
/// Blockchain management logic
use crate::block::Block;

/// Blockchain - simply a vector of blocks with validation methods
#[derive(Debug, Clone)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
}

impl Blockchain {
    /// Creates a new blockchain with a genesis block
    pub fn new() -> Self {
        let genesis = Block::genesis();
        Blockchain {
            chain: vec![genesis],
            difficulty: 4,
        }
    }

    /// Returns the latest block in the chain
    pub fn get_latest_block(&self) -> &Block {
        self.chain.last().expect("Blockchain is empty")
    }

    /// Adds a new block with data to the chain
    pub fn add_block(&mut self, data: String) {
        let previous_hash = self.get_latest_block().hash.clone();
        let mut new_block = Block::new(self.chain.len() as u64, data, previous_hash);

        // Mine the block before adding it
        new_block.mine_block(self.difficulty);
        self.chain.push(new_block);
    }

    /// Checks integrity of the entire blockchain
    /// Returns true if all blocks are valid
    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            // 1. Check link to the previous block
            if current.previous_hash != previous.hash {
                println!("❌ Error: previous_hash mismatch!");
                return false;
            }

            // 2. Check if block hash is correct
            if current.hash != current.calculate_hash() {
                println!("❌ Error: block hash does not match data!");
                return false;
            }

            // 3. Check Proof-of-Work
            let prefix = "0".repeat(self.difficulty);
            if !current.hash.starts_with(&prefix) {
                println!("❌ Error: hash does not meet difficulty!");
                return false;
            }
        }
        true
    }

    /// Prints the chain to console (for debugging)
    pub fn print_chain(&self) {
        println!("\n📜 StubChain ({} blocks):", self.chain.len());
        println!("{}", "=".repeat(60));
        for block in &self.chain {
            println!(
                "Block #{} | Hash: {}...{} | Nonce: {}",
                block.index,
                &block.hash[..8],
                &block.hash[block.hash.len() - 8..],
                block.nonce
            );
        }
        println!("{}", "=".repeat(60));
        println!(
            "Status: {}\n",
            if self.is_valid() {
                "✅ VALID"
            } else {
                "❌ BROKEN"
            }
        );
    }
}

impl Default for Blockchain {
    fn default() -> Self {
        Self::new()
    }
}
