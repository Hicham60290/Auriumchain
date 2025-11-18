#!/bin/bash
# verify_blockchain.sh - Script de vérification de la blockchain AuriumChain

echo "🔍 AuriumChain Blockchain Verification"
echo "======================================"
echo ""

# Vérifier si le fichier existe
BLOCKCHAIN_FILE="${1:-/tmp/auriumchain.json}"

if [ ! -f "$BLOCKCHAIN_FILE" ]; then
    echo "❌ Fichier blockchain non trouvé: $BLOCKCHAIN_FILE"
    exit 1
fi

echo "📁 Fichier: $BLOCKCHAIN_FILE"
echo ""

# Vérifier si le nœud RPC est actif
RPC_PORT="${2:-8001}"
if curl -s http://localhost:$RPC_PORT/chain/length > /dev/null 2>&1; then
    echo "✅ Nœud RPC actif sur le port $RPC_PORT"
    echo ""

    # Informations via RPC
    echo "📊 Informations de la Chaîne (via RPC):"
    echo "----------------------------------------"

    LENGTH=$(curl -s http://localhost:$RPC_PORT/chain/length)
    echo "Nombre de blocs: $LENGTH"

    echo ""
    echo "🔗 Dernier bloc:"
    curl -s http://localhost:$RPC_PORT/blocks/latest | jq '.'

    echo ""
    echo "👥 Pairs connectés:"
    curl -s http://localhost:$RPC_PORT/peers | jq '.'

else
    echo "⚠️  Nœud RPC non actif sur le port $RPC_PORT"
    echo "   Vérification via fichier uniquement..."
    echo ""
fi

# Vérification via fichier
echo "📂 Informations du Fichier:"
echo "----------------------------------------"

if command -v jq > /dev/null 2>&1; then
    # Avec jq
    BLOCK_COUNT=$(cat "$BLOCKCHAIN_FILE" | jq '.chain | length')
    echo "Nombre de blocs: $BLOCK_COUNT"

    echo ""
    echo "🔗 Premier bloc (Genesis):"
    cat "$BLOCKCHAIN_FILE" | jq '.chain[0]'

    echo ""
    echo "🔗 Dernier bloc:"
    cat "$BLOCKCHAIN_FILE" | jq '.chain[-1]'

    echo ""
    echo "📈 Liste des blocs:"
    cat "$BLOCKCHAIN_FILE" | jq '.chain[] | {index, hash: .hash[0:16], timestamp, nonce}'

else
    # Sans jq
    echo "⚠️  Installer 'jq' pour une meilleure visualisation: sudo apt install jq"
    echo ""
    echo "Contenu brut (premiers 500 caractères):"
    head -c 500 "$BLOCKCHAIN_FILE"
    echo ""
    echo "..."
fi

echo ""
echo "✅ Vérification terminée"
