# LPS-1 Living Provenance Standard (Specification v2.0)

**Standard Identifier:** `DONKAI:LPS1:v2.0`  
**Status:** Shipped Specification  
**Reference Implementation:** `crates/donkai-lps1`  

---

## 1. Specification Scope

The Living Provenance Standard 1 (LPS-1) defines deterministic canonical serialization rules, domain-separated cryptographic hashing, and Merkle tree commitments for human remembrances, evidence manifests, blind corroborations, and peer assessments.

---

## 2. Canonicalization Rules

A valid LPS-1 canonical object MUST satisfy:
1. **Encoding:** UTF-8 only.
2. **Normalization:** Unicode NFC (Normalization Form C).
3. **Key Ordering:** Object keys sorted strictly in recursive lexicographical (ASCII byte) order.
4. **Whitespace:** Compact JSON delimiters (`:` and `,`) with no insignificant spaces or line breaks.
5. **Prose Fidelity:** Original text must preserve exact character sequence, capitalization, dialect, slang, and punctuation.
6. **Timestamps:** RFC 3339 UTC format (e.g. `2026-08-29T06:36:00Z`).
7. **Dates:** ISO 8601 extended format for remembered events (`YYYY-MM-DD` or `YYYY`).

---

## 3. Cryptographic Primitives & Domain Separation

LPS-1 utilizes SHA-256 (FIPS 180-4) with mandatory domain separation prefixes:

### 3.1 Leaf Hashing
$$\text{LeafHash} = \text{SHA256}(\text{"DONKAI:LPS1:LEAF:"} \parallel \text{TYPE\_UPPERCASE} \parallel \text{":v1:"} \parallel \text{CanonicalBytes})$$

### 3.2 Internal Node Hashing
$$\text{NodeHash} = \text{SHA256}(\text{"DONKAI:LPS1:NODE:v1:"} \parallel \text{LeftChildBytes32} \parallel \text{RightChildBytes32})$$

### 3.3 Top-Level Bundle Root
$$\text{RootHash} = \text{SHA256}(\text{"DONKAI:LPS1:ROOT:"} \parallel \text{BUNDLE\_TYPE} \parallel \text{":v1:"} \parallel \text{MerkleTreeRootBytes32})$$

---

## 4. Core Object Schemas

### 4.1 RemembranceStatement (`donkai.remembrance.v1`)
```json
{
  "type": "donkai.remembrance.v1",
  "language": "en-US",
  "authoringMode": "human-authored",
  "narrative": "I remember an arcade on Main Street...",
  "eventDate": {
    "start": "1978-06-01",
    "end": "1978-08-31",
    "certainty": "approximate"
  },
  "location": {
    "label": "Austin, Texas, United States",
    "precision": "city"
  },
  "culturalContext": ["arcade", "1977-era", "space-invaders"],
  "authorAttestation": "I confirm this is my own independent recollection."
}
```

### 4.2 ConsentManifest (`donkai.consent.v1`)
```json
{
  "type": "donkai.consent.v1",
  "visibility": "public",
  "identityMode": "pseudonymous",
  "allowAggregateResearch": true,
  "allowPublicExcerpt": true,
  "allowTranslation": true,
  "allowIndependentCorroboration": true,
  "retentionPolicy": "until-withdrawn",
  "sensitiveContentFlags": []
}
```

### 4.3 EvidenceBundle (`donkai.evidence-bundle.v1`)
```json
{
  "type": "donkai.evidence-bundle.v1",
  "memoryId": "MEM-1977-0001",
  "items": [
    {
      "contentHash": "sha256:95046c808e6205ce4fb6798ed74e1e8e14397861ef47eec630f03ce0f19999c2",
      "storageRef": "ipfs://bafkreievarwibdtcaxhe7ntzr3lu4huocq4xqyppi7xmmmhqhtqpdgmzyi",
      "sourceClass": "author-provided",
      "evidenceTier": "contemporaneous-artifact",
      "claimedCreatedAt": "1978-07-12",
      "access": "public",
      "aiDisclosure": "none",
      "description": "Scanned brass arcade token with grooved edge."
    }
  ]
}
```

---

## 5. Merkle Proof Specification

Inclusion proofs are represented as:
```json
{
  "leaf_index": 2,
  "leaf_hash": "0x95046c808e6205ce4fb6798ed74e1e8e14397861ef47eec630f03ce0f19999c2",
  "siblings": [
    { "hash": "0xa1b2...", "is_left": false },
    { "hash": "0xc3d4...", "is_left": true }
  ],
  "root": "0xb081e19cbc9e655067a5cb7253fddbc37cca5e81f01c72dc615572b0a4d7d43e"
}
```
Verification proceeds from leaf to root by hashing siblings according to their `is_left` positional flag.
