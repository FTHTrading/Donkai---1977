# Living Provenance Standard 1 (LPS-1 v2.0): Complete Technical Specification

**Document Identifier:** `LPS-1-SPEC-v2.0`  
**Governing Standard:** Antigravity Human Remembrance Protocol  
**Revision:** `2.0.0-final`

---

## 1. Canonicalization & Unicode Invariants

1. **Encoding:** UTF-8 throughout.
2. **Unicode Normalization:** All string values recursively undergo **Unicode Normalization Form C (`NFC`)**.
3. **Line-Ending Normalization:** All Carriage Return (`CRLF` / `\r\n` or `CR` / `\r`) sequences normalize to Line Feed (`LF` / `\n`).
4. **JSON Key Ordering:** Object keys are sorted recursively in ascending lexicographical order (ASCII byte comparison).
5. **Whitespace Handling:** Insignificant whitespace is stripped; values are rendered compactly with `:` separating keys/values and `,` separating entries.

---

## 2. Merkle Tree & Commitment Invariants

### 2.1 Cryptographic Constants
```text
EMPTY_EVIDENCE_ROOT  = 0x0000000000000000000000000000000000000000000000000000000000000000
EMPTY_LEAF_CONSTANT  = 0x0b96f989296d0d7f9adcbad65a1161244e359831749a8564280854ac27202d22
                      (Derivation: SHA-256("DONKAI:LPS1:EMPTY_MERKLE_LEAF:v1"))
```

### 2.2 Leaf Hashing (Domain-Separated)
For any object type $T \in \{\text{"REMEMBRANCE"}, \text{"EVIDENCE"}, \text{"REVIEW"}, \text{"CORROBORATION"}\}$:
$$\text{leafHash} = \text{SHA-256}(\text{"DONKAI:LPS1:LEAF:"} \parallel T \parallel \text{":v1:"} \parallel \text{canonicalUtf8Bytes})$$

### 2.3 Internal Node Hashing & Odd-Leaf Balancing (Option B)
For left child $L$ and right child $R$:
$$\text{parentHash} = \text{SHA-256}(\text{"DONKAI:LPS1:NODE:v1:"} \parallel L \parallel R)$$

**Odd-Leaf Balancing Rule (Option B):**  
If layer $k$ has an odd number of nodes $2m + 1$, the final node $N_{2m}$ is paired positionally with `EMPTY_LEAF_CONSTANT`:
$$\text{parent}_{m} = \text{SHA-256}(\text{"DONKAI:LPS1:NODE:v1:"} \parallel N_{2m} \parallel \text{EMPTY\_LEAF\_CONSTANT})$$

### 2.4 Evidence Sorting
Evidence items are sorted ascending by unsigned big-endian 32-byte arrays of `(evidence_id, commitment)`. Duplicate pairs are rejected.

---

## 3. EIP-712 Structured Intent & Replay Protection

### 3.1 Domain Separator
```solidity
keccak256(
    abi.encode(
        keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)"),
        keccak256(bytes("DONKAI AI Human Remembrance Protocol")),
        keccak256(bytes("1")),
        chainId,
        verifyingContract
    )
);
```

### 3.2 CreateRemembrance Typehash & Struct Hashing
```solidity
bytes32 public constant CREATE_REMEMBRANCE_TYPEHASH = keccak256(
    "CreateRemembrance("
    "bytes32 recordId,"
    "bytes32 statementRoot,"
    "bytes32 evidenceRoot,"
    "bytes32 metadataRoot,"
    "bytes32 accessPolicyHash,"
    "bytes32 schemaHash,"
    "uint64 createdAt,"
    "uint64 deadline,"
    "uint256 authorNonce)"
);
```

---

## 4. Privacy & Authenticated Associated Data (AAD)

Private remembrance narratives are encrypted client-side using `AES-GCM-256`:
- **Key:** Derived client-side via PBKDF2 (100,000 rounds, SHA-256) or sovereign WebAuthn PRF.
- **IV:** Fresh, unpredictable 96-bit random IV per encryption.
- **AAD:** Explicitly bound to `recordId || schemaHash || recordVersion || accessPolicyHash`.
- **Integrity Rule:** Altered ciphertext, corrupted tag, mismatched AAD, or wrong key fails closed immediately before exposing plaintext.

---

## 5. Epistemic Terminology Dictionary

| Term | What It Establishes | What It Does NOT Claim |
| :--- | :--- | :--- |
| **`LPS-1 Conformant`** | Conforms to published canonicalization, leaf hashing, and Merkle balancing rules. | Does not validate historical accuracy. |
| **`EIP-712 Compatible`** | Generates or verifies typed structured authorization digests. | Does not imply automatic on-chain finality until anchored. |
| **`Signed Record`** | Authorized by a specific EVM account or WebAuthn credential. | Does not verify the author's physical identity or truthfulness. |
| **`Portable Manifest`** | Contains roots, schemas, and signatures for independent validation. | Does not guarantee eternal decentralized storage availability. |
| **`On-Chain Anchored`** | A real transaction on a verified contract recorded the commitment. | Does not mean the blockchain validated the narrative. |
| **`Historically Supported`** | Independent source evidence and corroboration met documented review criteria. | Does not make historical knowledge immutable or absolute. |
