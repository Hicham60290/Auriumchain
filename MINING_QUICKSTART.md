# ⛏️ Guide Rapide de Mining - AuriumChain

## 🚀 Démarrage en 3 Étapes

### Étape 1 : Installer

```bash
git clone https://github.com/Hicham60290/Auriumchain.git
cd Auriumchain
cargo build --release
```

### Étape 2 : Se Connecter au Réseau

**⚠️ IMPORTANT** : Vous devez vous connecter à un nœud existant !

```bash
./target/release/auriumchain \
  --mining \
  --wallet VOTRE_ADRESSE_AUR \
  --peer ADRESSE_IP_BOOTSTRAP:3001
```

### Étape 3 : Miner !

Le mining démarre automatiquement. Vous verrez :
```
Block 123 mined and saved (TLS)!
   Hash: 000abc...
   Chain: 124 blocks
```

---

## 🌐 Nœud de Bootstrap Officiel

**Pour rejoindre le réseau, connectez-vous au nœud principal :**

```
IP: [À METTRE À JOUR]
Port: 3001
```

**Commande complète** :
```bash
./target/release/auriumchain \
  --mining \
  --wallet VOTRE_ADRESSE \
  --peer [IP_BOOTSTRAP]:3001
```

---

## 💰 Récompenses de Mining

- **Récompense par bloc** : 50 AUR
- **Temps de bloc** : ~30 secondes
- **Difficulté** : Auto-ajustée
- **Halving** : Tous les 4,204,800 blocs

---

## 🔧 Options Avancées

### Mining avec Configuration Personnalisée

```bash
./target/release/auriumchain \
  --mining \
  --wallet VOTRE_ADRESSE \
  --peer IP:3001 \
  --port 3002 \
  --rpc-port 8002 \
  --data-file ~/auriumchain.json
```

### Vérifier Votre Statut

```bash
# Longueur de la chaîne
curl http://localhost:8001/chain/length

# Dernier bloc
curl http://localhost:8001/blocks/latest

# Pairs connectés
curl http://localhost:8001/peers
```

---

## ❓ FAQ Mining

### Q: Je n'ai pas d'adresse AUR, comment en obtenir une ?
**R:** Utilisez l'adresse par défaut pour commencer, ou générez-en une nouvelle avec le wallet tool (à venir).

### Q: Comment savoir si je mine vraiment ?
**R:** Vous verrez des messages "Block X mined" dans la console toutes les ~30 secondes.

### Q: Puis-je miner avec un GPU/ASIC ?
**R:** Actuellement, seul le CPU mining est implémenté. Le support ASIC est prévu pour v1.2.

### Q: Dois-je rester connecté 24/7 ?
**R:** Non, mais plus vous minez, plus vous avez de chances de recevoir des récompenses.

### Q: Que faire si "Connection refused" ?
**R:** Vérifiez que l'IP du bootstrap est correcte et que le nœud est en ligne. Voir [NETWORK_CONNECTION_GUIDE.md](NETWORK_CONNECTION_GUIDE.md).

---

## 🎯 Prochaines Étapes

1. **Rejoignez la communauté** : https://github.com/Hicham60290/Auriumchain/discussions
2. **Signalez des bugs** : https://github.com/Hicham60290/Auriumchain/issues
3. **Contribuez** : Pull requests welcome !

---

**Bon mining ! ⛏️🚀**
