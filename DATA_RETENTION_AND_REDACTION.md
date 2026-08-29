# DONK AI Data Retention, Redaction & Retraction Policy

**Policy Code:** `RET-RED-1.0`  
**Purpose:** Managing the lifecycle of on-chain commitments and off-chain payloads when legal, privacy, or ethical updates occur.

---

## 1. Immutability vs The Right to Be Forgotten

While on-chain Merkle root commitments cannot be deleted from historical block state:
1. **Tombstone Records:** An author can submit a cryptographically signed `retractRecord` transaction to `DonkaiMemoryLedger.sol`, transitioning the record status to `Retracted`.
2. **Payload Purging:** Off-chain encrypted payloads on IPFS or private storage nodes are purged upon verified retraction receipt.
3. **Redaction Nodes:** If specific details (e.g. minor names, private addresses) must be redacted, an amended record is linked via `VersionGraph` (`amendmentRelation = "redaction"`), preserving historical integrity while preventing display of prohibited data.

---

## 2. Retention Defaults

- `until-withdrawn`: Retained indefinitely until author issues a retraction.
- `time-bounded`: Expired and unpinned from IPFS after a declared expiration date.
- `provisional`: Staged for peer review; purged if uncorroborated after 1 year.
