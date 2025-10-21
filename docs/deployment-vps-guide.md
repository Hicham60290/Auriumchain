# Guide de Déploiement AuriumChain - 3 VPS

**Date**: 2025-10-21
**Configuration**: 3 VPS avec wallets classiques
**Blockchain**: Fresh start avec quantum hashing intégré

---

## 📋 Vue d'Ensemble

```
Architecture Finale:
═══════════════════════════════════════════════════════

VPS1 (Bootstrap Node - Principal)
├─ IP: [À remplir]
├─ Wallet: AUR3xxxxx (wallet_vps1.txt)
├─ Rôle: Nœud principal, mining
└─ Peers: Aucun (il est le premier)

VPS2 (Nœud Secondaire)
├─ IP: [À remplir]
├─ Wallet: AUR3xxxxx (wallet_vps2.txt)
├─ Rôle: Mining, validation
└─ Peers: VPS1

VPS3 (Nœud Tertiaire)
├─ IP: [À remplir]
├─ Wallet: AUR3xxxxx (wallet_vps3.txt)
├─ Rôle: Mining, validation
└─ Peers: VPS1, VPS2
```

---

## 🔑 ÉTAPE 1: Vérifier vos Wallets (Sur PC Local)

### **Vérifier que les wallets existent**:

```bash
cd ~/auriumchain-core/

# Lister les fichiers wallet
ls -lh wallet_vps*.txt

# Devrait montrer:
# wallet_vps1.txt
# wallet_vps2.txt
# wallet_vps3.txt
```

### **Extraire les adresses**:

```bash
# Créer un fichier récapitulatif
cat > vps_addresses.txt <<'EOF'
════════════════════════════════════════════════════════
  WALLETS AURIUMCHAIN - 3 VPS
════════════════════════════════════════════════════════

VPS1 (Bootstrap Node):
EOF

echo "Address: $(grep "Address:" wallet_vps1.txt | awk '{print $2}')" >> vps_addresses.txt
echo "Public:  $(grep "Public Key:" wallet_vps1.txt | awk '{print $3}')" >> vps_addresses.txt
echo "" >> vps_addresses.txt

cat >> vps_addresses.txt <<'EOF'
VPS2 (Secondary Node):
EOF

echo "Address: $(grep "Address:" wallet_vps2.txt | awk '{print $2}')" >> vps_addresses.txt
echo "Public:  $(grep "Public Key:" wallet_vps2.txt | awk '{print $3}')" >> vps_addresses.txt
echo "" >> vps_addresses.txt

cat >> vps_addresses.txt <<'EOF'
VPS3 (Tertiary Node):
EOF

echo "Address: $(grep "Address:" wallet_vps3.txt | awk '{print $2}')" >> vps_addresses.txt
echo "Public:  $(grep "Public Key:" wallet_vps3.txt | awk '{print $3}')" >> vps_addresses.txt

# Afficher le résultat
cat vps_addresses.txt
```

### **Sauvegarder de manière SÉCURISÉE**:

```bash
# 1. Combiner tous les wallets
cat wallet_vps1.txt wallet_vps2.txt wallet_vps3.txt > all_vps_wallets.txt

# 2. Chiffrer avec GPG
gpg --symmetric --cipher-algo AES256 all_vps_wallets.txt
# Entrez un mot de passe TRÈS FORT et notez-le!

# 3. Créer plusieurs backups
cp all_vps_wallets.txt.gpg ~/Documents/backup_vps_wallets_$(date +%Y%m%d).gpg
cp all_vps_wallets.txt.gpg /media/usb/backup_vps_wallets.gpg  # USB

# 4. SUPPRIMER les fichiers non-chiffrés (SÉCURITÉ!)
shred -u wallet_vps1.txt wallet_vps2.txt wallet_vps3.txt all_vps_wallets.txt

# 5. Garder vps_addresses.txt pour référence (pas de clés privées)
```

**⚠️ CRITIQUE**: Les clés privées sont maintenant UNIQUEMENT dans `all_vps_wallets.txt.gpg`!

---

## 🚀 ÉTAPE 2: Préparer le Script de Déploiement

### **Sur votre PC local**:

```bash
cd ~/auriumchain-core/

# Le script deploy_vps.sh existe déjà
ls -lh deploy_vps.sh

# Éditer le script pour ajouter vos adresses
nano deploy_vps.sh
```

### **Configuration à modifier dans le script**:

```bash
# Ligne ~15-17: Remplacer par vos VRAIES adresses
VPS1_WALLET="AUR3xxxxx"  # Copier depuis vps_addresses.txt
VPS2_WALLET="AUR3yyyyy"  # Copier depuis vps_addresses.txt
VPS3_WALLET="AUR3zzzzz"  # Copier depuis vps_addresses.txt

# Ligne ~19-21: Remplacer par vos IPs VPS
VPS1_IP="123.456.789.1"  # IP de votre VPS1
VPS2_IP="123.456.789.2"  # IP de votre VPS2
VPS3_IP="123.456.789.3"  # IP de votre VPS3
```

