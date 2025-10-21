# Gestion des Wallets - AuriumChain

## 🎯 Importance Critique

**AVANT DE DÉPLOYER** sur les VPS, vous DEVEZ :
1. ✅ Générer ou identifier votre wallet principal
2. ✅ Sauvegarder la clé privée en sécurité
3. ✅ Configurer l'adresse du mineur sur chaque nœud

---

## 🔑 Génération d'un Nouveau Wallet

### Méthode 1 : Outil de génération rapide (Recommandé)

```bash
# Générer un nouveau wallet AUR3 (hybride quantum-resistant)
./target/release/auriumchain-keygen

# Générer plusieurs wallets
./target/release/auriumchain-keygen -n 3

# Générer un wallet AUR1 (legacy)
./target/release/auriumchain-keygen -t AUR1
```

**Sortie exemple :**
```
AuriumChain Wallet Generator
============================

Address:      AUR3XyZ...abc123
Public Key:   04a7b3c9...
Private Key:  e8f2d1...

⚠️  IMPORTANT SECURITY WARNINGS:
   1. NEVER share your private key with anyone!
   2. Store your private key in a secure location
   3. Backup your private key - if lost, funds are unrecoverable
   4. Consider using a hardware wallet for large amounts

📋 Usage:
   To use this address for mining:
   ./auriumchain --mining --miner-address AUR3XyZ...abc123
```

### Méthode 2 : Wallet sécurisé avec BIP39

```bash
# Générer un wallet avec phrase mnémonique (24 mots)
./target/release/auriumchain-wallet create --name my_wallet

# Restaurer depuis une phrase mnémonique
./target/release/auriumchain-wallet restore --name my_wallet
```

---

## 📋 Configuration du Mineur

### Par défaut (NON RECOMMANDÉ pour production)
```bash
./auriumchain --mining
# ⚠️ Utilise l'adresse hardcodée: AUR3ZnxihprBGetUiMoHwRWZbcyU94TzP52Jkk
```

### Avec votre propre adresse (RECOMMANDÉ)
```bash
./auriumchain --mining --miner-address AUR3VotreAdresseIci123
```

### Exemple complet
```bash
./auriumchain \
  --mining \
  --port 3001 \
  --rpc-port 8001 \
  --rocksdb-path /var/lib/auriumchain/db \
  --miner-address AUR3VotreAdresseIci123
```

---

## 🏗️ Stratégie de Déploiement Multi-VPS

### Option 1 : Un wallet pour tous les nœuds (Simple)

**Avantage** : Tous les AUR dans une seule adresse, facile à gérer
**Inconvénient** : Point de défaillance unique

```bash
# Même adresse sur tous les VPS
VPS1: --miner-address AUR3MainWallet123
VPS2: --miner-address AUR3MainWallet123
VPS3: --miner-address AUR3MainWallet123
```

### Option 2 : Un wallet par VPS (Décentralisé)

**Avantage** : Sécurité maximale, distribution des risques
**Inconvénient** : Gestion de 3 wallets

```bash
VPS1: --miner-address AUR3Wallet1Address
VPS2: --miner-address AUR3Wallet2Address
VPS3: --miner-address AUR3Wallet3Address
```

### Option 3 : Wallet maître + wallets nœuds (Pro)

**Avantage** : Sécurité + traçabilité
**Utilisation** : Wallet froid (maître) + wallets chauds (nœuds)

```bash
# Wallet froid (stockage sécurisé, jamais en ligne)
Master: AUR3ColdStorageSecure999

# Wallets chauds (nœuds actifs)
VPS1: AUR3HotWallet1Node
VPS2: AUR3HotWallet2Node
VPS3: AUR3HotWallet3Node

# Transférer régulièrement de Hot → Cold
```

---

## 🔒 Sécurité des Clés Privées

### ⛔ NE JAMAIS FAIRE :
- ❌ Envoyer la clé privée par email/Slack/Discord
- ❌ Stocker en clair dans un fichier texte
- ❌ Commiter la clé privée sur GitHub
- ❌ Prendre une photo de la clé
- ❌ Copier-coller dans un document partagé

### ✅ BONNES PRATIQUES :

#### 1. Stockage local sécurisé
```bash
# Créer un fichier chiffré
gpg -c wallet_private_key.txt

# Déchiffrer quand nécessaire
gpg wallet_private_key.txt.gpg
```

#### 2. Gestionnaire de mots de passe
- 1Password
- Bitwarden
- KeePassXC

