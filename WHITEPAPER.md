# Donkai Network — `donkai-core` v0.1.0

**Chain ID:** `1977` (devnet)
**Repository target:** `https://github.com/donkai-org/donkai-core`
**Status:** reference implementation. Not audited. Not FIPS-certified.

This document describes what the `donkai-core` workspace *actually implements today*. Anything not listed under **Shipped** is future work and is not to be quoted as a running feature.

---

## Shipped in v0.1.0

| Module | Crate | What it does |
|---|---|---|
| Provenance | `donkai-lps1` | Paragraph-split SHA-256 binary Merkle tree with true O(log n) inclusion proofs. 58 discrete named audit checks, each a real programmatic assertion. |
| Consensus | `donkai-consensus` | Proof-of-Stubbornness validator tiers + Asinine Fault Tolerance strict 2/3 supermajority (`agreeing * 3 >= total * 2`, u128 saturating). |
| Post-quantum | `donkai-pqc` | ML-DSA-87 (FIPS 204) signature + ML-KEM-1024 (FIPS 203) KEM wrappers over `pqcrypto-mldsa` / `pqcrypto-mlkem` C reference implementations. Sign/verify/encapsulate/decapsulate roundtrips tested. |
| Agent security | `donkai-policyguard` | Deterministic multi-signature adjudication. Verifies each ML-DSA signature against a whitelisted validator set. Rejects unknown signers, tampered signatures, duplicate signers, and below-quorum proposals. |
| Storage | `donkai-ipfs` | Native CIDv1 computation (raw codec `0x55` + dag-pb codec `0x70` wrapping single-block UnixFS File), base32-lowercase-no-pad multibase. Blocking Kubo RPC pin client targeting `http://127.0.0.1:5001/api/v0`. |
| Node | `donkai-node` | Reference binary wiring the five modules together into a boot console. |

## Design notes

### The 58-check audit manifest is real

The old `verified_checks_passed: 58` self-set field is gone. `run_all_58_checks(tree, document)` executes 58 discrete named assertions grouped as:

- **1–15** Structural (leaves present, indices contiguous, depth = ceil(log₂ n), root deterministic on rebuild, …)
- **16–30** Content (paragraph parsing parity, UTF-8, size bounds, serde roundtrip, …)
- **31–45** Anchor metadata (Polygon Chain ID 137, schema version, hex roundtrip, level cache invariants, …)
- **46–58** Proof & verification behavior (build → prove → verify roundtrip; tampered leaf, tampered sibling, flipped direction, odd-tail duplication, out-of-range index, and bounded execution time)

`Lps1AuditReport::passed_count` is a counted result, not a hardcoded number.

### AFT math is strict 2/3 BFT — comment matches code

`AsinineFaultTolerance::has_supermajority(total, agreeing)` returns true iff `agreeing * 3 >= total * 2`. The "asinine" name is satirical framing; the math is real BFT. Both sides use `saturating_mul` so pathological weights do not panic.

### PolicyGuard actually verifies signatures

`PolicyGuardEvaluator::evaluate(proposal, validator_set, base_quorum)`:

- D0/D1 → no signatures required.
- D2/D3 → `base_quorum` valid signatures required.
- D4/D5 → `max(2, base_quorum)` valid signatures required.

Each `SignedApproval` is checked against the whitelisted validator set (`RejectedUnknownSigner`), deduplicated (`RejectedDuplicateSigner`), and cryptographically verified via ML-DSA-87 (`RejectedInvalidSignature`). Array length alone is not the gate.

### IPFS CIDs are computed, not fabricated

`compute_raw_cidv1(bytes)` → `bafkrei…` string.
`compute_dagpb_cidv1(bytes)` → `bafybei…` string, wrapping the payload as a single-block UnixFS File PBNode.
`pin_to_kubo_at(api_base, bytes)` → posts to Kubo `/add?cid-version=1&pin=true` and returns the daemon-reported CID.

Placeholder strings like `QmDonkaiL1QuantumResilientMesh2026` are not used anywhere in the codebase.

## Not shipped

The following appear in earlier design drafts but are **not** in the v0.1.0 code and should not be marketed as live features:

- Bitcoin OpenTimestamps anchoring (only Polygon Chain ID 137 metadata is present on the manifest; OTS submission is roadmap).
- SLH-DSA (FIPS 205) signatures.
- ERC-3643 / T-REX Solidity contracts.
- `$DONK-USD` stablecoin mint contracts.
- Two-museum IPFS-pinned lore archive (collectibles + format wars) — data only, no engine yet.
- Full Tendermint-like BFT gossip network. Consensus math is present; the p2p layer is not.

## Verification

```bash
cd dev/blockchain/donkai-core
cargo test --workspace
cargo run
```
