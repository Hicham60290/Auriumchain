# AuriumChain

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![Security](https://img.shields.io/badge/security-audited-green.svg)](SECURITY.md)

**AuriumChain** is a production-ready, high-security blockchain implementation written in Rust. It features a complete Proof of Work consensus mechanism, TLS-encrypted P2P networking, UTXO-based transactions, and comprehensive security protections.

## Table of Contents

- [Features](#features)
- [Architecture](#architecture)
- [Quick Start](#quick-start)
- [Installation](#installation)
- [Usage](#usage)
- [Security](#security)
- [Development](#development)
- [Contributing](#contributing)
- [License](#license)
- [Roadmap](#roadmap)

## Features

### Core Blockchain
- **Proof of Work (PoW)** consensus with configurable difficulty
- **UTXO-based transaction model** for efficient and secure payments
- **Genesis block** with deterministic generation
- **Dynamic fee system** based on transaction size
- **Block rewards** with automatic halving mechanism
- **Full chain validation** with cryptographic integrity checks

### Security
- **TLS 1.2+ encryption** for all network communications
- **secp256k1** elliptic curve cryptography for signatures
- **SHA-256/SHA-3** cryptographic hashing
- **Argon2** password hashing for wallet encryption
- **AES-GCM** encryption for sensitive data
- **Quantum-resistant** cryptography support
- **Comprehensive security test suite** (10/10 tests passing)
- **Attack protection** against double-spending, replay attacks, and chain manipulation

### Network & Communication
- **P2P networking** with automatic peer discovery
- **TLS-encrypted** peer-to-peer communication
- **Real-time block synchronization** across nodes
- **HTTP/REST RPC server** for node interaction
- **Multi-node support** with geographic redundancy
- **Resilient peer management** with automatic reconnection

### Wallet & Cryptography
- **Secure wallet** with password-based encryption
- **HD wallet** support (Hierarchical Deterministic)
- **Address generation** with Base58Check encoding
- **Balance tracking** with UTXO management
- **Transaction creation** and signing
- **Key import/export** capabilities

### Storage & Performance
- **RocksDB** for high-performance blockchain storage
- **Efficient UTXO indexing** for fast lookups
- **Persistent blockchain state** across restarts
- **Optimized memory usage** for long-running nodes

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      AuriumChain Node                       │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌─────────────┐  ┌──────────────┐  ┌─────────────────┐   │
│  │   P2P TLS   │  │  RPC Server  │  │  Mining Engine  │   │
│  │  (Port 8333)│  │  (Port 8545) │  │   (PoW/PoS)     │   │
│  └──────┬──────┘  └──────┬───────┘  └────────┬────────┘   │
│         │                │                   │            │
│         └────────────────┼───────────────────┘            │
│                          │                                │
│  ┌──────────────────────┴───────────────────────────┐    │
│  │           Blockchain Core Engine                  │    │
│  │  ┌─────────┐  ┌──────────┐  ┌────────────────┐  │    │
│  │  │  Chain  │  │   UTXO   │  │  Transactions  │  │    │
│  │  │Validator│  │  Manager │  │   Validator    │  │    │
│  │  └─────────┘  └──────────┘  └────────────────┘  │    │
│  └───────────────────────┬──────────────────────────┘    │
│                          │                                │
│  ┌──────────────────────┴───────────────────────────┐    │
│  │         Security & Cryptography Layer             │    │
│  │   secp256k1 | SHA-256/3 | AES-GCM | Argon2       │    │
│  └───────────────────────┬──────────────────────────┘    │
│                          │                                │
│  ┌──────────────────────┴───────────────────────────┐    │
│  │            Storage Layer (RocksDB)                │    │
│  │   Blocks | Transactions | UTXO Set | Indexes     │    │
│  └───────────────────────────────────────────────────┘    │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### Module Structure

- **`blockchain/`** - Core blockchain logic (blocks, chain, transactions, UTXO)
- **`wallet/`** - Wallet management, keys, addresses, encryption
- **`p2p/`** - Peer-to-peer networking with TLS
- **`rpc/`** - HTTP/REST API server for node communication
- **`mining/`** - Proof of Work mining engine
- **`security/`** - Attack protection, monitoring, validation
- **`storage/`** - RocksDB persistence layer
- **`utils/`** - Cryptography and configuration utilities

## Quick Start

### Prerequisites

- **Rust** 1.70 or higher
- **Cargo** (comes with Rust)
- **OpenSSL** development libraries

### Build from Source

```bash
# Clone the repository
git clone https://github.com/Hicham60290/Auriumchain.git
cd Auriumchain

# Build the project
cargo build --release

# Run tests
cargo test

# Run the node
./target/release/auriumchain
```

### Run a Node

```bash
# Start a mining node
./target/release/auriumchain --mining

# Start a node on custom ports
./target/release/auriumchain --port 3001 --rpc-port 8001

# Connect to a peer
./target/release/auriumchain --peer 192.168.1.100:8333

# Run multiple peers
./target/release/auriumchain --mining --port 3001 --rpc-port 8001 --peer PEER_IP:PORT
```

### Create a Wallet

```bash
# Create a new secure wallet
./target/release/secure_wallet

# Follow the interactive prompts to:
# 1. Set a password
# 2. Generate keys
# 3. Create an address
# 4. View your wallet information
```

## Installation

For detailed installation instructions, including platform-specific requirements and troubleshooting, see [docs/installation.md](docs/installation.md).

## Usage

### Mining

Start mining blocks to secure the network and earn rewards:

```bash
./target/release/auriumchain --mining
```

### RPC API

The RPC server exposes HTTP endpoints for blockchain interaction:

```bash
# Get blockchain info
curl http://localhost:8545/info

# Get block by height
curl http://localhost:8545/block/0

# Get transaction by hash
curl http://localhost:8545/tx/<hash>
```

### P2P Network

Connect to other nodes to participate in the distributed network:

```bash
# Connect to a specific peer
./target/release/auriumchain --peer 192.168.1.100:8333

# Your node will automatically sync blocks
```

## Security

AuriumChain takes security seriously. We have:

- **Comprehensive test suite** with 10 security-focused tests
- **TLS 1.2+ encryption** for all network traffic
- **Industry-standard cryptography** (secp256k1, SHA-256/3, AES-GCM, Argon2)
- **Attack protection** against common blockchain vulnerabilities
- **Bug bounty program** - See [SECURITY.md](SECURITY.md) for details

### Reporting Security Issues

Please report security vulnerabilities to the contact information provided in [SECURITY.md](SECURITY.md). Do not open public issues for security concerns.

## Development

### Project Structure

```
Auriumchain/
├── src/
│   ├── blockchain/      # Core blockchain implementation
│   ├── wallet/          # Wallet and cryptography
│   ├── p2p/             # P2P networking with TLS
│   ├── rpc/             # HTTP RPC server
│   ├── mining/          # Proof of Work engine
│   ├── security/        # Security and validation
│   ├── storage/         # Database layer
│   ├── utils/           # Utilities and config
│   └── main.rs          # Entry point
├── tests/               # Integration and security tests
├── docs/                # Documentation
└── Cargo.toml           # Dependencies
```

### Running Tests

```bash
# Run all tests
cargo test

# Run security tests specifically
cargo test test_security

# Run with output
cargo test -- --nocapture
```

### Code Quality

```bash
# Format code
cargo fmt

# Run linter
cargo clippy

# Check for security vulnerabilities
cargo audit
```

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on:

- Code style and standards
- Pull request process
- Issue reporting
- Development workflow
- Testing requirements

Please also read our [Code of Conduct](CODE_OF_CONDUCT.md) before contributing.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

Copyright (c) 2025 Hicham60290

## Roadmap

### Planned Features

- [ ] **Smart Contracts** - Turing-complete scripting language
- [ ] **Multi-signature Wallets** - Enhanced security for high-value transactions
- [ ] **Light Clients** - SPV clients for mobile and low-resource devices
- [ ] **Mining Pools** - Cooperative mining support
- [ ] **Web Wallet Interface** - Browser-based wallet management
- [ ] **Mobile Applications** - iOS and Android wallet apps
- [ ] **Cross-chain Bridges** - Interoperability with other blockchains
- [ ] **Sharding** - Horizontal scaling for increased throughput
- [ ] **Layer 2 Solutions** - Lightning Network-style payment channels

### Version History

See [CHANGELOG.md](CHANGELOG.md) for a detailed history of changes.

---

**Built with** ❤️ **by the AuriumChain community**

For questions, discussions, or support, please open an issue on GitHub.
