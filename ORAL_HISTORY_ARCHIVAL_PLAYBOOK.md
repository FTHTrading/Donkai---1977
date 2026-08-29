# DONK AI: Oral History & Field Archival Playbook

**Document Identifier:** `ORAL-ARCH-v1.0`  
**Status:** Best Practice Standard for Community Archivists and Field Researchers

---

## 1. Field Recording Standards

When collecting first-person oral remembrances for LPS-1 anchoring:
1. **Audio Format:** Uncompressed 24-bit / 96kHz PCM WAV master recording.
2. **Environmental Baseline:** 30 seconds of ambient room tone recorded prior to interview commencement.
3. **Hardware Provenance:** Capture microphone make/model, serial number, and recorder timestamp in `ContextManifest`.

---

## 2. Ingestion & Provenance Workflow

```mermaid
sequenceDiagram
    autonumber
    actor Witness as Human Witness
    actor Interviewer as Field Archivist
    participant Studio as DONK AI Studio
    participant IPFS as Encrypted IPFS
    participant Ledger as DonkaiMemoryLedger.sol

    Interviewer->>Witness: Conducts structured, non-leading interview
    Interviewer->>Studio: Ingests raw audio master + transcription
    Studio->>Witness: Signs human-authorship & consent manifest
    Studio->>IPFS: Stores content-addressed audio (CIDv1)
    Studio->>Ledger: Commits LPS-1 Statement & Evidence Roots
    Ledger-->>Interviewer: Anchors timestamped Provenance Receipt
```

---

## 3. Retraction & Empathy Protocol

Archivists must inform witnesses of their permanent right to retract or restrict their oral testimony. If requested, a tombstone event is signed, and media payloads are unpinned immediately.
