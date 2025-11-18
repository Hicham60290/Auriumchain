# Template pour Reddit r/rust

**Titre du post** : [Project] AuriumChain - First PoW Blockchain with TLS-Encrypted P2P Networking

**Corps du message** :

---

Hey Rustaceans! 🦀

I've been working on **AuriumChain**, a proof-of-work blockchain that brings modern security standards to cryptocurrency.

**What makes it interesting from a Rust perspective:**

- Written entirely in Rust (no unsafe code in core logic)
- Leverages async/await with tokio for P2P networking
- Uses secp256k1, sha2, aes-gcm, argon2 crates
- 20/20 tests passing with comprehensive security coverage
- Clean architecture demonstrating Rust's strengths for blockchain

**Unique technical feature:**

AuriumChain implements TLS 1.2+ encryption for ALL P2P communications - something no other PoW blockchain does. This prevents network-level attacks and eavesdropping.

**Tech Stack:**
- tokio (async runtime)
- axum (RPC server)
- serde (serialization)
- secp256k1 (cryptography)
- Custom UTXO implementation

**Project Stats:**
- ~5,000 lines of Rust
- MIT licensed
- Full CI/CD with GitHub Actions
- Cross-platform (Linux, macOS, Windows)

**Looking for:**
- Code reviews
- Performance optimization suggestions
- Contributors interested in blockchain tech

**Repo**: https://github.com/Hicham60290/Auriumchain

Would love feedback from the Rust community, especially on:
- Architecture decisions
- Error handling patterns
- Async/await usage
- Testing strategies

Happy to answer any questions!

---

## Instructions pour poster :

1. Allez sur : https://www.reddit.com/r/rust/submit
2. Choisissez "Text Post"
3. Titre : [Project] AuriumChain - First PoW Blockchain with TLS-Encrypted P2P Networking
4. Copiez le corps du message ci-dessus
5. Flair : Sélectionnez "project" ou "show and tell" si disponible
6. Cliquez "Post"

## Conseils :

- Postez entre 14h-18h UTC (quand USA + Europe sont actifs)
- Évitez les week-ends
- Répondez rapidement aux commentaires (< 2h si possible)
- Soyez humble et ouvert aux critiques constructives
