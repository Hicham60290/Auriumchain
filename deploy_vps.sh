#!/bin/bash
# AuriumChain VPS Deployment Script
# Usage: ./deploy_vps.sh [vps1|vps2|vps3]

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
GITHUB_REPO="https://github.com/Hicham60290/Auriumchain.git"
BRANCH="claude/review-auriumchain-history-011CULocdT6NysTPpkvXH2K1"
INSTALL_DIR="$HOME/auriumchain-core"

# VPS wallet addresses - Classic ECDSA wallets (AUR1...)
VPS1_WALLET="AUR1Z9uNKX94ZRv7zgTN9HHGWdMzu8BSssRP9A"
VPS2_WALLET="AUR1Yx1Xx9teinx3VpLuZGwQ4ns5sH1yfnfMBa"
VPS3_WALLET="AUR1YsVs3WKhMSmdyN3NXP4qwq3Huz4BuAHEoB"

# Peer addresses
VPS1_IP="85.190.98.161"
VPS2_IP="192.162.86.5"
VPS3_IP="192.162.86.32"

# RocksDB storage path (production mode)
DATA_DIR="$HOME/auriumchain-data"
ROCKSDB_PATH="$DATA_DIR/rocksdb"

usage() {
    echo "Usage: $0 [vps1|vps2|vps3]"
    echo ""
    echo "Exemples:"
    echo "  $0 vps1  # Déployer sur VPS1 (nœud principal)"
    echo "  $0 vps2  # Déployer sur VPS2"
    echo "  $0 vps3  # Déployer sur VPS3"
    exit 1
}

check_dependencies() {
    echo -e "${YELLOW}Vérification des dépendances...${NC}"

    # Rust
    if ! command -v cargo &> /dev/null; then
        echo -e "${RED}Rust n'est pas installé. Installation...${NC}"
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source $HOME/.cargo/env
    else
        echo -e "${GREEN}✓ Rust installé${NC}"
    fi

    # Git
    if ! command -v git &> /dev/null; then
        echo -e "${RED}Git n'est pas installé. Installation...${NC}"
        sudo apt update && sudo apt install -y git
    else
        echo -e "${GREEN}✓ Git installé${NC}"
    fi

    # Build tools
    if ! dpkg -l | grep -q build-essential; then
        echo -e "${YELLOW}Installation des outils de compilation...${NC}"
        sudo apt install -y build-essential pkg-config libssl-dev clang
    else
        echo -e "${GREEN}✓ Outils de compilation installés${NC}"
    fi
}

clone_or_update() {
    if [ -d "$INSTALL_DIR" ]; then
        echo -e "${YELLOW}Mise à jour du code existant...${NC}"
        cd "$INSTALL_DIR"
        git fetch origin
        git checkout "$BRANCH"
        git pull origin "$BRANCH"
    else
        echo -e "${YELLOW}Clonage du repository...${NC}"
        git clone "$GITHUB_REPO" "$INSTALL_DIR"
        cd "$INSTALL_DIR"
        git checkout "$BRANCH"
    fi
    echo -e "${GREEN}✓ Code à jour${NC}"
}

compile() {
    echo -e "${YELLOW}Compilation (cela peut prendre 5-10 minutes)...${NC}"
    cd "$INSTALL_DIR"
    cargo build --release
    echo -e "${GREEN}✓ Compilation réussie${NC}"
}

get_wallet_for_vps() {
    local vps_num=$1
    case $vps_num in
        1) echo "$VPS1_WALLET" ;;
        2) echo "$VPS2_WALLET" ;;
        3) echo "$VPS3_WALLET" ;;
        *) echo "" ;;
    esac
}

get_peers_for_vps() {
    local vps_num=$1
    case $vps_num in
        1) echo "" ;;  # VPS1 est le bootstrap node
        2) echo "${VPS1_IP}:3001" ;;
        3) echo "${VPS1_IP}:3001,${VPS2_IP}:3001" ;;
        *) echo "" ;;
    esac
}

