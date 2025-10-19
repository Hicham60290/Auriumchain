# Installation Guide

## System Requirements

### Minimum
- **OS**: Ubuntu 22.04+ / Debian 11+ / Other Linux
- **CPU**: 2 cores
- **RAM**: 4 GB
- **Storage**: 50 GB SSD
- **Network**: Stable internet connection

### Recommended
- **CPU**: 4+ cores
- **RAM**: 8 GB
- **Storage**: 100 GB SSD

## Step-by-Step Installation

### 1. Install Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustc --version  # Verify installation
```

### 2. Install Dependencies
```bash
sudo apt update
sudo apt install -y build-essential git pkg-config libssl-dev
```

### 3. Clone Repository
```bash
git clone https://github.com/Hicham60290/Auriumchain.git
cd Auriumchain
```

### 4. Build
```bash
cargo build --release
# This takes 10-20 minutes on first build
```

### 5. Install Binary
```bash
sudo cp target/release/auriumchain /usr/local/bin/
sudo chmod +x /usr/local/bin/auriumchain
```

### 6. Test
```bash
auriumchain --help
```

## Running a Node

### Genesis Node (First Node)
```bash
auriumchain \
    --genesis \
    --port 3001 \
    --rpc-port 8001 \
    --mining \
    --wallet YOUR_AUR3_ADDRESS
```

### Regular Node (Connect to Network)
```bash
auriumchain \
    --port 3002 \
    --rpc-port 8002 \
    --mining \
    --wallet YOUR_AUR3_ADDRESS \
    --peer GENESIS_IP:3001
```

## Systemd Service (Production)

### Create Service File
```bash
sudo nano /etc/systemd/system/auriumchain.service
```
```ini
[Unit]
Description=Auriumchain Node
After=network.target

[Service]
Type=simple
User=youruser
WorkingDirectory=/home/youruser
ExecStart=/usr/local/bin/auriumchain \
    --mining \
    --wallet YOUR_ADDRESS \
    --port 3001 \
    --rpc-port 8001
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

### Enable and Start
```bash
sudo systemctl daemon-reload
sudo systemctl enable auriumchain
sudo systemctl start auriumchain
sudo systemctl status auriumchain
```

### View Logs
```bash
sudo journalctl -u auriumchain -f
```

## Firewall Configuration
```bash
sudo ufw allow 3001/tcp comment 'Auriumchain P2P'
sudo ufw allow 8001/tcp comment 'Auriumchain RPC'
sudo ufw enable
```

## Verification
```bash
# Check node status
curl http://localhost:8001/status

# Should return JSON with blockchain info
```

## Troubleshooting

### Build Errors
```bash
# Update Rust
rustup update

# Clean and rebuild
cargo clean
cargo build --release
```

### Connection Issues

- Check firewall rules
- Verify ports are not in use: `sudo netstat -tlnp`
- Check logs: `journalctl -u auriumchain -n 50`

## Next Steps

- Read the main README for more features
- Join the community on GitHub Discussions
- Report issues on GitHub Issues

---

Need help? Open an issue on GitHub!
