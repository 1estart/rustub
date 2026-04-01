/// src/main.rs
/// Entry point for RUSTubChain
mod block;
mod chain;
mod transaction;

use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

use chain::Blockchain;
use transaction::Transaction;

fn handle_peer(mut stream: TcpStream, chain_shared: std::sync::Arc<std::sync::Mutex<Blockchain>>) {
    let peer_addr = stream
        .peer_addr()
        .map(|a| a.to_string())
        .unwrap_or_else(|_| "unknown".into());
    println!("🔗 [PEER] Connected: {}", peer_addr);

    let mut buf = [0u8; 1024];
    loop {
        match stream.read(&mut buf) {
            Ok(0) => {
                println!("🔌 [PEER] Disconnected: {}", peer_addr);
                break;
            }
            Ok(n) => {
                // Simple protocol - string "TX:from:to:amount"
                let msg = String::from_utf8_lossy(&buf[..n]).trim().to_string();
                println!("📡 [PEER] Received from {}: {}", peer_addr, msg);

                if msg.starts_with("TX:") {
                    let parts: Vec<&str> = msg[3..].split(':').collect();
                    if parts.len() == 3 {
                        if let (Ok(amount), from, to) =
                            (parts[2].parse::<u64>(), parts[0], parts[1])
                        {
                            let tx = Transaction::new(from.into(), to.into(), amount);
                            if let Ok(mut chain) = chain_shared.lock() {
                                chain.add_transaction(tx);
                            }
                        }
                    }
                }
                // We can add "GETCHAIN", "BLOCK", etc.
            }
            Err(e) => {
                println!("❌ [PEER] Read error from {}: {}", peer_addr, e);
                break;
            }
        }
    }
}

fn main() {
    println!("🚀 RUSTubChain Node — Simple P2P (log mode)\n");

    // Load/init blockchain
    let mut chain = Blockchain::load_from_file("chain.dat").unwrap_or_else(Blockchain::new);

    // Check integrity on start
    if !chain.is_valid() {
        println!("⚠️ Chain is invalid! Starting fresh with genesis.");
        chain = Blockchain::new();
    }
    chain.print_chain();

    // Shared state for threads
    let chain_shared = std::sync::Arc::new(std::sync::Mutex::new(chain));

    // Load TCP-server ( simple, blocking, in standalone thread )
    let listener = TcpListener::bind("0.0.0.0:3000").expect("Failed to bind port 3000");
    println!("🔊 [NET] Listening for peers on 0.0.0.0:3000\n");

    let chain_clone = std::sync::Arc::clone(&chain_shared);
    thread::spawn(move || {
        for stream in listener.incoming() {
            match stream {
                Ok(s) => {
                    let chain_for_peer = std::sync::Arc::clone(&chain_clone);
                    thread::spawn(move || handle_peer(s, chain_for_peer));
                }
                Err(e) => eprintln!("❌ [NET] Accept error: {}", e),
            }
        }
    });

    // Main cycle - wating for events
    let mut block_counter = 0;
    loop {
        thread::sleep(Duration::from_secs(10));

        // Auto-mining every 30 seconds
        block_counter += 1;
        if block_counter % 3 == 0 {
            if let Ok(mut chain) = chain_shared.lock() {
                if !chain.mempool.is_empty() {
                    println!(
                        "⏰ [MINER] Time to mine! Mempool size: {}",
                        chain.mempool.len()
                    );
                    chain.mine_mempool("node_reward".into());
                    chain.print_chain();
                    chain.save_to_file("chain.dat"); // stub
                } else {
                    println!("⏰ [MINER] No transactions, waiting...");
                }
            }
        }

        // Can add
        // - mempool asking
        // - sending new blocks to peers
        // - processing console command
        println!(
            "🔄 [LOOP] Node alive, mempool: {} txs",
            chain_shared.lock().unwrap().mempool.len()
        );
    }
}