deploy_vps() {
    local vps_num=$1
    local wallet=$(get_wallet_for_vps $vps_num)
    local peers=$(get_peers_for_vps $vps_num)

    if [ -z "$wallet" ]; then
        echo -e "${RED}Erreur: Wallet pour VPS${vps_num} non configuré!${NC}"
        echo "Éditez ce script et remplissez VPS${vps_num}_WALLET"
        exit 1
    fi

    echo ""
    echo -e "${GREEN}╔═══════════════════════════════════════════════╗${NC}"
    echo -e "${GREEN}║   Déploiement VPS${vps_num}                            ║${NC}"
    echo -e "${GREEN}╠═══════════════════════════════════════════════╣${NC}"
    echo -e "${GREEN}║   Wallet: ${wallet:0:20}...     ║${NC}"
    if [ -n "$peers" ]; then
        echo -e "${GREEN}║   Peers: ${peers:0:30}...  ║${NC}"
    fi
    echo -e "${GREEN}║   Storage: RocksDB (Production)               ║${NC}"
    echo -e "${GREEN}╚═══════════════════════════════════════════════╝${NC}"
    echo ""

    # Créer le répertoire pour RocksDB
    echo -e "${YELLOW}Création du répertoire de données RocksDB...${NC}"
    mkdir -p "$ROCKSDB_PATH"
    echo -e "${GREEN}✓ Répertoire créé: $ROCKSDB_PATH${NC}"

    # Créer le fichier de service systemd
    create_systemd_service $vps_num "$wallet" "$peers"

    # Démarrer le service
    echo -e "${YELLOW}Démarrage du service...${NC}"
    sudo systemctl daemon-reload
    sudo systemctl enable auriumchain
    sudo systemctl restart auriumchain

    echo -e "${GREEN}✓ Service démarré${NC}"
    echo ""
    echo -e "${YELLOW}Vérification du statut:${NC}"
    sudo systemctl status auriumchain --no-pager

    echo ""
    echo -e "${GREEN}╔═══════════════════════════════════════════════╗${NC}"
    echo -e "${GREEN}║   VPS${vps_num} Déployé avec Succès! ✅              ║${NC}"
    echo -e "${GREEN}╚═══════════════════════════════════════════════╝${NC}"
    echo ""
    echo "Commandes utiles:"
    echo "  sudo systemctl status auriumchain    # Voir le statut"
    echo "  sudo journalctl -u auriumchain -f    # Voir les logs en temps réel"
    echo "  sudo systemctl restart auriumchain   # Redémarrer"
    echo "  sudo systemctl stop auriumchain      # Arrêter"
}

create_systemd_service() {
    local vps_num=$1
    local wallet=$2
    local peers=$3

    echo -e "${YELLOW}Création du service systemd avec RocksDB...${NC}"

    local peers_arg=""
    if [ -n "$peers" ]; then
        peers_arg="--peers $peers"
    fi

    sudo tee /etc/systemd/system/auriumchain.service > /dev/null <<EOF
[Unit]
Description=AuriumChain Node VPS${vps_num} (Production RocksDB)
After=network.target

[Service]
Type=simple
User=$USER
WorkingDirectory=$INSTALL_DIR
ExecStart=$INSTALL_DIR/target/release/auriumchain --mining --miner-address $wallet --rocksdb-path $ROCKSDB_PATH $peers_arg
Restart=always
RestartSec=10
StandardOutput=journal
StandardError=journal
SyslogIdentifier=auriumchain

# Security
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=read-only
ReadWritePaths=$DATA_DIR

[Install]
WantedBy=multi-user.target
EOF

    echo -e "${GREEN}✓ Service systemd créé avec RocksDB${NC}"
}

main() {
    if [ $# -eq 0 ]; then
        usage
    fi

    VPS_TYPE=$1

    case $VPS_TYPE in
        vps1)
            VPS_NUM=1
            ;;
        vps2)
            VPS_NUM=2
            ;;
        vps3)
            VPS_NUM=3
            ;;
        *)
            echo -e "${RED}Erreur: Type de VPS invalide${NC}"
            usage
            ;;
    esac

    echo -e "${GREEN}╔═══════════════════════════════════════════════╗${NC}"
    echo -e "${GREEN}║   AuriumChain VPS Deployment Script          ║${NC}"
    echo -e "${GREEN}║   Target: VPS${VPS_NUM}                                ║${NC}"
    echo -e "${GREEN}╚═══════════════════════════════════════════════╝${NC}"
    echo ""

    check_dependencies
    clone_or_update
    compile
    deploy_vps $VPS_NUM

    echo ""
    echo -e "${GREEN}════════════════════════════════════════════════${NC}"
    echo -e "${GREEN}  Déploiement terminé! 🚀${NC}"
    echo -e "${GREEN}════════════════════════════════════════════════${NC}"
}

main "$@"
