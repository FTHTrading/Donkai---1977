# DONK AI — The Human Remembrance Protocol

> **"WHAT DO YOU REMEMBER?"**  
> *Sign your account. Preserve your evidence. Discover where memory converges, conflicts, and remains unresolved.*

[![Rust Tests](https://img.shields.io/badge/Rust-1.80%2B-blue?logo=rust)](https://www.rust-lang.org/)
[![LPS-1 Standard](https://img.shields.io/badge/Standard-LPS--1%20v2.0-emerald)](./LPS1_SPEC_v1.md)
[![EVM Compatibility](https://img.shields.io/badge/EVM-Chain%201977-slate)](./contracts)
[![License](https://img.shields.io/badge/License-MIT%20%2F%20Apache--2.0-gray)](./LICENSE-MIT)

---

## 1. Executive Overview

**DONK AI** is a global, human-governed memory and evidence network. It preserves what people remember, cryptographically proves when and how a record was submitted via **Living Provenance Standard 1 (LPS-1)**, binds supporting evidence to its provenance, maps independent blind corroboration across languages and cultures, and visibly separates personal recall from historically supported claims and open uncertainties.

DONK AI is **not** an artificial-memory system, a social-media feed, or a tokenized truth market. It is an infrastructure for **human remembrance**.

---

## 2. Non-Negotiable Epistemic Rules

The protocol technically and visibly enforces these distinctions across smart contracts, Rust engines, and UI layers:

$$\text{Popularity} \neq \text{Corroboration} \neq \text{Evidence Integrity} \neq \text{Historical Support}$$
$$\text{Stake} \neq \text{Truth}$$

### What an LPS-1 Record Establishes:
- A signer submitted a specified canonical record.
- The record existed no later than its chain-anchored timestamp.
- The committed record has not changed when its domain-separated Merkle root is recomputed.
- An artifact was supplied, hashed, attributed, and linked to a record.
- Multiple witnesses independently submitted overlapping accounts under blind conditions.
- A credentialed reviewer applied an open, versioned rubric.

### What an LPS-1 Record CANNOT Establish:
- That an author's subjective recollection is objectively true.
- That an artifact's interpretation is singular or complete.
- That consensus, upvotes, or token stakes constitute historical reality.
- That AI model outputs constitute truth verdicts.

---

## 3. Product Architecture

```
DONK AI Ecosystem
├── Web Experience (Obsidian & Liquid Metal Aesthetic)
│   ├── / (Overview & Epistemic Charter)
│   ├── /remember (Remembrance Studio with live LPS-1 canonicalization & client encryption)
│   ├── /atlas (Memory Drift Atlas: 1977 Cultural Era, Mandela divergences, era/language filters)
│   ├── /record/:id (5-Panel Living Provenance Record Ledger)
│   ├── /corroborate (Blind-First Independent Corroboration Flow)
│   ├── /proof (Client-Side LPS-1 Proof Inspector & Validator)
│   ├── /assembly (Role-Based Human Governance, Review Queues, & Treasury Grants)
│   └── /protocol (LPS-1 Specification & Research Charters)
│
├── Rust Crates (crates/)
│   ├── donkai-lps1 (Core LPS-1 v2 canonicalization, Merkle trees, proofs, validator)
│   ├── donkai-identity (Human Pass, WebAuthn Passkey, & DID credential adapters)
│   ├── donkai-evidence (Evidence manifests, SHA-256 artifact hashing, & access rules)
│   ├── donkai-corroboration (Blind commit-reveal primitives & salt verifier)
│   ├── donkai-translation (Signed derivative translation bundles & human attestations)
│   ├── donkai-review (Versioned review rubrics, assessments, & appeals)
│   ├── donkai-ipfs (CIDv1 raw & UnixFSv1 dag-pb computation & Kubo client)
│   └── donkai-node (Reference node & CLI verification engine)
│
└── EVM Smart Contracts (contracts/src/)
    ├── DonkaiMemoryLedger.sol (Primary LPS-1 commitment & status registry)
    ├── DonkaiHumanPass.sol (Non-transferable participation credential)
    ├── DonkaiEvidenceRegistry.sol (Artifact hash attachments & tier provenance)
    ├── DonkaiBlindCorroboration.sol (2-Phase commit-reveal independent witness protocol)
    ├── DonkaiReviewRegistry.sol (Bounded historical-support assessments & appeals)
    ├── DonkaiTranslationRegistry.sol (Signed derivative translation ledger)
    ├── DonkaiPrivacyRegistry.sol (Access policy hashes & revocation tombstones)
    ├── DonkaiResearchCredits.sol (Non-monetary milestone forecast credits)
    ├── DonkaiArchiveTreasury.sol (Non-factual archival preservation grants)
    └── DonkaiAnchorMirror.sol (Periodic Merkle checkpoint notary)
```

---

## 4. Quickstart & Verification

### Build & Run Rust Workspace
```bash
cargo build --workspace
cargo test --workspace
cargo run -p donkai-node
```

### CLI Tool Suite
```bash
# Validate a remembrance record
cargo run -p donkai-lps1 --bin donkai-lps1 -- validate sample_remembrance.json

# Compute canonical bytes and LPS-1 leaf commitment
cargo run -p donkai-lps1 --bin donkai-lps1 -- commit sample_remembrance.json

# Generate Merkle inclusion proof for a bundle leaf
cargo run -p donkai-lps1 --bin donkai-lps1 -- prove bundle.json --leaf 0x9504...99c2

# Verify an inclusion proof
cargo run -p donkai-lps1 --bin donkai-lps1 -- verify-proof proof.json
```

### Launch Web Interface
```bash
cd web
python -m http.server 8910
# Open http://localhost:8910 in your browser
```

---

## 5. Living Documentation Suite

- [WHITEPAPER.md](./WHITEPAPER.md) — Comprehensive technical and philosophical whitepaper.
- [DONKAI_CHARTER.md](./DONKAI_CHARTER.md) — Fundamental human remembrance constitution.
- [LPS1_SPEC_v1.md](./LPS1_SPEC_v1.md) — Complete byte-level LPS-1 standard specification.
- [RESEARCH_PROTOCOL_v0.1.md](./RESEARCH_PROTOCOL_v0.1.md) — Oral history and memory drift research protocols.
- [EVIDENCE_STANDARD_v0.1.md](./EVIDENCE_STANDARD_v0.1.md) — Artifact tiers, custody rules, and metadata requirements.
- [CORROBORATION_METHOD_v0.1.md](./CORROBORATION_METHOD_v0.1.md) — Blind commit-reveal methodology.
- [REVIEW_RUBRIC_v0.1.md](./REVIEW_RUBRIC_v0.1.md) — Bounded historical support evaluation rubrics.
- [PRIVACY_AND_CONSENT.md](./PRIVACY_AND_CONSENT.md) — Privacy modes, client encryption, and minor protection.
- [AI_ASSISTANCE_DISCLOSURE.md](./AI_ASSISTANCE_DISCLOSURE.md) — Strict boundaries on AI usage.
- [GOVERNANCE_AND_APPEALS.md](./GOVERNANCE_AND_APPEALS.md) — Role separation, Human Pass, and appeal procedures.
- [DATA_RETENTION_AND_REDACTION.md](./DATA_RETENTION_AND_REDACTION.md) — Tombstone lifecycles and redaction graphs.
- [THREAT_MODEL.md](./THREAT_MODEL.md) — Cryptographic, sybil, and adversarial threat assessments.

---

## 6. License

Dual-licensed under MIT OR Apache-2.0.
