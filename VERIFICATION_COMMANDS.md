# 🔍 Commandes de Vérification AuriumChain

## 🚀 Vérification Rapide

### 1. Vérifier la Longueur de la Chaîne

```bash
curl http://localhost:8001/chain/length
```

**Résultat attendu** :
```json
{"length": 42}
```

---

### 2. Voir le Dernier Bloc

```bash
curl http://localhost:8001/blocks/latest
```

**Résultat attendu** :
```json
{
  "index": 41,
  "timestamp": 1234567890,
  "transactions": [],
  "previous_hash": "000abc...",
  "hash": "000def...",
  "nonce": 12345
}
```

---

### 3. Vérifier les Pairs Connectés

```bash
curl http://localhost:8001/peers
```

**Résultat attendu** :
```json
{
  "peers": [
    "192.168.1.100:3001",
    "10.0.0.5:3001"
  ]
}
```

---

## 📊 Vérification Complète

### Script Automatique

J'ai créé un script de vérification complet :

```bash
# Vérifier avec le fichier par défaut
./verify_blockchain.sh

# Vérifier avec un fichier spécifique
./verify_blockchain.sh /chemin/vers/blockchain.json

# Vérifier avec un port RPC personnalisé
./verify_blockchain.sh /tmp/auriumchain.json 8002
```

**Ce script vérifie** :
- ✅ Existence du fichier
- ✅ État du nœud RPC
- ✅ Nombre de blocs
- ✅ Premier bloc (Genesis)
- ✅ Dernier bloc miné
- ✅ Pairs connectés

---

## 🔧 Vérifications Avancées

### Vérifier l'Intégrité de la Chaîne

```bash
# Via les tests Rust
cargo test is_chain_valid

# Résultat attendu:
# test blockchain::tests::test_is_chain_valid ... ok
```

---

### Vérifier Tous les Blocs

```bash
curl http://localhost:8001/chain | jq '.'
```

**Note** : Peut être très long si beaucoup de blocs !

---

### Vérifier un Bloc Spécifique

```bash
# Bloc numéro 10
curl http://localhost:8001/blocks/10

# Bloc Genesis (toujours index 0)
curl http://localhost:8001/blocks/0
```

---

### Vérifier le Fichier de Données

```bash
# Avec jq (formatage JSON)
cat /tmp/auriumchain.json | jq '.'

# Nombre de blocs
cat /tmp/auriumchain.json | jq '.chain | length'

# Liste des hash des 10 derniers blocs
cat /tmp/auriumchain.json | jq '.chain[-10:] | .[] | .hash'

# Vérifier que tous les blocs ont un hash
cat /tmp/auriumchain.json | jq '.chain[] | select(.hash == null or .hash == "")'
```

---

## 🧪 Tests de Sécurité

### Lancer Tous les Tests de Sécurité

```bash
cargo test --test test_security_attacks
```

**Devrait afficher** :
```
running 10 tests
test test_block_hash_validation ... ok
test test_genesis_block_protection ... ok
test test_invalid_previous_hash ... ok
test test_proof_of_work_requirement ... ok
test test_sequential_index ... ok
test test_timestamp_manipulation ... ok
test test_double_spend_prevention ... ok
test test_invalid_block_reward ... ok
test test_chain_reorganization ... ok
test test_deterministic_genesis ... ok

test result: ok. 10 passed; 0 failed
```

---

## 🐛 Débogage

### Vérifier si le Nœud Est en Cours d'Exécution

```bash
# Vérifier le processus
ps aux | grep auriumchain

# Vérifier le port P2P
netstat -tuln | grep 3001

# Vérifier le port RPC
netstat -tuln | grep 8001

# Ou avec lsof
lsof -i :3001
lsof -i :8001
```

---

### Logs Détaillés

```bash
# Lancer avec logs de debug
RUST_LOG=debug ./target/release/auriumchain --mining

# Logs uniquement pour la blockchain
RUST_LOG=auriumchain::blockchain=trace ./target/release/auriumchain --mining

# Logs pour le P2P
RUST_LOG=auriumchain::p2p=debug ./target/release/auriumchain --mining
```

---

### Vérifier la Synchronisation

```bash
# Comparer la longueur de chaîne avec un pair
curl http://localhost:8001/chain/length
curl http://PEER_IP:8001/chain/length

# Comparer les hash des derniers blocs
curl http://localhost:8001/blocks/latest | jq '.hash'
curl http://PEER_IP:8001/blocks/latest | jq '.hash'
```

---

## 📈 Monitoring en Temps Réel

### Watch la Longueur de la Chaîne

```bash
# Met à jour toutes les 2 secondes
watch -n 2 'curl -s http://localhost:8001/chain/length'
```

### Watch le Dernier Bloc

```bash
watch -n 2 'curl -s http://localhost:8001/blocks/latest | jq "{index, hash, timestamp}"'
```

### Watch les Pairs

```bash
watch -n 5 'curl -s http://localhost:8001/peers | jq "."'
```

---

## 🔍 Vérifications Critiques Avant Production

### Checklist de Vérification

```bash
# 1. Vérifier que le Genesis block existe
curl http://localhost:8001/blocks/0 | jq '.index'
# Devrait retourner: 0

# 2. Vérifier que la chaîne est valide
cargo test is_chain_valid
# Devrait passer: ok

# 3. Vérifier qu'on peut ajouter des blocs
# (observer les logs de mining)

# 4. Vérifier la connexion P2P
curl http://localhost:8001/peers
# Devrait montrer au moins 1 pair

# 5. Vérifier que le fichier se sauvegarde
ls -lh /tmp/auriumchain.json
# Devrait exister et grandir

# 6. Vérifier les permissions
ls -la /tmp/auriumchain.json
# Devrait être lisible/modifiable
```

---

## 🚨 Problèmes Fréquents

### "curl: (7) Failed to connect"

**Cause** : Le nœud RPC n'est pas lancé ou sur un port différent

**Solution** :
```bash
# Vérifier quel port écoute
netstat -tuln | grep LISTEN

# Ou lancer le nœud si pas démarré
./target/release/auriumchain --mining
```

---

### "No such file or directory"

**Cause** : Le fichier blockchain n'existe pas encore

**Solution** :
```bash
# Créer un Genesis block
./target/release/auriumchain --genesis --mining
```

---

### Chaîne ne grandit pas

**Cause** : Mining pas activé ou problème de difficulté

**Solution** :
```bash
# Relancer avec mining
./target/release/auriumchain --mining

# Vérifier les logs
RUST_LOG=debug ./target/release/auriumchain --mining
```

---

## 📚 Ressources

- **README.md** : Guide complet du projet
- **TESTING.md** : Documentation des tests
- **NETWORK_CONNECTION_GUIDE.md** : Guide de connexion réseau
- **MINING_QUICKSTART.md** : Guide rapide de mining

---

## 💡 Astuce Pro

Créez un alias pour vérifier rapidement :

```bash
# Ajoutez dans ~/.bashrc ou ~/.zshrc
alias aur-status='curl -s http://localhost:8001/chain/length && curl -s http://localhost:8001/blocks/latest | jq "{index, hash}"'

# Puis utilisez simplement :
aur-status
```

---

**Besoin d'aide ?** Consultez les [Issues GitHub](https://github.com/Hicham60290/Auriumchain/issues) ou créez une [Discussion](https://github.com/Hicham60290/Auriumchain/discussions).
