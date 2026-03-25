/// src/main.rs
/// Entry point for RUSTubChain
mod block;
mod chain;

use chain::Blockchain;

fn main() {
    println!("🚀 RUSTubChain — Simple Tiny Understanding Blockchain\n");

    // Create a new blockchain
    let mut my_chain = Blockchain::new();

    // Add some blocks
    my_chain.add_block("First Transaction: Alice -> Bob 10 coins".to_string());
    my_chain.add_block("Second Transaction: Bob -> Charlie 5 coins".to_string());
    my_chain.add_block("Third Transaction: Charlie -> Dave 3 coins".to_string());

    // Show the result
    my_chain.print_chain();

    // Integrity demonstration
    println!("🧪 Integrity Test: Imagine changing data in Block #1...");
    println!("   The validation would fail because hashes wouldn't match.");
}
