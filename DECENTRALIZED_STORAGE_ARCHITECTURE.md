# DONK AI: Decentralized Storage & Payload Routing Architecture

**Document Identifier:** `STORAGE-ARCH-v1.0`  
**Status:** In Development  
**Scope:** Multi-Tiered Content Addressed Storage, Client-Side Encryption Routing, IPFS CIDv1, and Permanent Archival Deals.

---

## 1. Storage Tiering Topology

```mermaid
flowchart TD
    subgraph Client [Client-Side Browser Studio]
        P[Plaintext Testimony / High-Res Artifact] --> E{Visibility Policy?}
        E -->|Public| C1[Canonical JSON / Raw CIDv1]
        E -->|Restricted| K[Generate Random AES-GCM-256 Key + IV]
        K --> ENC[Client-Side Encrypted Blob]
        ENC --> C2[Encrypted CIDv1]
    end

    subgraph Transport [Decentralized Storage Layer]
        C1 --> IPFS[Kubo IPFS Pinning Nodes]
        C2 --> IPFS
        IPFS --> FIL[Filecoin Long-Term Storage Deals]
        IPFS --> AR[Arweave Permanent Archival Grid]
    end

    subgraph Ledger [EVM State Layer]
        C1 --> MERKLE[LPS-1 Merkle Commitment Root]
        C2 --> MERKLE
        MERKLE --> CONTRACT[DonkaiMemoryLedger.sol]
    end
```

---

## 2. Key Distribution & Epoch Management

For `reviewer-only` and `trusted-circle` visibility modes, symmetric AES keys are encrypted under authorized public keys using ECIES (Elliptic Curve Integrated Encryption Scheme) or WebAuthn public key sets.

When access is revoked or an author amends their consent manifest:
1. A new **Key Epoch** is initiated.
2. Unrevoked recipients receive re-wrapped keys.
3. The previous epoch is tombstoned on `DonkaiPrivacyRegistry.sol`.
