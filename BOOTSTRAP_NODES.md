# 🌐 AuriumChain Bootstrap Nodes

## Official Bootstrap Nodes

AuriumChain operates a globally distributed network of bootstrap nodes. Connect to any of these nodes to join the network and start mining!

---

## 🌍 Available Nodes

### 🇩🇪 Node 1 - Frankfurt, Germany (Primary)
- **IP**: `135.125.174.27`
- **P2P Port**: 3001
- **RPC Port**: 8001
- **Location**: Germany 🇩🇪
- **Uptime**: 99.9%
- **Status**: ✅ Online

### 🇨🇦 Node 2 - Beauharnois, Canada
- **IP**: `158.69.1.236`
- **P2P Port**: 3001
- **RPC Port**: 8001
- **Location**: Canada 🇨🇦
- **Uptime**: 99.9%
- **Status**: ✅ Online

### 🇮🇹 Node 3 - Milan, Italy
- **IP**: `57.131.22.120`
- **P2P Port**: 3001
- **RPC Port**: 8001
- **Location**: Milan, Italy 🇮🇹
- **Uptime**: 99.9%
- **Status**: ✅ Online

---

## 🚀 How to Connect

### Quick Start

Connect to any bootstrap node to join the network:

```bash
# Clone the repository
git clone https://github.com/Hicham60290/Auriumchain.git
cd Auriumchain

# Build
cargo build --release

# Connect to Germany node (recommended for Europe)
./target/release/auriumchain \
  --mining \
  --wallet YOUR_AUR_ADDRESS \
  --peer 135.125.174.27:3001

# OR connect to Canada node (recommended for Americas)
./target/release/auriumchain \
  --mining \
  --wallet YOUR_AUR_ADDRESS \
  --peer 158.69.1.236:3001

# OR connect to Milan node (recommended for Europe/Mediterranean)
./target/release/auriumchain \
  --mining \
  --wallet YOUR_AUR_ADDRESS \
  --peer 57.131.22.120:3001
```

---

## 🌐 Choose Your Nearest Node

**For best performance, connect to the geographically closest node:**

| Your Location | Recommended Node |
|---------------|------------------|
| 🇪🇺 Europe (West/Central) | 🇩🇪 Germany |
| 🇪🇺 Europe (South) | 🇮🇹 Milan |
| 🇬🇧 UK | 🇩🇪 Germany |
| 🇺🇸 North America | 🇨🇦 Canada |
| 🇲🇽 Latin America | 🇨🇦 Canada |
| 🇯🇵 Asia | 🇩🇪 Germany (best available) |
| 🇦🇺 Oceania | 🇨🇦 Canada (best available) |
| 🌍 Africa | 🇮🇹 Milan or 🇩🇪 Germany |

---

## ✅ Verify Connection

After connecting, verify you're syncing with the network:

```bash
# Check your chain height
curl http://localhost:8001/chain_info

# Compare with a bootstrap node
curl http://135.125.174.27:8001/chain_info

# If heights match, you're synchronized! ✅
```

---

## 🔧 Advanced Configuration

### Connect to Multiple Nodes (Future Feature)

Currently, AuriumChain connects to one peer at a time. Multi-peer support is planned for v1.2.

### Check Node Status Before Connecting

```bash
# Check Germany node
curl http://135.125.174.27:8001/status

# Check Canada node
curl http://158.69.1.236:8001/status

# Check Milan node
curl http://57.131.22.120:8001/status
```

All should return `{"status":"running"}` if online.

---

## 📊 Network Statistics

**Total Bootstrap Nodes**: 3
**Geographic Distribution**: Europe (2), North America (1)
**Total Network Hashrate**: [To be calculated]
**Active Miners**: [To be calculated]
**Current Block Height**: Check any node's `/chain_info` endpoint

---

## 🚨 Troubleshooting

### "Connection refused"

**Cause**: Node might be temporarily down or firewall issue

**Solution**:
1. Try another bootstrap node
2. Check if you can ping the IP: `ping [IP]`
3. Verify port 3001 is accessible: `telnet [IP] 3001`

### "Blockchain not syncing"

**Cause**: Network issues or version mismatch

**Solution**:
1. Ensure you're using the latest release
2. Restart your node
3. Check logs for errors: `RUST_LOG=debug ./target/release/auriumchain ...`

### Multiple bootstrap nodes available

You can try connecting to different nodes if one is slow or unreachable.

---

## 🛡️ Security Notes

- All P2P communications are **TLS 1.2+ encrypted**
- Bootstrap nodes are maintained by the AuriumChain team
- Never share your private keys with anyone
- Verify you're downloading from the official repository

---

## 📞 Support

**Node Issues?**
- Report at: https://github.com/Hicham60290/Auriumchain/issues
- Tag with `network` label

**Questions?**
- Discussions: https://github.com/Hicham60290/Auriumchain/discussions

---

## 🔮 Future Plans

- **Q1 2025**: DNS seed nodes for automatic discovery
- **Q2 2025**: Additional nodes in Asia and Oceania
- **Q3 2025**: Community-run bootstrap nodes program

---

**Last Updated**: 2025-01-18
**Network Version**: 1.0.0
