# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 1.0.x   | ✅ Yes            |
| < 1.0   | ❌ No             |

## Reporting a Vulnerability

**DO NOT report security vulnerabilities in public issues.**

### How to Report

1. **GitHub Security Advisory** (preferred):
   - Go to Security tab → Report a vulnerability
   - Provide details privately

2. **Private Discussion**: For less critical issues

### What to Include

- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)

### Response Time

- Initial response: **48 hours**
- Status updates: Every 72 hours
- Fix timeline: Depends on severity

## Bug Bounty Program

We offer rewards for critical vulnerabilities:

| Severity | Reward |
|----------|--------|
| Critical | 10,000 AUR |
| High | 5,000 AUR |
| Medium | 1,000 AUR |
| Low | 500 AUR |

### Scope

✅ **In scope:**
- Blockchain core vulnerabilities
- Consensus mechanism issues
- Cryptographic weaknesses
- Wallet security issues
- RPC API vulnerabilities

❌ **Out of scope:**
- Social engineering
- Physical attacks
- DDoS attacks
- Third-party dependencies

## Security Best Practices

### For Users

1. ✅ Backup seed phrases on paper (never digital)
2. ✅ Never share private keys or seed phrases
3. ✅ Use strong passwords (16+ characters)
4. ✅ Keep software updated
5. ✅ Verify download checksums

### For Node Operators

1. ✅ Keep VPS/server updated
2. ✅ Use firewall (ufw recommended)
3. ✅ Monitor logs regularly
4. ✅ Use SSH keys (disable password auth)
5. ✅ Regular backups of configuration

## Disclosure Policy

- Responsible disclosure: **90 days**
- Critical issues: Immediate patch release
- Public disclosure after fix is available
- CVE assigned for serious vulnerabilities
- Contributors credited (if desired)

## Contact

- **Security Issues**: GitHub Security Advisory
- **General Security Questions**: GitHub Discussions

---

**Security is everyone's responsibility. Report issues responsibly.**

*Last updated: October 2025*
