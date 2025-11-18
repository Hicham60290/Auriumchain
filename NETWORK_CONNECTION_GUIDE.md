# 🌐 Guide de Connexion au Réseau AuriumChain

## ⚠️ IMPORTANT : Connexion Manuelle Requise

AuriumChain nécessite actuellement une connexion manuelle à un pair (peer) existant pour rejoindre le réseau.

---

## 🎯 Pour les Nouveaux Mineurs

### Étape 1 : Installer AuriumChain

```bash
# Cloner le repository
git clone https://github.com/Hicham60290/Auriumchain.git
cd Auriumchain

# Compiler
cargo build --release
```

### Étape 2 : Se Connecter au Réseau

**Vous devez vous connecter à un nœud de bootstrap (seed node)**

```bash
./target/release/auriumchain \
  --mining \
  --wallet VOTRE_ADRESSE_AUR \
  --peer ADRESSE_IP_BOOTSTRAP:3001
```

### Exemple Concret

```bash
# Remplacez XXX.XXX.XXX.XXX par l'IP du nœud bootstrap
./target/release/auriumchain \
  --mining \
  --wallet AUR_VOTRE_ADRESSE_ICI \
  --peer XXX.XXX.XXX.XXX:3001
```

---

## 🌟 Nœuds de Bootstrap Officiels

### Nœud Principal (Officiel)

**À METTRE À JOUR avec votre adresse IP publique :**

```
IP: [VOTRE_IP_PUBLIQUE_ICI]
Port P2P: 3001
Port RPC: 8001
```

**Commande de connexion** :
```bash
./target/release/auriumchain \
  --mining \
  --wallet VOTRE_ADRESSE \
  --peer [VOTRE_IP]:3001
```

---

## 🔧 Configuration Réseau

### Pour le Propriétaire du Nœud Bootstrap

**IMPORTANT** : Pour accepter les connexions externes, vous devez :

#### 1. Lancer avec l'option `--host 0.0.0.0`

```bash
./target/release/auriumchain \
  --genesis \
  --mining \
  --wallet VOTRE_ADRESSE \
  --host 0.0.0.0 \
  --port 3001
```

**⚠️ ATTENTION** : Actuellement le code bind sur `127.0.0.1` par défaut, ce qui signifie que **PERSONNE ne peut se connecter de l'extérieur** !

#### 2. Ouvrir les Ports dans votre Firewall

**Linux (ufw)** :
```bash
sudo ufw allow 3001/tcp
sudo ufw allow 8001/tcp
```

**Windows** :
- Panneau de configuration → Pare-feu Windows
- Règles de trafic entrant → Nouvelle règle
- Port TCP 3001 et 8001

#### 3. Configuration du Routeur (NAT/Port Forwarding)

Si vous êtes derrière un routeur :
- Connectez-vous à votre routeur (généralement 192.168.1.1)
- Allez dans "Port Forwarding" ou "NAT"
- Créez une règle :
  - Port externe : 3001 → IP locale : [VOTRE_IP_LOCALE]:3001
  - Port externe : 8001 → IP locale : [VOTRE_IP_LOCALE]:8001

#### 4. Trouver Votre IP Publique

```bash
# Linux/Mac
curl ifconfig.me

# Ou visitez
https://whatismyipaddress.com/
```

---

## 📋 Checklist pour le Lancement Public

### Pour le Propriétaire du Projet (VOUS)

- [ ] Lancer le nœud Genesis avec `--host 0.0.0.0`
- [ ] Configurer le firewall pour ouvrir le port 3001
- [ ] Configurer le routeur (port forwarding si nécessaire)
- [ ] Récupérer votre IP publique
- [ ] Mettre à jour le README avec votre IP de bootstrap
- [ ] Créer un fichier BOOTSTRAP_NODES.md avec la liste des nœuds
- [ ] Tester la connexion depuis un autre ordinateur/réseau

### Pour les Mineurs

- [ ] Compiler AuriumChain
- [ ] Récupérer l'IP du nœud bootstrap depuis le README
- [ ] Lancer avec `--peer IP_BOOTSTRAP:3001`
- [ ] Vérifier la synchronisation dans les logs

---

## 🚨 Problèmes Courants

### "Connection refused"

**Cause** : Le nœud bootstrap n'est pas accessible
**Solution** :
1. Vérifier que le nœud bootstrap est lancé
2. Vérifier le firewall
3. Vérifier le port forwarding
4. Ping l'IP du bootstrap pour tester la connectivité

### "No peers connected"

**Cause** : Mauvaise adresse IP ou port
**Solution** :
1. Vérifier l'adresse IP du bootstrap
2. Vérifier que le port est 3001
3. Essayer avec `--peer IP:3001` (pas de http://)

### "Blockchain not syncing"

**Cause** : Problème de synchronisation P2P
**Solution** :
1. Redémarrer le nœud
2. Vérifier les logs pour les erreurs TLS
3. S'assurer que la version du code est la même

---

## 💡 Améliorations Futures

Pour faciliter la connexion, les améliorations suivantes sont prévues :

- **v1.1** : Liste de nœuds de bootstrap hardcodés dans le code
- **v1.2** : Découverte automatique de pairs (peer discovery)
- **v1.3** : DNS seeds pour la découverte de nœuds
- **v2.0** : DHT (Distributed Hash Table) pour découverte décentralisée

---

## 📞 Support

Si vous ne parvenez pas à vous connecter au réseau :

1. Vérifiez les [Issues GitHub](https://github.com/Hicham60290/Auriumchain/issues)
2. Créez une issue avec :
   - Votre système d'exploitation
   - Les logs d'erreur
   - La commande utilisée
3. Rejoignez les [Discussions](https://github.com/Hicham60290/Auriumchain/discussions)

---

## ⚙️ Commandes Utiles

### Vérifier les Pairs Connectés

```bash
# Via RPC
curl http://localhost:8001/peers
```

### Vérifier la Longueur de la Chaîne

```bash
curl http://localhost:8001/chain/length
```

### Logs Détaillés

```bash
RUST_LOG=debug ./target/release/auriumchain --mining --peer IP:3001
```

---

**Important** : Pour le moment, le réseau AuriumChain dépend de nœuds de bootstrap centralisés. Assurez-vous de maintenir au moins un nœud public accessible 24/7 pour permettre aux nouveaux mineurs de rejoindre le réseau.