### **Copier le script vers chaque VPS**:

```bash
# Copier vers VPS1
scp deploy_vps.sh user@VPS1_IP:~/

# Copier vers VPS2
scp deploy_vps.sh user@VPS2_IP:~/

# Copier vers VPS3
scp deploy_vps.sh user@VPS3_IP:~/
```

---

## 🖥️ ÉTAPE 3: Déployer VPS1 (Bootstrap Node)

### **SSH vers VPS1**:

```bash
ssh user@VPS1_IP
```

### **Exécuter le déploiement**:

```bash
# Rendre le script exécutable
chmod +x deploy_vps.sh

# Lancer le déploiement VPS1
./deploy_vps.sh vps1

# Le script va:
# 1. Installer les dépendances (Rust, Git, etc.)
# 2. Cloner le repository
# 3. Compiler le projet (~5-10 min)
# 4. Créer le service systemd
# 5. Démarrer le mining
```

**Sortie attendue**:
```
╔═══════════════════════════════════════════════╗
║   AuriumChain VPS Deployment Script          ║
║   Target: VPS1                                ║
╚═══════════════════════════════════════════════╝

✓ Rust installé
✓ Git installé
✓ Outils de compilation installés
✓ Code à jour
✓ Compilation réussie
✓ Service systemd créé
✓ Service démarré

╔═══════════════════════════════════════════════╗
║   VPS1 Déployé avec Succès! ✅              ║
╚═══════════════════════════════════════════════╝
```

### **Vérifier que ça fonctionne**:

```bash
# Voir le statut
sudo systemctl status auriumchain

# Voir les logs en temps réel
sudo journalctl -u auriumchain -f

# Devrait afficher:
# AuriumChain Node - TLS P2P Edition
# Mining:    true
# Miner addr: AUR3xxxxx
# ⛏️  Mining block 1 (difficulty 1)...
```

### **Attendre le premier bloc**:

```bash
# Le premier bloc devrait être miné en ~30 secondes
# Vous verrez:
# ✅ Block 1 mined in 23s!
#    Hash: 0abc123def... (64 hex chars, quantum-safe)
#    Nonce: 42
```

**⚠️ NE PAS PASSER À VPS2/VPS3 AVANT QUE VPS1 AIT MINÉ AU MOINS 1 BLOC!**

---

## 🖥️ ÉTAPE 4: Déployer VPS2 (Après VPS1 a 1+ blocs)

### **SSH vers VPS2**:

```bash
ssh user@VPS2_IP
```

### **Exécuter le déploiement**:

```bash
chmod +x deploy_vps.sh
./deploy_vps.sh vps2

# Le script configure automatiquement:
# --peers VPS1_IP:3001
```

### **Vérifier la synchronisation**:

```bash
sudo journalctl -u auriumchain -f

# Devrait afficher:
# 🔄 Starting initial synchronization...
# Blockchain loaded: 1 blocks
# ⛏️  Mining block 2 (difficulty 1)...
```

---

## 🖥️ ÉTAPE 5: Déployer VPS3 (Après VPS2 synchronisé)

### **SSH vers VPS3**:

```bash
ssh user@VPS3_IP
```

### **Exécuter le déploiement**:

```bash
chmod +x deploy_vps.sh
./deploy_vps.sh vps3

# Le script configure automatiquement:
# --peers VPS1_IP:3001,VPS2_IP:3001
```

### **Vérifier la synchronisation**:

```bash
sudo journalctl -u auriumchain -f

# Devrait afficher:
# 🔄 Starting initial synchronization...
# Blockchain loaded: X blocks
# ⛏️  Mining block X+1...
```

---

## ✅ ÉTAPE 6: Vérification Finale

### **Sur CHAQUE VPS, vérifier**:

```bash
# 1. Service actif
sudo systemctl status auriumchain
# Devrait montrer: active (running)

# 2. Nombre de blocs
# TODO: Ajouter commande RPC pour vérifier blockchain height

# 3. Connexions P2P
sudo journalctl -u auriumchain | grep "Connected to peer"
# Devrait montrer connexions entre VPS

# 4. Mining actif
sudo journalctl -u auriumchain | grep "Mining block"
# Devrait montrer le mining en cours
```

### **Tester la connectivité P2P**:

```bash
# Sur VPS1
telnet VPS2_IP 3001
# Devrait se connecter (Ctrl+C pour quitter)

# Sur VPS2
telnet VPS1_IP 3001
telnet VPS3_IP 3001

# Sur VPS3
telnet VPS1_IP 3001
telnet VPS2_IP 3001
```

---

## 📊 Monitoring

### **Commandes Utiles**:

```bash
# Voir les logs en temps réel
sudo journalctl -u auriumchain -f

# Voir les dernières 100 lignes
sudo journalctl -u auriumchain -n 100

# Redémarrer le service
sudo systemctl restart auriumchain

# Arrêter le service
sudo systemctl stop auriumchain

# Démarrer le service
sudo systemctl start auriumchain
```

### **Vérifier qui mine des blocs**:

