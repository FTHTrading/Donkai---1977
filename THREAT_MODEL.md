# DONK AI Threat Model & Security Architecture

**Document Code:** `SEC-THR-1.0`  
**Purpose:** Comprehensive security, privacy, sybil resistance, and adversarial analysis.

---

## 1. Adversarial Scenarios & Mitigations

### 1.1 Synthetic Swarm Ingestion (AI Bot Sybil Attack)
- **Threat:** An adversary floods the protocol with AI-generated witness statements to create false corroboration consensus.
- **Mitigation:** Mandatory `DonkaiHumanPass.sol` identity credentials for corroboration weighting, rate limiting per pass, and human reviewer inspection.

### 1.2 Bandwagon & Hindsight Anchoring Bias
- **Threat:** A witness reads an initial account and retrofits their own memory to match the author's narrative.
- **Mitigation:** Blind Commit-Reveal (`DonkaiBlindCorroboration.sol`). Witnesses only see neutral metadata (time, broad place, category) and must seal their recall on-chain before reading the primary text.

### 1.3 Pre-Image & Cross-Type Collision Attacks
- **Threat:** An attacker crafts an evidence artifact hash that collides with a remembrance statement leaf.
- **Mitigation:** Typed domain separation prefixes on all SHA-256 leaves (`DONKAI:LPS1:LEAF:<TYPE>:v1:`) and roots.

### 1.4 Financialized Truth Capture
- **Threat:** Wealthy entities buy governance tokens to dictate historical classifications.
- **Mitigation:** Absolute separation of stake from truth. Tokens cannot vote on historical veracity. Governance roles are soulbound and rubric-enforced.

### 1.5 Privacy Leakage of Sensitive Testimony
- **Threat:** A sensitive or political memory is exposed to public indexing.
- **Mitigation:** Client-side AES-GCM-256 encryption. Only the 32-byte Merkle root touches the blockchain.
