# 🌐 AuriumChain RPC API Reference

## 📋 Routes Disponibles

Le serveur RPC écoute par défaut sur **http://localhost:8001**

---

## 📊 GET /status

Obtenir le statut du nœud

**Exemple** :
```bash
# Linux/Mac
curl http://localhost:8001/status

# Windows PowerShell
(Invoke-WebRequest http://localhost:8001/status).Content
```

**Réponse** :
```json
{
  "status": "running",
  "version": "1.0.0",
  "block_height": 42,
  "latest_hash": "000abc123...",
  "difficulty": 4,
  "is_valid": true,
  "pending_transactions": 0
}
```

---

## 🔗 GET /blocks

Obtenir tous les blocs de la blockchain

**Exemple** :
```bash
# Linux/Mac
curl http://localhost:8001/blocks

# Windows PowerShell
(Invoke-WebRequest http://localhost:8001/blocks).Content | ConvertFrom-Json
```

**Réponse** :
```json
[
  {
    "index": 0,
    "timestamp": 1234567890,
    "transactions": [],
    "previous_hash": "0",
    "hash": "000genesis...",
    "nonce": 0
  },
  {
    "index": 1,
    "timestamp": 1234567920,
    "transactions": [],
    "previous_hash": "000genesis...",
    "hash": "000block1...",
    "nonce": 12345
  }
]
```

**⚠️ Attention** : Peut être très volumineux sur une longue chaîne !

---

## 📈 GET /chain_info

Obtenir les informations de la chaîne

**Exemple** :
```bash
# Linux/Mac
curl http://localhost:8001/chain_info

# Windows PowerShell
(Invoke-WebRequest http://localhost:8001/chain_info).Content
```

**Réponse** :
```json
{
  "height": 42,
  "latest_hash": "000abc123...",
  "difficulty": 4
}
```

---

## 💰 GET /balance/{address}

Obtenir la balance d'une adresse

**Exemple** :
```bash
# Linux/Mac
curl http://localhost:8001/balance/AUR3ZnxihprBGetUiMoHwRWZbcyU94TzP52Jkk

# Windows PowerShell
(Invoke-WebRequest http://localhost:8001/balance/AUR3ZnxihprBGetUiMoHwRWZbcyU94TzP52Jkk).Content
```

**Réponse** :
```json
{
  "address": "AUR3ZnxihprBGetUiMoHwRWZbcyU94TzP52Jkk",
  "balance": 2500,
  "currency": "AUR"
}
```

---

## 🔢 GET /blocks_from/{height}

Obtenir les blocs à partir d'une certaine hauteur

**Exemple** :
```bash
# Blocs à partir du bloc 10
curl http://localhost:8001/blocks_from/10

# Windows PowerShell
(Invoke-WebRequest http://localhost:8001/blocks_from/10).Content | ConvertFrom-Json
```

**Réponse** :
```json
[
  {
    "index": 10,
    "timestamp": 1234568000,
    "transactions": [],
    "previous_hash": "000block9...",
    "hash": "000block10...",
    "nonce": 23456
  },
  {
    "index": 11,
    ...
  }
]
```

**Usage** : Utile pour la synchronisation et récupérer uniquement les nouveaux blocs

---

## ➕ POST /new_block

Soumettre un nouveau bloc à la blockchain

**Exemple** :
```bash
# Linux/Mac
curl -X POST http://localhost:8001/new_block \
  -H "Content-Type: application/json" \
  -d '{
    "index": 43,
    "timestamp": 1234567890,
    "transactions": [],
    "previous_hash": "000abc...",
    "hash": "000def...",
    "nonce": 12345
  }'

# Windows PowerShell
$body = @{
  index = 43
  timestamp = 1234567890
  transactions = @()
  previous_hash = "000abc..."
  hash = "000def..."
  nonce = 12345
} | ConvertTo-Json

Invoke-WebRequest -Uri http://localhost:8001/new_block -Method POST -Body $body -ContentType "application/json"
```

**Réponse (succès)** :
```json
{
  "status": "block_accepted"
}
```

**Réponse (erreur)** :
```json
{
  "error": "invalid_block"
}
```

---

## 🚨 Routes NON Disponibles

Ces routes **n'existent PAS** dans AuriumChain (contrairement à d'autres blockchains) :

- ❌ `/chain/length` - Utilisez `/chain_info` à la place
- ❌ `/blocks/latest` - Utilisez `/status` pour obtenir le dernier hash
- ❌ `/blocks/{index}` - Utilisez `/blocks` et filtrez localement
- ❌ `/peers` - Pas d'endpoint pour lister les pairs (à implémenter)
- ❌ `/mine` - Le mining se fait via le flag `--mining` au démarrage
- ❌ `/transactions` - Pas d'endpoint transactions (à implémenter)

