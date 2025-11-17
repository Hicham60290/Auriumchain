use auriumchain::wallet::secure_wallet::SecureWallet;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "auriumchain-secure-wallet")]
#[command(about = "AuriumChain Ultra-Secure Wallet Manager 🔐", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Generate {
        #[arg(short, long)]
        name: String,

        #[arg(short, long, default_value = "AUR3")]
        type_addr: String,
    },
    Show {
        #[arg(short, long)]
        name: String,
    },
    ExportSeed {
        #[arg(short, long)]
        name: String,
    },
    Verify {
        #[arg(short, long)]
        name: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Generate { name, type_addr } => {
            generate_secure_wallet(name, type_addr);
        }
        Commands::Show { name } => {
            show_wallet(name);
        }
        Commands::ExportSeed { name } => {
            export_seed(name);
        }
        Commands::Verify { name } => {
            verify_wallet(name);
        }
    }
}

fn generate_secure_wallet(name: &str, addr_type: &str) {
    println!("\n╔════════════════════════════════════════════════╗");
    println!("║   🔐 ULTRA-SECURE WALLET GENERATION 🔐        ║");
    println!("╚════════════════════════════════════════════════╝\n");

    println!("🔑 Enter a STRONG password (min 16 characters):");
    let password = rpassword::prompt_password("Password: ").unwrap();

    if password.len() < 16 {
        println!("❌ Password too weak! Minimum 16 characters required.");
        return;
    }

    println!("🔑 Confirm password:");
    let confirm = rpassword::prompt_password("Password: ").unwrap();

    if password != confirm {
        println!("❌ Passwords don't match!");
        return;
    }

    println!("\n⏳ Generating ultra-secure wallet...\n");

    let wallet = match SecureWallet::generate(name.to_string(), &password, addr_type) {
        Ok(w) => w,
        Err(e) => {
            println!("❌ Error: {}", e);
            return;
        }
    };

    match wallet.save("wallets") {
        Ok(filename) => {
            println!("\n✅ Wallet saved: {}\n", filename);
            wallet.security_info();

            println!("📍 Your Address:");
            println!("   {}\n", wallet.address);

            println!("⚠️  SECURITY REMINDERS:");
            println!("   1. Your seed phrase is your ULTIMATE backup");
            println!("   2. NEVER share your password or seed");
            println!("   3. Store the seed phrase on PAPER in a SAFE");
            println!("   4. Make MULTIPLE backups of the wallet file");
            println!("   5. Test recovery BEFORE sending funds\n");
        }
        Err(e) => println!("❌ Failed to save: {}", e),
    }
}

fn show_wallet(name: &str) {
    let filename = format!("wallets/{}.secure.wallet", name);

    let wallet = match SecureWallet::load(&filename) {
        Ok(w) => w,
        Err(e) => {
            println!("❌ Error: {}", e);
            return;
        }
    };

    println!("\n╔════════════════════════════════════════════════╗");
    println!("║            WALLET INFORMATION                  ║");
    println!("╠════════════════════════════════════════════════╣");
    println!("║  Name    : {:<38} ║", wallet.name);
    println!("║  Type    : {:<38} ║", wallet.address_type);
    println!(
        "║  Address : {:<38} ║",
        &wallet.address[..40.min(wallet.address.len())]
    );
    if wallet.address.len() > 40 {
        println!("║            {:<38} ║", &wallet.address[40..]);
    }
    println!("║  Created : {:<38} ║", &wallet.created_at[..19]);
    println!("╚════════════════════════════════════════════════╝\n");

    wallet.security_info();
}

fn export_seed(name: &str) {
    println!("\n⚠️  WARNING: You are about to export your SEED PHRASE!");
    println!("⚠️  This is the MOST SENSITIVE information!");
    println!("⚠️  Make sure nobody is watching your screen!\n");

    println!("Type 'I UNDERSTAND THE RISKS' to continue:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    if input.trim() != "I UNDERSTAND THE RISKS" {
        println!("❌ Cancelled.");
        return;
    }

    let filename = format!("wallets/{}.secure.wallet", name);
    let wallet = match SecureWallet::load(&filename) {
        Ok(w) => w,
        Err(e) => {
            println!("❌ Error: {}", e);
            return;
        }
    };

    println!("\n🔑 Enter wallet password:");
    let password = rpassword::prompt_password("Password: ").unwrap();

    match wallet.decrypt_seed(&password) {
        Ok(seed) => {
            println!("\n╔════════════════════════════════════════════════╗");
            println!("║              🔐 SEED PHRASE 🔐                 ║");
            println!("╠════════════════════════════════════════════════╣");
            println!("║                                                ║");

            let words: Vec<&str> = seed.split_whitespace().collect();
            for (i, word) in words.iter().enumerate() {
                if i % 4 == 0 {
                    print!("║  ");
                }
                print!("{:2}. {:<10} ", i + 1, word);
                if (i + 1) % 4 == 0 {
                    println!("║");
                }
            }

            println!("║                                                ║");
            println!("╚════════════════════════════════════════════════╝\n");
        }
        Err(e) => println!("❌ Error: {}", e),
    }
}

fn verify_wallet(name: &str) {
    let filename = format!("wallets/{}.secure.wallet", name);

    let wallet = match SecureWallet::load(&filename) {
        Ok(w) => w,
        Err(e) => {
            println!("❌ Error: {}", e);
            return;
        }
    };

    if wallet.verify_integrity() {
        println!("✅ Wallet integrity: VERIFIED");
        println!("✅ Wallet has NOT been tampered with");
    } else {
        println!("❌ WARNING: Wallet integrity check FAILED!");
        println!("❌ Wallet may have been tampered with!");
        println!("⚠️  DO NOT USE THIS WALLET!");
    }
}
