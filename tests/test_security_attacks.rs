use auriumchain::blockchain::{Blockchain, Block, Transaction, TxInput, TxOutput};
use auriumchain::blockchain::genesis::calculate_block_reward;
use chrono::Utc;

/// TEST 1: Tenter de miner un bloc avec récompense EXCESSIVE
#[test]
fn test_reject_excessive_mining_reward() {
    let mut blockchain = Blockchain::new();
    let genesis = blockchain.get_latest_block().clone();
    
    let malicious_coinbase = Transaction {
        id: "hack_attempt_1".to_string(),
        inputs: vec![],
        outputs: vec![TxOutput {
            value: 1_000_000_00000000,
            address: "HACKER".to_string(),
        }],
        timestamp: Utc::now().timestamp(),
        signature: String::new(),
    };
    
    let mut bad_block = Block::new(
        1,
        vec![malicious_coinbase],
        genesis.hash.clone(),
        4,
        "HACKER".to_string()
    );
    
    bad_block.mine();
    let result = blockchain.add_block(bad_block);
    
    assert!(result.is_err(), "❌ CRITICAL: Excessive reward was ACCEPTED!");
    println!("✅ Test 1 PASSED: Excessive reward rejected");
}

/// TEST 2: Bloc avec Proof of Work INVALIDE
#[test]
fn test_reject_invalid_pow() {
    let mut blockchain = Blockchain::new();
    let genesis = blockchain.get_latest_block().clone();
    
    let coinbase = Transaction {
        id: "coinbase".to_string(),
        inputs: vec![],
        outputs: vec![TxOutput {
            value: calculate_block_reward(1),
            address: "Miner".to_string(),
        }],
        timestamp: Utc::now().timestamp(),
        signature: String::new(),
    };
    
    let mut bad_block = Block::new(
        1,
        vec![coinbase],
        genesis.hash.clone(),
        4,
        "Miner".to_string()
    );
    
    bad_block.hash = "1234567890abcdef".to_string();
    bad_block.nonce = 999;
    
    let result = blockchain.add_block(bad_block);
    assert!(result.is_err(), "❌ CRITICAL: Invalid PoW accepted!");
    println!("✅ Test 2 PASSED: Invalid PoW rejected");
}

/// TEST 3: Bloc avec previous_hash INCORRECT
#[test]
fn test_reject_wrong_previous_hash() {
    let mut blockchain = Blockchain::new();
    let genesis = blockchain.get_latest_block().clone();
    
    let coinbase = Transaction {
        id: "coinbase".to_string(),
        inputs: vec![],
        outputs: vec![TxOutput {
            value: calculate_block_reward(1),
            address: "Miner".to_string(),
        }],
        timestamp: Utc::now().timestamp(),
        signature: String::new(),
    };
    
    let mut bad_block = Block::new(
        1,
        vec![coinbase],
        "0000WRONG_HASH".to_string(),
        4,
        "Miner".to_string()
    );
    
    bad_block.mine();
    let result = blockchain.add_block(bad_block);
    
    assert!(result.is_err(), "❌ CRITICAL: Wrong previous hash accepted!");
    println!("✅ Test 3 PASSED: Wrong previous hash rejected");
}

/// TEST 4: Tentative de MODIFIER le Genesis Block
#[test]
fn test_genesis_immutability() {
    let mut blockchain = Blockchain::new();
    let original_genesis_hash = blockchain.chain[0].hash.clone();
    
    blockchain.chain[0].nonce += 1;
    blockchain.chain[0].hash = blockchain.chain[0].calculate_hash();
    
    assert!(!blockchain.is_valid(), "❌ CRITICAL: Genesis block was modified!");
    
    blockchain = Blockchain::new();
    assert_eq!(blockchain.chain[0].hash, original_genesis_hash);
    
    println!("✅ Test 4 PASSED: Genesis immutability verified");
}