---

## 💡 Exemples Pratiques

### Vérifier la Hauteur de la Chaîne

```bash
# Linux/Mac
curl -s http://localhost:8001/chain_info | jq '.height'

# Windows PowerShell
(Invoke-WebRequest http://localhost:8001/chain_info).Content | ConvertFrom-Json | Select-Object -ExpandProperty height
```

---

### Obtenir le Dernier Hash

```bash
# Linux/Mac
curl -s http://localhost:8001/status | jq '.latest_hash'

# Windows PowerShell
(Invoke-WebRequest http://localhost:8001/status).Content | ConvertFrom-Json | Select-Object -ExpandProperty latest_hash
```

---

### Compter les Blocs

```bash
# Linux/Mac
curl -s http://localhost:8001/blocks | jq '. | length'

# Windows PowerShell
((Invoke-WebRequest http://localhost:8001/blocks).Content | ConvertFrom-Json).Count
```

---

### Vérifier Votre Balance

```bash
# Remplacez par votre adresse
curl http://localhost:8001/balance/VOTRE_ADRESSE_AUR
```

---

### Synchroniser Depuis un Autre Nœud

```bash
# Obtenir la hauteur locale
LOCAL_HEIGHT=$(curl -s http://localhost:8001/chain_info | jq '.height')

# Récupérer les nouveaux blocs depuis un pair
curl http://PEER_IP:8001/blocks_from/$LOCAL_HEIGHT
```

---

## 🔧 Configuration du Serveur

### Démarrer avec un Port Personnalisé

```bash
./target/release/auriumchain --rpc-port 9000
```

### Bind sur Toutes les Interfaces (Pour Accès Externe)

```bash
./target/release/auriumchain --host 0.0.0.0 --rpc-port 8001
```

**⚠️ Attention** : Exposer le RPC publiquement peut être dangereux. Utilisez un firewall !

---

## 🐛 Codes d'Erreur

| Code | Message | Signification |
|------|---------|---------------|
| 200 | `{"error":"Not found"}` | Route invalide |
| 200 | `{"error":"Invalid balance request"}` | Adresse manquante dans /balance/ |
| 200 | `{"error":"Serialization failed"}` | Erreur interne de sérialisation |
| 200 | `{"error":"invalid_block"}` | Bloc soumis invalide |
| 200 | `{"error":"invalid_json"}` | JSON malformé dans POST |

**Note** : Actuellement, toutes les réponses retournent 200 OK, même en cas d'erreur.

---

## 📊 Monitoring Script

### Linux/Mac

Créez `monitor.sh` :
```bash
#!/bin/bash
while true; do
  clear
  echo "=== AuriumChain Status ==="
  curl -s http://localhost:8001/status | jq '.'
  echo ""
  echo "=== Chain Info ==="
  curl -s http://localhost:8001/chain_info | jq '.'
  sleep 5
done
```

```bash
chmod +x monitor.sh
./monitor.sh
```

### Windows PowerShell

```powershell
while ($true) {
  Clear-Host
  Write-Host "=== AuriumChain Status ===" -ForegroundColor Green
  (Invoke-WebRequest http://localhost:8001/status).Content | ConvertFrom-Json | Format-List

  Write-Host "`n=== Chain Info ===" -ForegroundColor Green
  (Invoke-WebRequest http://localhost:8001/chain_info).Content | ConvertFrom-Json | Format-List

  Start-Sleep -Seconds 5
}
```

---

## 🔒 Sécurité

### Recommandations

1. **Ne pas exposer publiquement** : Par défaut, bind sur 127.0.0.1
2. **Firewall** : Si vous devez exposer, utilisez un firewall strict
3. **Rate limiting** : Pas implémenté - à ajouter pour production
4. **Authentication** : Pas implémenté - considérez d'ajouter des tokens
5. **HTTPS** : Actuellement HTTP seulement - considérez un reverse proxy

---

## 🚀 Améliorations Futures

Routes prévues pour les futures versions :

- `GET /peers` - Lister les pairs connectés
- `GET /blocks/{index}` - Obtenir un bloc spécifique
- `GET /transactions` - Lister les transactions
- `POST /transactions` - Soumettre une transaction
- `GET /mempool` - Voir les transactions en attente
- `GET /mining/stats` - Statistiques de mining
- `WebSocket /subscribe` - Notifications en temps réel

---

## 📞 Support

Des questions sur l'API ?

- **Issues** : https://github.com/Hicham60290/Auriumchain/issues
- **Discussions** : https://github.com/Hicham60290/Auriumchain/discussions
- **Documentation** : Voir README.md

---

**Version de l'API** : 1.0.0
**Dernière mise à jour** : 2025-01-01
