# DONK AI Whitepaper: The Human Remembrance Protocol

**Version:** 0.1.0-draft  
**Status:** Shipped Prototype & Working Protocol  
**Domain:** Protocol Architecture, Epistemic Integrity & Cryptographic Provenance  

---

## 1. Abstract

Digital history is increasingly vulnerable to silent alteration, AI-generated synthetic revisionism, and algorithmic popularity loops that conflate viral consensus with historical reality. **DONK AI** establishes a decentralized, human-governed remembrance and evidence protocol. Anchored by **Living Provenance Standard 1 (LPS-1)**, the network enables individuals, communities, and archival institutions to preserve first-person testimony in its original language, cryptographically timestamp submission provenance, link physical artifacts to verifiable hashes, and gather blind independent human corroboration.

Crucially, the protocol makes **uncertainty visible**. It does not treat blockchain consensus, token holdings, or machine learning models as arbiters of truth. Instead, it maintains a structured 5-panel ledger distinguishing personal recollection, independent witness testimony, artifact integrity, bounded historical support, and open research questions.

---

## 2. The Crisis of Digital Memory

Three convergent vectors threaten the integrity of human collective memory:
1. **Synthetic Fluidity:** Generative AI models can produce plausible historical texts, images, and audio, eroding trust in unanchored digital documents.
2. **Bandwagon Epistemology:** Social platforms reward popular consensus, creating instant echo chambers where shared misconceptions (e.g., the Mandela Effect) are either dismissed as trivial or weaponized for speculative engagement.
3. **Financialization of Truth:** Prediction markets and tokenized governance models allow capital concentrations to purchase consensus, confusing economic stake with historical fact.

DONK AI resolves these failures not by declaring what is "true," but by cryptographically proving **who submitted what, when it was submitted, what evidence was attached, and what methodology was applied by human reviewers.**

---

## 3. Living Provenance Standard (LPS-1 v2.0)

LPS-1 is a typed, canonicalized, domain-separated cryptographic commitment standard.

### Core Data Objects:
1. **RemembranceStatement:** Canonical human prose, language tag (BCP 47), date range with explicit certainty, location descriptor with precision rating, and human-authorship attestation.
2. **ContextManifest:** Discovery metadata partitioned between public indexing fields and protected personal context.
3. **ConsentManifest:** Granular visibility policies (Public, Reviewer-Only, Trusted-Circle, Delayed-Public, Aggregate-Only, Private), identity modes, and retention rules.
4. **EvidenceBundle:** Content-addressed hashes (SHA-256 / CIDv1), source classes, evidence tiers, and explicit AI disclosure flags.
5. **CorroborationCommitment:** Sealed independent recall root committed prior to exposure to primary accounts.
6. **ReviewAssessment:** Bounded classification under versioned open rubrics with written rationale roots.
7. **VersionGraph:** Append-only graph of amendments, corrections, redactions, and retractions.

### Typed Domain Separation:
To prevent cross-type collision and semantic impersonation, all hashes use strict domain separation:
$$\text{Leaf} = \text{SHA256}(\text{"DONKAI:LPS1:LEAF:"} \parallel \text{TYPE} \parallel \text{":v1:"} \parallel \text{CanonicalBytes})$$
$$\text{Node} = \text{SHA256}(\text{"DONKAI:LPS1:NODE:v1:"} \parallel \text{LeftHash} \parallel \text{RightHash})$$
$$\text{Root} = \text{SHA256}(\text{"DONKAI:LPS1:ROOT:"} \parallel \text{BUNDLE} \parallel \text{":v1:"} \parallel \text{TreeRoot})$$

---

## 4. Blind Independent Corroboration

A fundamental product innovation of DONK AI is the elimination of bandwagon bias through **blind commit-reveal protocols**.

1. **Discovery Stage:** A potential witness views only neutral discovery parameters (era: 1977–1980, place: Austin, category: arcade, tags: [tokens, coin-op]). The primary narrative remains concealed.
2. **Local Sealing:** The witness authors their recollection locally. The client computes:
   $$\text{SealedRoot} = \text{SHA256}(\text{"DONKAI:LPS1:BLIND\_CORROBORATION:v1:"} \parallel \text{Salt} \parallel \text{Narrative})$$
3. **On-Chain Commitment:** The sealed root is anchored to `DonkaiBlindCorroboration.sol`.
4. **Reveal & Peer Analysis:** After an embargo or review trigger, the witness reveals the narrative and salt. Reviewers analyze genuine thematic convergence without the corrupting influence of social conformity.

---

## 5. Bounded Historical Support & The 5-Panel Ledger

Public records are rendered through a mandatory 5-panel ledger:
1. **What is Remembered:** Subjective testimony and original prose.
2. **What is Independently Recalled:** Blind witness metrics, geographic distribution, and overlap/divergence themes.
3. **What Evidence Was Submitted:** Contemporaneous artifacts, photographs, newspaper clippings, and digital records.
4. **What Can Be Historically Supported:** Bounded review classifications:
   - *Historically Supported*
   - *Partially Supported*
   - *Conflicting Evidence*
   - *Unresolved*
   - *Insufficient Evidence*
   - *Retracted by Author*
5. **What Remains Unresolved:** Explicit documentation of missing archives, conflicting witness accounts, and open research questions.

---

## 6. Tokenomics & Non-Financial Governance

DONK AI intentionally rejects speculative token mechanics for truth determination:
- **No Truth Markets:** Financial stakes cannot purchase, alter, or vote on historical classifications.
- **Human Pass (SBT):** Soulbound credentials ensure sybil resistance and verify curator roles.
- **Archive Treasury:** Protocol grants are allocated exclusively for archival digitizations, community oral history projects, and multilingual translation subsidies.
- **Research Credits:** Non-transferable, non-monetary protocol forecast credits are permitted only for verifiable operational milestones (e.g., reaching witness counts), never on human trauma or historical veracity.

---

## 7. Status & Roadmap

| Component | Status | Target |
| :--- | :--- | :--- |
| LPS-1 v2.0 Rust Core | **Shipped** | crates/donkai-lps1 |
| Solidity EVM Suite | **Shipped** | contracts/src/ |
| 5-Panel Web Experience | **Shipped** | web/ |
| Memory Drift Atlas Pilot | **Shipped** | 1977 Foundation Collection |
| Zero-Knowledge Personhood Adapters | *Prototype* | donkai-identity v0.2 |
| Encrypted Decentralized Storage RPC | *In Development* | donkai-ipfs v0.2 |
