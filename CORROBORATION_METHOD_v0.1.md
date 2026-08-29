# DONK AI Corroboration Method (v0.1)

**Standard:** `CORROB-MTH-0.1`  
**Core Goal:** Eliminating Bandwagon Bias in Witness Recalls

---

## 1. The Blind Commit-Reveal Sequence

To ensure authentic independent recollection:

```
[Discovery Context Only]
         │ (Timeframe, Broad Location, Topic Category)
         ▼
[Witness Authors Recall]
         │ (Local Device Only)
         ▼
[Compute Sealed Root] ──► SHA256( Prefix || Salt || Narrative )
         │
         ▼
[On-Chain Anchor] ─────► DonkaiBlindCorroboration.sol (Commit Phase)
         │
         ▼ (Review Window / Unlock Event)
[Reveal Phase] ────────► Submits Plain Narrative + Salt
         │
         ▼
[Peer Comparison] ─────► Human Reviewers Grade Overlap Under Versioned Rubric
```

---

## 2. Prohibition of AI Judgment

No neural network or large language model may independently mark two accounts as "corroborated" or "true." Clustering models may assist reviewers in highlighting semantic themes, but the final assessment requires human review.
