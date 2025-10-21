# Migration vers RocksDB - AuriumChain

## 🎯 Objectif

Migrer le stockage de la blockchain depuis JSON (développement) vers RocksDB (production) pour améliorer les performances et la scalabilité.

## 📊 Avantages de RocksDB

- ⚡ **Performance** : Lectures/écritures optimisées avec compression LZ4
- 💾 **Scalabilité** : Gère des millions de blocs sans ralentissement
- 🔒 **Fiabilité** : Opérations atomiques avec transactions batch
- 📦 **Compression** : Réduction de 50-70% de l'espace disque
- 🚀 **Production-ready** : Utilisé par Facebook, LinkedIn, etc.

## 🔧 Utilisation

### Mode JSON (Développement)
```bash
# Comportement par défaut
./auriumchain --mining --port 3001 --rpc-port 8001
```

### Mode RocksDB (Production)
```bash
# Utiliser RocksDB
./auriumchain --mining --port 3001 --rpc-port 8001 --rocksdb-path /var/lib/auriumchain/db
```

## 📦 Migration des Données Existantes

### Étape 1 : Backup de vos données JSON
```bash
cp /tmp/auriumchain.json /tmp/auriumchain.json.backup
```

### Étape 2 : Exécuter l'outil de migration
```bash
./auriumchain-migrate \
  --json-file /tmp/auriumchain.json \
  --rocksdb-path /var/lib/auriumchain/db \
  --verify
```

### Étape 3 : Vérifier la migration
L'outil affiche :
- ✓ Nombre de blocs migrés
- ✓ Total de AUR dans la blockchain
- ✓ Taille de la base de données
- ✓ Vérification de l'intégrité

### Étape 4 : Démarrer le nœud avec RocksDB
```bash
./auriumchain --mining --rocksdb-path /var/lib/auriumchain/db
```

## 🏗️ Structure RocksDB

### Tables (Column Families)
- `block:{index}` → Bloc sérialisé (bincode)
- `hash:{hash}` → Index du bloc
- `latest_index` → Dernier index de bloc
- `utxo:{tx_id}:{output_index}` → UTXO set

### Exemples de clés
```
block:0          → Genesis block
block:750        → Block #750
hash:a7f3c9...   → Index du bloc avec ce hash
latest_index     → 750
utxo:tx123:0     → UTXO non dépensé
```

## 📈 Performance

### Benchmarks (estimés)

| Opération | JSON | RocksDB | Gain |
|-----------|------|---------|------|
| Lecture 1 bloc | 50ms | 0.5ms | **100x** |
| Écriture 1 bloc | 100ms | 1ms | **100x** |
| Recherche par hash | O(n) | O(1) | **n×** |
| Taille sur disque | 100MB | 30MB | **-70%** |

## 🔄 Rétrogradation (Rollback)

Si vous voulez revenir au JSON :

```bash
# Exporter RocksDB vers JSON (à implémenter si nécessaire)
# Pour l'instant, gardez votre backup JSON

# Redémarrer sans --rocksdb-path
./auriumchain --mining --data-file /tmp/auriumchain.json
```

## 🚨 Important pour le Déploiement VPS

Lors du déploiement sur VPS 1, 2, 3 :

### VPS 1 (déjà en production avec 750+ AUR)
```bash
# 1. Arrêter le nœud
systemctl stop auriumchain

# 2. Migrer les données
./auriumchain-migrate \
  --json-file /tmp/auriumchain.json \
  --rocksdb-path /var/lib/auriumchain/db \
  --verify

# 3. Mettre à jour le service systemd
# Modifier: --rocksdb-path /var/lib/auriumchain/db

# 4. Redémarrer
systemctl start auriumchain
```

### VPS 2 & 3 (nouveaux nœuds)
```bash
# Démarrer directement avec RocksDB
./auriumchain --mining \
  --port 3001 \
  --rpc-port 8001 \
  --rocksdb-path /var/lib/auriumchain/db \
  --peer <IP_VPS1>:3001
```

## 🔍 Monitoring

### Vérifier l'état de la DB
```bash
# Via l'API RPC
curl http://localhost:8001/status

# Logs du nœud
tail -f /var/log/auriumchain/node.log
```

### Compacter la DB (si nécessaire)
```rust
// Via code
db.compact()?;
```

## 📝 Notes Techniques

- **Compression** : LZ4 activée par défaut
- **Parallélisme** : Auto-détecté (nombre de CPU cores)
- **Cache** : Géré automatiquement par RocksDB
- **Durabilité** : Toutes les écritures sont synchrones

## 🐛 Troubleshooting

### Erreur : "Cannot open database"
```bash
# Vérifier les permissions
sudo chown -R auriumchain:auriumchain /var/lib/auriumchain/db

# Vérifier l'espace disque
df -h
```

### Performance dégradée
```bash
# Compacter manuellement
# (fonction à ajouter dans RPC si nécessaire)
```

## 📚 Ressources

- [RocksDB Documentation](https://rocksdb.org/)
- [RocksDB Tuning Guide](https://github.com/facebook/rocksdb/wiki/RocksDB-Tuning-Guide)