#### 3. Wallet matériel (Hardware Wallet)
Pour grandes quantités :
- Ledger Nano
- Trezor
- (À implémenter : support hardware wallet AuriumChain)

#### 4. Backup physique
```
1. Écrire la phrase mnémonique (24 mots) sur papier
2. Stocker dans un coffre-fort
3. Faire 2-3 copies dans des lieux différents
```

---

## 💰 Migration des 750+ AUR Existants

### ⚠️ VÉRIFICATION URGENTE

Si vous avez déjà **750+ AUR minés** sur VPS1 avec l'adresse :
```
AUR3ZnxihprBGetUiMoHwRWZbcyU94TzP52Jkk
```

**VÉRIFIEZ IMMÉDIATEMENT :**

```bash
# Sur votre PC local, cherchez la clé privée
find ~ -type f -name "*.wallet" -o -name "*.key" | xargs grep -l "AUR3Znxihpr"

# Ou dans les fichiers de configuration
cat ~/.auriumchain/wallet.json
cat ~/.config/auriumchain/keys
```

### Si vous N'AVEZ PAS la clé privée :

**🚨 URGENT : Les 750 AUR sont inaccessibles !**

Solutions possibles :
1. Chercher dans vos backups
2. Vérifier les logs de création du wallet initial
3. Contacter si vous aviez un service de génération de wallet

### Si vous AVEZ la clé privée :

✅ Vous êtes sécurisé ! Continuez avec cette adresse ou migrez :

```bash
# Option A : Continuer avec cette adresse
./auriumchain --mining --miner-address AUR3ZnxihprBGetUiMoHwRWZbcyU94TzP52Jkk

# Option B : Migrer vers nouvelle adresse
# 1. Générer nouvelle adresse
./auriumchain-keygen

# 2. Créer transaction de transfert (À IMPLÉMENTER)
./auriumchain-wallet transfer \
  --from AUR3ZnxihprBGetUiMoHwRWZbcyU94TzP52Jkk \
  --to AUR3NouvelleAdresse \
  --amount 750000000 \
  --private-key-file old_wallet.key
```

---

## 📊 Vérification du Solde

### Via RPC API
```bash
# Vérifier le solde d'une adresse
curl http://localhost:8001/balance/AUR3VotreAdresse

# Réponse
{
  "address": "AUR3VotreAdresse",
  "balance": 750000000,
  "currency": "AUR"
}
```

### Via Blockchain Explorer (À développer)
```bash
# Liste des blocs minés par une adresse
curl http://localhost:8001/blocks_by_miner/AUR3VotreAdresse
```

---

## 🔄 Types d'Adresses AuriumChain

| Type | Description | Sécurité | Utilisation |
|------|-------------|----------|-------------|
| **AUR1** | Legacy ECDSA (secp256k1) | Standard | Compatible, rapide |
| **AUR2** | Quantum-resistant seul | Haute | Future-proof |
| **AUR3** | Hybride (ECDSA + Quantum) | Maximale | **Recommandé** |

---

## 📝 Checklist Pré-Déploiement

Avant de déployer sur VPS :

- [ ] Wallet principal généré
- [ ] Clé privée sauvegardée (3 copies, lieux différents)
- [ ] Phrase mnémonique notée et sécurisée
- [ ] Adresse wallet testée localement
- [ ] Balance initiale vérifiée (si migration)
- [ ] Script de démarrage configuré avec `--miner-address`
- [ ] Service systemd mis à jour
- [ ] Documentation des adresses par VPS

---

## 🆘 Récupération en Cas de Perte

### Si vous perdez la clé privée :
- ❌ **Fonds irrécupérables** (c'est la nature des crypto-monnaies)
- ✅ Phrase mnémonique peut régénérer la clé

### Si vous perdez la phrase mnémonique :
- ✅ Tant que vous avez la clé privée, vous pouvez accéder aux fonds
- ⚠️ Risque de perte permanente si perte de clé + phrase

---

## 🎯 Action Immédiate Requise

**AVANT DE CONTINUER :**

1. **Générez un wallet maintenant** :
```bash
cd ~/auriumchain-core
./target/release/auriumchain-keygen
```

2. **Sauvegardez la sortie** dans un endroit sûr

3. **Confirmez** que vous avez accès à vos 750+ AUR existants

4. **Planifiez** votre stratégie de wallet (1, 2, ou 3 de ce document)

---

## 📞 Support

En cas de problème avec vos wallets :
- Consultez les logs : `/var/log/auriumchain/`
- Vérifiez la blockchain : RPC API
- **Ne partagez JAMAIS votre clé privée**
