/// AuriumChain Maximum Security Quantum Wallet Generator
///
/// Generates wallets with TRIPLE POST-QUANTUM PROTECTION:
/// - ECDSA secp256k1 (classical security)
/// - Dilithium5 (lattice-based post-quantum)
/// - SPHINCS+ (hash-based post-quantum)
///
/// WARNING: This takes ~800ms per wallet due to SPHINCS+ key generation

use clap::Parser;
use auriumchain::wallet::quantum_max::MaxSecurityWallet;
use auriumchain::blockchain::quantum_transaction::QuantumTransaction;

#[derive(Parser, Debug)]
#[command(name = "AuriumChain Quantum Wallet Generator")]
#[command(about = "Generate maximum security quantum-resistant wallets (ECDSA + Dilithium + SPHINCS+)")]
struct Args {
    #[arg(short = 'n', long, default_value = "1", help = "Number of wallets to generate")]
    count: usize,

    #[arg(short = 't', long, help = "Generate a test transaction after wallet creation")]
    test_transaction: bool,

    #[arg(short = 'v', long, help = "Verbose mode (show detailed security info)")]
    verbose: bool,
}

fn main() {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║     AURIUMCHAIN MAXIMUM SECURITY QUANTUM WALLET GENERATOR     ║");
    println!("╠═══════════════════════════════════════════════════════════════╣");
    println!("║  TRIPLE POST-QUANTUM PROTECTION:                             ║");
    println!("║  ✓ ECDSA secp256k1 (256-bit classical)                       ║");
    println!("║  ✓ Dilithium5 (NIST Level 5 post-quantum)                    ║");
    println!("║  ✓ SPHINCS+ SHA-256 (stateless post-quantum)                 ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!();

    let args = Args::parse();

    if args.count > 1 {
        println!("⏱️  Generating {} wallets (this will take ~{} seconds)...\n",
            args.count, (args.count as f64 * 0.8).ceil() as u64);
    }

    let start = std::time::Instant::now();

    for i in 0..args.count {
        if args.count > 1 {
            println!("═══════════════ Wallet #{}/{} ═══════════════", i + 1, args.count);
        }

        // Generate wallet with triple security
        let wallet = MaxSecurityWallet::new();

        println!("\n🔐 WALLET INFORMATION:");
        println!("────────────────────────────────────────────────────────");
        println!("Address:       {}", wallet.address());
        println!("────────────────────────────────────────────────────────");

        if args.verbose {
            println!("\n🛡️  SECURITY LEVEL:");
            println!("────────────────────────────────────────────────────────");
            println!("{}", wallet.security_level());
            println!("────────────────────────────────────────────────────────");
        }

        println!("\n⚠️  CRITICAL SECURITY WARNINGS:");
        println!("────────────────────────────────────────────────────────");
        println!("1. NEVER share your private keys with anyone!");
        println!("2. This wallet uses 3 private keys (ECDSA, Dilithium, SPHINCS+)");
        println!("3. ALL 3 keys are needed to sign transactions");
        println!("4. Backup securely - if lost, funds are UNRECOVERABLE");
        println!("5. Consider using hardware wallet or encrypted storage");
        println!("────────────────────────────────────────────────────────");

        println!("\n📋 USAGE:");
        println!("────────────────────────────────────────────────────────");
        println!("To receive AUR:");
        println!("  Share this address: {}", wallet.address());
        println!();
        println!("To mine with this wallet:");
        println!("  ./auriumchain --mining --miner-address {}", wallet.address());
        println!("────────────────────────────────────────────────────────");

        // Test transaction if requested
        if args.test_transaction {
            println!("\n🧪 GENERATING TEST TRANSACTION:");
            println!("────────────────────────────────────────────────────────");

            let mut tx = QuantumTransaction::new(
                wallet.address().to_string(),
                "AURtestRecipient123456789".to_string(),
                100_000_000, // 1 AUR
                0,           // nonce
                1_000_000    // 0.01 AUR fee
            );

            // Sign with triple security
            match tx.sign(&wallet) {
                Ok(_) => {
                    tx.display();
                    println!("\n✅ Test transaction signed successfully!");
                    println!("   Signature size: {} KB", tx.signature_size() / 1024);
                }
                Err(e) => {
                    println!("❌ Failed to sign test transaction: {}", e);
                }
            }
            println!("────────────────────────────────────────────────────────");
        }

        if args.count > 1 && i < args.count - 1 {
            println!("\n");
        }
    }

    let elapsed = start.elapsed();

    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║                    GENERATION COMPLETE                        ║");
    println!("╠═══════════════════════════════════════════════════════════════╣");
    println!("║  Wallets generated: {}                                         ", args.count);
    println!("║  Total time:        {:?}                                   ", elapsed);
    println!("║  Avg per wallet:    {:?}                                   ",
        elapsed / args.count as u32);
    println!("╚═══════════════════════════════════════════════════════════════╝");

    println!("\n💡 NEXT STEPS:");
    println!("────────────────────────────────────────────────────────────");
    println!("1. Backup your wallet address securely");
    println!("2. Store private keys in encrypted vault (not shown here for security)");
    println!("3. Use this address for mining or receiving AUR");
    println!("4. Test sending a transaction on testnet before mainnet");
    println!("────────────────────────────────────────────────────────────");

    println!("\n📊 SECURITY COMPARISON:");
    println!("────────────────────────────────────────────────────────────");
    println!("Standard ECDSA wallet:     Vulnerable to quantum computers");
    println!("Single post-quantum:       Quantum-resistant (1 algorithm)");
    println!("TRIPLE post-quantum (YOU): ✅ MAXIMUM SECURITY (3 algorithms)");
    println!("────────────────────────────────────────────────────────────");
    println!();
    println!("🎯 Your wallet is protected against:");
    println!("   ✓ Classical attacks");
    println!("   ✓ Quantum computer attacks (Shor's algorithm)");
    println!("   ✓ Future cryptographic breaks (redundancy)");
    println!();
}
