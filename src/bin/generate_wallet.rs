use clap::Parser;
use auriumchain::wallet::keys::KeyPair;
use auriumchain::wallet::address::generate_address;

#[derive(Parser, Debug)]
#[command(name = "AuriumChain Wallet Generator")]
#[command(about = "Generate a new AuriumChain wallet address")]
struct Args {
    #[arg(short = 'n', long, default_value = "1", help = "Number of addresses to generate")]
    count: usize,
}

fn main() {
    let args = Args::parse();

    println!("AuriumChain Wallet Generator");
    println!("============================\n");

    for i in 0..args.count {
        if args.count > 1 {
            println!("Wallet #{}", i + 1);
            println!("--------");
        }

        // Generate key pair
        let keypair = KeyPair::generate();

        // Generate address (AUR1 by default)
        let address = generate_address(&keypair.public_key.serialize());

        // Display wallet info
        println!("Address:      {}", address);
        println!("Public Key:   {}", hex::encode(keypair.public_key.serialize()));
        println!("Private Key:  {}", hex::encode(keypair.private_key.secret_bytes()));

        println!("\n⚠️  IMPORTANT SECURITY WARNINGS:");
        println!("   1. NEVER share your private key with anyone!");
        println!("   2. Store your private key in a secure location");
        println!("   3. Backup your private key - if lost, funds are unrecoverable");
        println!("   4. Consider using a hardware wallet for large amounts");

        println!("\n📋 Usage:");
        println!("   To use this address for mining:");
        println!("   ./auriumchain --mining --miner-address {}", address);

        if args.count > 1 && i < args.count - 1 {
            println!("\n---\n");
        }
    }

    println!("\n✅ Wallet(s) generated successfully!");
    println!("\n🔐 NOTE: This generates AUR1 addresses (standard ECDSA)");
}
