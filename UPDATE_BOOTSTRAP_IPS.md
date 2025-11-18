# 📝 Guide: Mettre à Jour les IP des Nœuds Bootstrap

## 🎯 Objectif

Remplacer les placeholders `[DE_NODE_IP]`, `[CA_NODE_IP]`, `[IT_NODE_IP]` par vos vraies adresses IP publiques des VPS.

---

## 📋 Récupérer les IP de vos VPS

### Sur chaque VPS, exécutez :

```bash
# Méthode 1 : Via curl
curl ifconfig.me

# Méthode 2 : Via API
curl https://api.ipify.org

# Méthode 3 : Via commande système
curl icanhazip.com
```

**Notez les 3 IP publiques :**
- 🇩🇪 VPS Allemagne : `___.___.___. ___`
- 🇨🇦 VPS Canada : `___.___.___. ___`
- 🇮🇹 VPS Milan : `___.___.___. ___`

---

## 📁 Fichiers à Mettre à Jour

### 1. BOOTSTRAP_NODES.md

**Remplacez :**
```markdown
### 🇩🇪 Node 1 - Germany (Primary)
- **IP**: `[À REMPLACER PAR IP VPS1]`

### 🇨🇦 Node 2 - Canada
- **IP**: `[À REMPLACER PAR IP VPS2]`

### 🇮🇹 Node 3 - Milan, Italy
- **IP**: `[À REMPLACER PAR IP VPS3]`
```

**Par :**
```markdown
### 🇩🇪 Node 1 - Germany (Primary)
- **IP**: `123.45.67.89`  # Votre vraie IP VPS Allemagne

### 🇨🇦 Node 2 - Canada
- **IP**: `234.56.78.90`  # Votre vraie IP VPS Canada

### 🇮🇹 Node 3 - Milan, Italy
- **IP**: `345.67.89.01`  # Votre vraie IP VPS Milan
```

**Plus loin dans le même fichier, dans les exemples de code :**

Remplacez :
```bash
--peer [IP_VPS1]:3001
--peer [IP_VPS2]:3001
--peer [IP_VPS3]:3001
```

Par :
```bash
--peer 123.45.67.89:3001  # Allemagne
--peer 234.56.78.90:3001  # Canada
--peer 345.67.89.01:3001  # Milan
```

---

### 2. README.md

**Dans la section "Join the Network":**

Remplacez :
```bash
--peer [DE_NODE_IP]:3001
--peer [CA_NODE_IP]:3001
--peer [IT_NODE_IP]:3001
```

Par :
```bash
--peer 123.45.67.89:3001  # Germany
--peer 234.56.78.90:3001  # Canada
--peer 345.67.89.01:3001  # Milan
```

---

### 3. REDDIT_RUST_POST.md

**Si vous voulez mentionner les nœuds dans le post Reddit**, ajoutez :

```markdown
**Bootstrap Nodes**:
- 🇩🇪 Germany: `123.45.67.89:3001`
- 🇨🇦 Canada: `234.56.78.90:3001`
- 🇮🇹 Milan: `345.67.89.01:3001`
```

---

### 4. REDDIT_MINING_POST.md

Mettez à jour la commande de connexion :

```bash
# Connectez-vous au nœud le plus proche
./target/release/auriumchain \
  --mining \
  --wallet VOTRE_ADRESSE \
  --peer 123.45.67.89:3001  # Germany (ou Canada/Milan)
```

---

## 🔧 Commande de Remplacement Rapide (Linux/Mac)

Si vous voulez automatiser le remplacement :

```bash
# Définissez vos IP
DE_IP="123.45.67.89"
CA_IP="234.56.78.90"
IT_IP="345.67.89.01"

# Remplacer dans BOOTSTRAP_NODES.md
sed -i "s/\[À REMPLACER PAR IP VPS1\]/$DE_IP/g" BOOTSTRAP_NODES.md
sed -i "s/\[À REMPLACER PAR IP VPS2\]/$CA_IP/g" BOOTSTRAP_NODES.md
sed -i "s/\[À REMPLACER PAR IP VPS3\]/$IT_IP/g" BOOTSTRAP_NODES.md

sed -i "s/\[IP_VPS1\]/$DE_IP/g" BOOTSTRAP_NODES.md
sed -i "s/\[IP_VPS2\]/$CA_IP/g" BOOTSTRAP_NODES.md
sed -i "s/\[IP_VPS3\]/$IT_IP/g" BOOTSTRAP_NODES.md

# Remplacer dans README.md
sed -i "s/\[DE_NODE_IP\]/$DE_IP/g" README.md
sed -i "s/\[CA_NODE_IP\]/$CA_IP/g" README.md
sed -i "s/\[IT_NODE_IP\]/$IT_IP/g" README.md
```

---

## ✅ Vérification

Après avoir mis à jour, vérifiez que :

1. **Aucun placeholder ne reste** :
   ```bash
   grep -r "\[.*_IP.*\]" README.md BOOTSTRAP_NODES.md
   # Devrait ne rien retourner
   ```

2. **Les IP sont valides** (format XXX.XXX.XXX.XXX) :
   ```bash
   grep -E "([0-9]{1,3}\.){3}[0-9]{1,3}" README.md
   ```

3. **Testez la connexion** :
   ```bash
   curl http://VOTRE_IP_VPS:8001/status
   # Devrait retourner {"status":"running",...}
   ```

---

## 🚀 Après la Mise à Jour

1. **Commitez les changements** :
   ```bash
   git add README.md BOOTSTRAP_NODES.md
   git commit -m "docs: Update bootstrap node IPs with production values"
   git push
   ```

2. **Testez depuis un autre ordinateur** :
   ```bash
   # Sur un autre PC/serveur
   ./target/release/auriumchain \
     --mining \
     --wallet TEST_ADDRESS \
     --peer VOTRE_IP_VPS:3001
   ```

3. **Lancez les annonces** sur Reddit/Twitter avec les vraies IP !

---

## ⚠️ Sécurité

**NE PARTAGEZ JAMAIS :**
- ❌ Vos clés privées de wallet
- ❌ Vos mots de passe
- ❌ Vos clés SSH

**VOUS POUVEZ PARTAGER :**
- ✅ Les IP publiques des VPS (nécessaire pour le réseau)
- ✅ Les ports P2P/RPC (3001, 8001)
- ✅ Les adresses de wallet publiques

---

**Questions ?** Ouvrez une issue : https://github.com/Hicham60290/Auriumchain/issues
