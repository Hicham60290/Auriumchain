use auriumchain::blockchain::Blockchain;
use auriumchain::storage::db::BlockchainDB;
use clap::Parser;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(name = "AuriumChain RocksDB Migration Tool")]
#[command(about = "Migrate blockchain from JSON to RocksDB")]
struct Args {
    #[arg(long, help = "Path to JSON blockchain file")]
    json_file: String,

    #[arg(long, help = "Path to RocksDB directory")]
    rocksdb_path: String,

    #[arg(long, help = "Verify migration after completion")]
    verify: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let args = Args::parse();

    println!("AuriumChain - JSON to RocksDB Migration Tool");
    println!("==============================================\n");

    // Vérifier que le fichier JSON existe
    if !Path::new(&args.json_file).exists() {
        eprintln!("Error: JSON file not found: {}", args.json_file);
        std::process::exit(1);
    }

    println!("1. Loading blockchain from JSON file: {}", args.json_file);
    let blockchain = match Blockchain::load_from_file(&args.json_file) {
        Ok(chain) => {
            println!("   ✓ Successfully loaded {} blocks", chain.chain.len());
            chain
        },
        Err(e) => {
            eprintln!("   ✗ Failed to load blockchain: {}", e);
            std::process::exit(1);
        }
    };

    // Calculer le total de AUR
    let mut total_aur = 0u64;
    for block in &blockchain.chain {
        for tx in &block.transactions {
            for output in &tx.outputs {
                total_aur += output.value;
            }
        }
    }

    println!("\n2. Blockchain Statistics:");
    println!("   - Total Blocks: {}", blockchain.chain.len());
    println!("   - Total AUR: {} AUR", total_aur as f64 / 1_000_000.0);
    println!("   - Genesis Block: {}", blockchain.chain.first().map(|b| &b.hash).unwrap_or(&"N/A".to_string()));
    println!("   - Latest Block: {}", blockchain.chain.last().map(|b| &b.hash).unwrap_or(&"N/A".to_string()));

    println!("\n3. Opening RocksDB at: {}", args.rocksdb_path);
    let db = match BlockchainDB::open(&args.rocksdb_path) {
        Ok(database) => {
            println!("   ✓ RocksDB opened successfully");
            database
        },
        Err(e) => {
            eprintln!("   ✗ Failed to open RocksDB: {}", e);
            std::process::exit(1);
        }
    };

    println!("\n4. Migrating blocks to RocksDB...");
    match db.save_blockchain(&blockchain) {
        Ok(_) => {
            println!("   ✓ Successfully migrated {} blocks", blockchain.chain.len());
        },
        Err(e) => {
            eprintln!("   ✗ Migration failed: {}", e);
            std::process::exit(1);
        }
    }

    // Migration des UTXOs
    println!("\n5. Building UTXO set...");
    let mut utxo_count = 0;
    for block in &blockchain.chain {
        for tx in &block.transactions {
            let tx_hash = &tx.id;
            for (idx, output) in tx.outputs.iter().enumerate() {
                if let Err(e) = db.save_utxo(tx_hash, idx as u32, output.value, &output.address) {
                    eprintln!("   Warning: Failed to save UTXO: {}", e);
                } else {
                    utxo_count += 1;
                }
            }
        }
    }
    println!("   ✓ Created {} UTXOs", utxo_count);

    // Compacter la base de données
    println!("\n6. Compacting database...");
    if let Err(e) = db.compact() {
        eprintln!("   Warning: Compaction failed: {}", e);
    } else {
        println!("   ✓ Database compacted");
    }

    // Vérification optionnelle
    if args.verify {
        println!("\n7. Verifying migration...");

        match db.get_chain_height() {
            Ok(height) => {
                if height as usize == blockchain.chain.len() {
                    println!("   ✓ Chain height correct: {}", height);
                } else {
                    eprintln!("   ✗ Chain height mismatch! Expected {}, got {}", blockchain.chain.len(), height);
                    std::process::exit(1);
                }
            },
            Err(e) => {
                eprintln!("   ✗ Failed to verify chain height: {}", e);
                std::process::exit(1);
            }
        }

        // Vérifier quelques blocs aléatoirement
        let verify_blocks = [0, blockchain.chain.len() / 2, blockchain.chain.len() - 1];
        for &idx in &verify_blocks {
            match db.get_block(idx as u64) {
                Ok(Some(block)) => {
                    if block.hash == blockchain.chain[idx].hash {
                        println!("   ✓ Block {} verified", idx);
                    } else {
                        eprintln!("   ✗ Block {} hash mismatch!", idx);
                        std::process::exit(1);
                    }
                },
                Ok(None) => {
                    eprintln!("   ✗ Block {} not found in RocksDB!", idx);
                    std::process::exit(1);
                },
                Err(e) => {
                    eprintln!("   ✗ Failed to verify block {}: {}", idx, e);
                    std::process::exit(1);
                }
            }
        }
    }

    // Statistiques finales
    println!("\n8. Final Database Statistics:");
    match db.get_stats() {
        Ok(stats) => println!("{}", stats),
        Err(e) => eprintln!("   Warning: Could not get stats: {}", e),
    }

    println!("\n✅ Migration completed successfully!");
    println!("\nNext steps:");
    println!("  1. Backup your JSON file: cp {} {}.backup", args.json_file, args.json_file);
    println!("  2. Update your node to use RocksDB: --rocksdb-path {}", args.rocksdb_path);
    println!("  3. Test the node with RocksDB before removing JSON backup");

    Ok(())
}
