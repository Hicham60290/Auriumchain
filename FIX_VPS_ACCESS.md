# 🔧 Corriger l'Accès Public aux Nœuds VPS

## ⚠️ Problème

Les 3 VPS retournent "Access denied" pour les requêtes RPC externes.

**Cause** : Le nœud écoute uniquement sur `127.0.0.1` (localhost) au lieu de `0.0.0.0` (toutes les interfaces).

---

## ✅ Solution (Sur CHAQUE VPS)

### Étape 1 : Se Connecter au VPS

```bash
# VPS Francfort
ssh root@135.125.174.27

# VPS Beauharnois
ssh root@158.69.1.236

# VPS Milan
ssh root@57.131.22.120
```

---

### Étape 2 : Vérifier le Processus Actuel

```bash
ps aux | grep auriumchain
```

**Vérifiez si `--host 0.0.0.0` est présent** dans la commande.

---

### Étape 3 : Ouvrir les Ports (Si pas déjà fait)

```bash
# Firewall UFW (Ubuntu/Debian)
sudo ufw allow 3001/tcp
sudo ufw allow 8001/tcp
sudo ufw reload
sudo ufw status

# Ou Firewalld (CentOS/Rocky)
sudo firewall-cmd --permanent --add-port=3001/tcp
sudo firewall-cmd --permanent --add-port=8001/tcp
sudo firewall-cmd --reload
```

---

### Étape 4 : Arrêter le Nœud Actuel

```bash
# Trouver le PID
ps aux | grep auriumchain

# Tuer le processus
pkill auriumchain

# Ou avec le PID spécifique
kill <PID>
```

---

### Étape 5 : Relancer avec --host 0.0.0.0

#### Option A : Lancement Direct (Pour Test)

```bash
cd /chemin/vers/Auriumchain

./target/release/auriumchain \
  --mining \
  --wallet VOTRE_ADRESSE_AUR \
  --host 0.0.0.0 \
  --port 3001 \
  --rpc-port 8001 \
  --data-file /var/lib/auriumchain/blockchain.json
```

**⚠️ Problème** : Se ferme si vous déconnectez SSH !

---

#### Option B : Avec screen (Recommandé)

```bash
# Installer screen si pas déjà fait
sudo apt install screen  # Ubuntu/Debian
sudo yum install screen  # CentOS/Rocky

# Lancer dans screen
screen -S auriumchain

# Dans screen, lancer le nœud
cd /chemin/vers/Auriumchain

./target/release/auriumchain \
  --mining \
  --wallet VOTRE_ADRESSE_AUR \
  --host 0.0.0.0 \
  --port 3001 \
  --rpc-port 8001 \
  --data-file /var/lib/auriumchain/blockchain.json

# Détacher screen : Ctrl+A puis D
# Reconnecter à screen : screen -r auriumchain
```

---

#### Option C : Avec systemd (Meilleur pour Production)

Créez `/etc/systemd/system/auriumchain.service` :

```ini
[Unit]
Description=AuriumChain Node
After=network.target

[Service]
Type=simple
User=root
WorkingDirectory=/root/Auriumchain
ExecStart=/root/Auriumchain/target/release/auriumchain \
  --mining \
  --wallet VOTRE_ADRESSE_AUR \
  --host 0.0.0.0 \
  --port 3001 \
  --rpc-port 8001 \
  --data-file /var/lib/auriumchain/blockchain.json
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

Activez le service :

```bash
sudo systemctl daemon-reload
sudo systemctl enable auriumchain
sudo systemctl start auriumchain
sudo systemctl status auriumchain
```

---

### Étape 6 : Vérifier que Ça Marche

```bash
# Depuis le VPS lui-même
curl http://localhost:8001/status

# Devrait retourner le status du nœud
```

---

### Étape 7 : Tester l'Accès Externe

**Depuis votre PC Windows** :

```powershell
# Test VPS Francfort
(Invoke-WebRequest http://135.125.174.27:8001/status).Content

# Test VPS Beauharnois
(Invoke-WebRequest http://158.69.1.236:8001/status).Content

# Test VPS Milan
(Invoke-WebRequest http://57.131.22.120:8001/status).Content
```

**Devrait retourner** :
```json
{"status":"running","version":"1.0.0","block_height":XXXX,...}
```

**PAS** : "Access denied"

---

## 🔍 Vérification des Ports

### Vérifier que les Ports Écoutent

```bash
# Sur le VPS
netstat -tuln | grep 3001
netstat -tuln | grep 8001

# Devrait montrer :
# tcp6   0   0 :::3001   :::*   LISTEN
# tcp6   0   0 :::8001   :::*   LISTEN
```

Si `127.0.0.1:3001` au lieu de `:::3001` → **PROBLÈME** : pas en `--host 0.0.0.0`

---

## 🚨 Troubleshooting

### "Connection refused"

**Cause** : Firewall bloque ou port non ouvert

**Solution** :
```bash
sudo ufw allow 3001/tcp
sudo ufw allow 8001/tcp
```

---

### "Access denied"

**Cause** : Nœud écoute sur 127.0.0.1 uniquement

**Solution** : Relancer avec `--host 0.0.0.0`

---

### "Connection timeout"

**Cause** : Nœud pas démarré ou crash

**Solution** :
```bash
ps aux | grep auriumchain
# Si rien → redémarrer le nœud
```

---

## ✅ Checklist Finale

**Pour CHAQUE VPS** :

- [ ] SSH connecté
- [ ] Ports 3001 et 8001 ouverts dans firewall
- [ ] Nœud arrêté (`pkill auriumchain`)
- [ ] Nœud relancé avec `--host 0.0.0.0`
- [ ] Test local réussi (`curl localhost:8001/status`)
- [ ] Test externe réussi (depuis votre PC)
- [ ] Nœud configuré pour auto-restart (systemd ou screen)

---

## 📊 Après Correction

Une fois les 3 VPS accessibles :

1. **Testez la connexion** avec un nouveau mineur
2. **Mergez la Pull Request** vers main
3. **Publiez les annonces** sur Reddit/Twitter

---

## 💡 Commande Rapide (Tout-en-un)

Exécutez ceci sur chaque VPS :

```bash
# Arrêter
pkill auriumchain

# Ouvrir ports
sudo ufw allow 3001/tcp && sudo ufw allow 8001/tcp

# Relancer (remplacez VOTRE_ADRESSE)
cd /root/Auriumchain && screen -dmS aur ./target/release/auriumchain --mining --wallet VOTRE_ADRESSE --host 0.0.0.0 --port 3001 --rpc-port 8001

# Vérifier
sleep 3 && curl localhost:8001/status
```

---

**Questions ?** Contactez-moi ou consultez les logs : `journalctl -u auriumchain -f` (si systemd)