/// TEST 5: Index de bloc INCORRECT
#[test]
fn test_reject_wrong_index() {
    let mut blockchain = Blockchain::new();
    let genesis = blockchain.get_latest_block().clone();
    
    let coinbase = Transaction {
        id: "coinbase".to_string(),
        inputs: vec![],
        outputs: vec![TxOutput {
            value: calculate_block_reward(1),
            address: "Miner".to_string(),
        }],
        timestamp: Utc::now().timestamp(),
        signature: String::new(),
    };
    
    let mut bad_block = Block::new(
        99,
        vec![coinbase],
        genesis.hash.clone(),
        4,
        "Miner".to_string()
    );
    
    bad_block.mine();
    let result = blockchain.add_block(bad_block);
    
    assert!(result.is_err(), "❌ CRITICAL: Wrong index accepted!");
    println!("✅ Test 5 PASSED: Wrong index rejected");
}

/// TEST 6: Chaîne complète doit rester valide après plusieurs blocs
#[test]
fn test_blockchain_validity_after_multiple_blocks() {
    let mut blockchain = Blockchain::new();
    
    for i in 1..=5 {
        blockchain.mine_pending_transactions(format!("Miner_{}", i));
    }
    
    assert!(blockchain.is_valid(), "❌ CRITICAL: Blockchain became invalid!");
    assert_eq!(blockchain.chain.len(), 6); // Genesis + 5 blocs
    
    println!("✅ Test 6 PASSED: Blockchain stays valid after multiple blocks");
}

/// TEST 7: Balance calculation correcte
#[test]
fn test_balance_calculation() {
    let mut blockchain = Blockchain::new();
    
    blockchain.mine_pending_transactions("Miner1".to_string());
    blockchain.mine_pending_transactions("Miner1".to_string());
    blockchain.mine_pending_transactions("Miner2".to_string());
    
    let balance1 = blockchain.get_balance("Miner1");
    let balance2 = blockchain.get_balance("Miner2");
    
    let expected1 = calculate_block_reward(1) + calculate_block_reward(2);
    let expected2 = calculate_block_reward(3);
    
    assert_eq!(balance1, expected1, "❌ CRITICAL: Wrong balance for Miner1!");
    assert_eq!(balance2, expected2, "❌ CRITICAL: Wrong balance for Miner2!");
    
    println!("✅ Test 7 PASSED: Balance calculation correct");
}

/// TEST 8: Hash du Genesis doit TOUJOURS être identique
#[test]
fn test_deterministic_genesis() {
    let chain1 = Blockchain::new();
    let chain2 = Blockchain::new();
    let chain3 = Blockchain::new();
    
    assert_eq!(chain1.chain[0].hash, chain2.chain[0].hash);
    assert_eq!(chain2.chain[0].hash, chain3.chain[0].hash);
    
    let expected_hash = "0000521165d99d6bcd916e3ac5ecc5897084ddd0572b5de740cc55972de500d9";
    assert_eq!(chain1.chain[0].hash, expected_hash, "❌ CRITICAL: Genesis hash changed!");
    
    println!("✅ Test 8 PASSED: Genesis is deterministic");
}

/// TEST 9: Difficulté doit être respectée
#[test]
fn test_difficulty_respected() {
    let mut blockchain = Blockchain::new();
    
    blockchain.mine_pending_transactions("Miner1".to_string());
    
    let latest = blockchain.get_latest_block();
    let target = "0".repeat(blockchain.difficulty as usize);
    
    assert!(latest.hash.starts_with(&target), "❌ CRITICAL: Block doesn't meet difficulty!");
    
    println!("✅ Test 9 PASSED: Difficulty respected");
}

/// TEST 10: Stress test - 20 blocs consécutifs
#[test]
fn test_stress_20_blocks() {
    let mut blockchain = Blockchain::new();
    
    for i in 1..=20 {
        blockchain.mine_pending_transactions(format!("Miner_{}", i % 3));
    }
    
    assert_eq!(blockchain.chain.len(), 21); // Genesis + 20
    assert!(blockchain.is_valid(), "❌ CRITICAL: Chain invalid after stress test!");
    
    println!("✅ Test 10 PASSED: Stress test with 20 blocks succeeded");
}