```bash
# Voir quel wallet mine
sudo journalctl -u auriumchain | grep "mined"

# Exemple de sortie:
# ✅ Block 5 mined by AUR3xxxxx (VPS1)
# ✅ Block 6 mined by AUR3yyyyy (VPS2)
# ✅ Block 7 mined by AUR3zzzzz (VPS3)
```

---

## 🔥 En Cas de Problème

### **Problème: Service ne démarre pas**

```bash
# Voir les erreurs
sudo journalctl -u auriumchain -n 50

# Vérifier la configuration
cat /etc/systemd/system/auriumchain.service

# Tester manuellement
cd ~/auriumchain-core
./target/release/auriumchain --mining --miner-address AUR3xxxxx
```

### **Problème: Pas de connexion P2P**

```bash
# Vérifier le firewall
sudo ufw status

# Ouvrir le port 3001 si nécessaire
sudo ufw allow 3001/tcp

# Vérifier que le port écoute
sudo netstat -tlnp | grep 3001
```

### **Problème: Mining très lent**

```bash
# Vérifier la difficulté
sudo journalctl -u auriumchain | grep "difficulty"

# Devrait être 1 au début
# Si trop élevé, possibilité de redémarrer fresh
```

### **Redémarrage complet (Fresh start)**:

```bash
# Arrêter le service
sudo systemctl stop auriumchain

# Supprimer la blockchain
rm -rf /tmp/auriumchain.json
rm -rf ~/.auriumchain/

# Redémarrer
sudo systemctl start auriumchain

# Le genesis block sera recréé
```

---

## 📈 Performance Attendue

### **Avec 3 VPS qui minent**:

```
Temps de bloc: ~30 secondes
Difficulté initiale: 1
Récompense par bloc: 50 AUR

Production par jour (théorique):
├─ Blocs/jour: 2,880 blocs (1 bloc/30s)
├─ AUR/jour: 144,000 AUR (50 × 2,880)
└─ Par VPS: ~48,000 AUR/jour (si distribution égale)

En pratique:
└─ VPS le plus rapide mine plus souvent
   (dépend du CPU et de la latence réseau)
```

---

## 🎯 Checklist de Déploiement

### **Avant de Commencer**:

- [ ] 3 wallets créés (wallet_vps1/2/3.txt)
- [ ] Adresses extraites et notées
- [ ] Clés privées sauvegardées (GPG + USB + Cloud)
- [ ] Clés privées ORIGINALES supprimées (shred)
- [ ] Script deploy_vps.sh configuré avec IPs et adresses
- [ ] Script copié sur les 3 VPS

### **Déploiement**:

- [ ] VPS1 déployé et mining
- [ ] VPS1 a miné au moins 1 bloc
- [ ] VPS2 déployé et synchronisé
- [ ] VPS3 déployé et synchronisé
- [ ] Les 3 VPS se voient en P2P
- [ ] Les 3 VPS minent activement

### **Vérification**:

- [ ] Services systemd actifs sur les 3 VPS
- [ ] Logs montrent mining actif
- [ ] Connectivité P2P OK
- [ ] Blocs se propagent entre VPS
- [ ] Pas d'erreurs dans les logs

---

## 🔐 Sécurité Post-Déploiement

### **Sur CHAQUE VPS**:

```bash
# 1. Firewall (n'autoriser que SSH + P2P)
sudo ufw default deny incoming
sudo ufw default allow outgoing
sudo ufw allow 22/tcp   # SSH
sudo ufw allow 3001/tcp # AuriumChain P2P
sudo ufw enable

# 2. Fail2ban (protection SSH)
sudo apt install fail2ban -y
sudo systemctl enable fail2ban
sudo systemctl start fail2ban

# 3. Mises à jour auto
sudo apt install unattended-upgrades -y
sudo dpkg-reconfigure -plow unattended-upgrades
```

---

## 📞 Support

### **Si vous rencontrez des problèmes**:

1. Vérifier les logs: `sudo journalctl -u auriumchain -n 100`
2. Vérifier le statut: `sudo systemctl status auriumchain`
3. Tester manuellement: `cd ~/auriumchain-core && ./target/release/auriumchain --mining --miner-address AUR3xxxxx`

### **Informations utiles pour debug**:

```bash
# Version Rust
rustc --version

# État du service
sudo systemctl status auriumchain

# Ports ouverts
sudo netstat -tlnp

# Connexions P2P actives
sudo netstat -tnp | grep 3001

# Espace disque
df -h

# RAM utilisée
free -h
```

---

## ✅ Déploiement Réussi!

Une fois que vous voyez:

```
VPS1: ✅ Mining, 10+ blocs
VPS2: ✅ Mining, synchronisé
VPS3: ✅ Mining, synchronisé
P2P:  ✅ Les 3 VPS connectés
```

**Félicitations! Votre blockchain AuriumChain est opérationnelle! 🎉**

---

**Prochaines étapes**:
1. Monitorer la production quotidienne
2. Créer un wallet quantum pour épargne (optionnel)
3. Implémenter RPC pour vérifier les soldes
4. Configurer alertes (monitoring)
