# DONK AI Evidence Standard (v0.1)

**Standard Code:** `EVID-STD-0.1`  
**Purpose:** Defining artifact integrity, provenance classes, evidence tiers, and digital custody requirements.

---

## 1. Evidence Tiers

1. **Contemporaneous Artifact (Tier 1):** Physical items, documents, film negatives, audio recordings, or broadcasts created during the historical period in question.
2. **Subsequent Documentation (Tier 2):** Periodicals, retrospective interviews, court filings, or municipal records created within 10 years of the event.
3. **Recollection Sketch / Oral Testimony (Tier 3):** Contemporary witness interviews, drawings, or audio recordings describing past events.
4. **Derivative Analysis (Tier 4):** Modern academic research papers, journalistic investigations, or forensic analyses.
5. **Unverified Lead (Tier 5):** Unattributed photographs, forum discussions, or unverified secondary leads.

---

## 2. Source Classification

- `author-provided`: Uploaded directly by primary witness.
- `third-party-witness`: Provided by a secondary observer.
- `institutional-archive`: Provided by an accredited library, museum, or government repository.
- `public-record`: Sourced from open government or municipal gazettes.
- `media-broadcast`: Off-air archival television, radio, or newsreel recording.

---

## 3. Cryptographic Custody Requirements

- Every artifact file must be hashed via SHA-256 and content-addressed via IPFS CIDv1.
- AI restoration, colorization, denoising, or OCR must be explicitly disclosed under `AiDisclosure`.
