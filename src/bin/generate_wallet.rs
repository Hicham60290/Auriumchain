use clap::Parser;
use auriumchain::wallet::keys::KeyPair;
use auriumchain::wallet::address::generate_address;

#[derive(Parser, Debug)]
#[command(name = "AuriumChain Wallet Generator")]
#[command(about = "Generate a new AuriumChain wallet address")]
struct Args {
    #[arg(short = 'n', long, default_value = "1", help = "Number of addresses to generate")]
    count: usize,

    #[arg(short = 't', long, default_value = "AUR3", help = "Address type: AUR1, AUR2, or AUR3")]
    address_type: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!("AuriumChain Wallet Generator");
    println!("============================\n");

    // Validate address type
    if !["AUR1", "AUR2", "AUR3"].contains(&args.address_type.as_str()) {
        eprintln!("Error: Invalid address type. Use AUR1, AUR2, or AUR3");
        std::process::exit(1);
    }

    for i in 0..args.count {
        if args.count > 1 {
            println!("Wallet #{}", i + 1);
            println!("--------");
        }

        // Generate key pair
        let keypair = KeyPair::generate()?;

        // Generate address based on type
        let address = match args.address_type.as_str() {
            "AUR1" => generate_address(&keypair.public_key, "AUR1")?,
            "AUR2" => {
                // For quantum-resistant, we'd need different key generation
                println!("⚠️  AUR2 (quantum-resistant) requires special key generation");
                generate_address(&keypair.public_key, "AUR2")?
            },
            "AUR3" => generate_address(&keypair.public_key, "AUR3")?,
            _ => unreachable!(),
        };

        // Display wallet info
        println!("Address:      {}", address);
        println!("Public Key:   {}", hex::encode(&keypair.public_key));
        println!("Private Key:  {}", hex::encode(&keypair.secret_key));

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

    Ok(())
}
