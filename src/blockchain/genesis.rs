use super::block::{Block, Transaction, TxOutput};

pub const GENESIS_TIMESTAMP: i64 = 1729382400;
pub const TOTAL_SUPPLY: u64 = 21_000_000_00000000;

pub fn create_genesis_block() -> Block {
    println!("╔════════════════════════════════════════════════╗");
    println!("║          AURIUMCHAIN GENESIS BLOCK             ║");
    println!("║                                                ║");
    println!("║  🔒 Immutable & Autonomous Protocol           ║");
    println!("║  ⚖️  Code is Law                              ║");
    println!("║  🌍 True Decentralization                     ║");
    println!("║                                                ║");
    println!("║  Supply:    21,000,000 AUR                    ║");
    println!("║  Halving:   Every 4 years                     ║");
    println!("║  Block:     30 seconds                        ║");
    println!("║  Consensus: Optimized PoW                     ║");
    println!("║                                                ║");
    println!("║  Launched:  2025-10-20 00:00:00 UTC          ║");
    println!("║  Creator:   Anonymous                         ║");
    println!("║  Control:   NONE                              ║");
    println!("╚════════════════════════════════════════════════╝");

    let genesis_message = "AuriumChain Genesis - October 20, 2025 - Autonomous & Decentralized";

    let genesis_tx = Transaction {
        id: "genesis".to_string(),
        inputs: vec![],
        outputs: vec![TxOutput {
            value: 0,
            address: genesis_message.to_string(),
        }],
        timestamp: GENESIS_TIMESTAMP,
        signature: String::new(),
    };

    let mut genesis = Block {
        index: 0,
        timestamp: GENESIS_TIMESTAMP,
        transactions: vec![genesis_tx],
        previous_hash: "0".repeat(64),
        hash: String::new(),
        nonce: 0,
        difficulty: 4,
        miner_address: "GENESIS_ANONYMOUS".to_string(),
        merkle_root: String::new(),
    };

    genesis.merkle_root = Block::calculate_merkle_root(&genesis.transactions);
    genesis.mine();

    println!("\n🌟 Genesis Block Created!");
    println!("   Hash: {}", genesis.hash);
    println!("\n✨ AuriumChain is now ALIVE and AUTONOMOUS!\n");

    genesis
}

pub fn calculate_block_reward(block_height: u64) -> u64 {
    const INITIAL_REWARD: u64 = 50_00000000;
    const HALVING_INTERVAL: u64 = 4_204_800;

    let halvings = block_height / HALVING_INTERVAL;

    if halvings >= 64 {
        return 0;
    }

    INITIAL_REWARD >> halvings
}
