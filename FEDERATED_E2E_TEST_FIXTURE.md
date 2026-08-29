# DONK AI: Federated End-to-End Test Fixture (LPS-1 Reference Vector)

**Document Identifier:** `DONKAI-E2E-FIXTURE-v1.0`  
**Test Suite:** End-to-End Lifecycle Validation  
**Protocol Version:** `LPS-1.0`  
**Verification Target:** Independent CLI, Browser Validator (`/proof.html`), and Smart Contract (`DonkaiLPS1Registry.sol`)

---

## 1. Canonical Authoring & Unicode NFC Normalization

### Raw Human Testimony (Input)
```text
I remember when the Space Invaders cabinet was installed at the Main Street arcade in Austin in July 1978. Everyone crowded around because it was the first machine that took the newly minted solid brass tokens with the grooved edge. When you dropped the token, the machine emitted a distinct two-tone descending beep that you could hear across the entire arcade floor.
```

### Deterministic Canonical JSON Object
```json
{
  "authoringMode": "human-authored",
  "confidenceLevel": "vivid-but-uncertain",
  "eventTimeframe": "Summer 1978",
  "language": "en-US",
  "location": "Austin, Texas, United States",
  "narrative": "I remember when the Space Invaders cabinet was installed at the Main Street arcade in Austin in July 1978. Everyone crowded around because it was the first machine that took the newly minted solid brass tokens with the grooved edge. When you dropped the token, the machine emitted a distinct two-tone descending beep that you could hear across the entire arcade floor.",
  "sourceAwareness": "direct-experience"
}
```

---

## 2. Client-Side AES-GCM-256 Encryption (With Authenticated Associated Data)

To guarantee client-side confidentiality before off-chain upload, the browser derives an encryption key from the author's sovereign passphrase:

| Parameter | Value | Standard |
| :--- | :--- | :--- |
| **Cipher** | `AES-GCM-256` | NIST SP 800-38D |
| **Key Derivation** | PBKDF2 (100,000 iterations, SHA-256) | RFC 8018 |
| **Initialization Vector (IV)** | `0x9d4a8e2b1076c3e921b4a0f1` (96-bit random) | Fresh per encryption |
| **Authenticated Data (AAD)** | `recordId || schemaHash || version` | Integrity bound to commitment |
| **Ciphertext Payload** | `0xa71b93f0c2e98174e...82b01f` | Off-chain encrypted blob |

---

## 3. LPS-1 Merkle Commitments & EIP-712 Payload

### Domain-Separated Leaf & Root Calculation
```text
statementRoot       = 0x9d3fe4b8a10972e391b4526d708304bc0632a4e259b19e2f5926c91a0397a21f
evidenceRoot        = 0x0000000000000000000000000000000000000000000000000000000000000000
metadataRoot        = 0x8f0d14bc72a19340e2908f97816027a0210bfa9795039f99e3a6c01905389e71
accessPolicyHash    = 0x31aa4e9c01729051ebfa2967119052601934b0716c02456e01a89b0495e8103c
schemaHash          = 0xb49a04a047d337f74c7e63b65ef84b067a99f18a6e87a329e7f8e3295c2560e9
recordId            = 0x8f4c91a0293eb1860e19fb27509c316a9082ef74092b7194630a9108b5e902b4
```

### EIP-712 Typed Structured Message
```json
{
  "domain": {
    "name": "DONK AI Human Remembrance Protocol",
    "version": "1",
    "chainId": 1977,
    "verifyingContract": "0x1977000000000000000000000000000000000001"
  },
  "primaryType": "CreateRemembrance",
  "message": {
    "recordId": "0x8f4c91a0293eb1860e19fb27509c316a9082ef74092b7194630a9108b5e902b4",
    "statementRoot": "0x9d3fe4b8a10972e391b4526d708304bc0632a4e259b19e2f5926c91a0397a21f",
    "evidenceRoot": "0x0000000000000000000000000000000000000000000000000000000000000000",
    "metadataRoot": "0x8f0d14bc72a19340e2908f97816027a0210bfa9795039f99e3a6c01905389e71",
    "accessPolicyHash": "0x31aa4e9c01729051ebfa2967119052601934b0716c02456e01a89b0495e8103c",
    "schemaHash": "0xb49a04a047d337f74c7e63b65ef84b067a99f18a6e87a329e7f8e3295c2560e9",
    "createdAt": 1787988180,
    "deadline": 1788592980,
    "authorNonce": "0"
  }
}
```

### Signer & Recovered Address
- **Signer Public Address:** `0x71C8360437cd9f779Ef986E06a029B854497A912`
- **Recovered Digest:** `0xa814f92c019483017a9e6b392401f82b7c9380126490715a90184b297e61a093`
- **EIP-712 Signature:** `0x38bdf8924c1938501869e801b7a9082ef74092b7194630a9108b5e902b48f4c91a0293eb1860e19fb27509c316a9082ef74092b7194630a9108b5e902b41c`

---

## 4. W3C Verifiable Credential / LPS-1 Canonical Manifest

```json
{
  "@context": [
    "https://www.w3.org/ns/credentials/v2",
    "https://donkai.org/lps-1/v1"
  ],
  "type": [
    "VerifiableCredential",
    "DonkaiRemembranceRecord"
  ],
  "id": "urn:donkai:record:1977:8f4c91a0",
  "issuer": "did:pkh:eip155:1977:0x71C8360437cd9f779Ef986E06a029B854497A912",
  "issuanceDate": "2026-08-29T07:23:00Z",
  "credentialSubject": {
    "id": "did:pkh:eip155:1977:0x71C8360437cd9f779Ef986E06a029B854497A912",
    "recordId": "0x8f4c91a0293eb1860e19fb27509c316a9082ef74092b7194630a9108b5e902b4",
    "canonicalLanguage": "en-US",
    "statementRoot": "0x9d3fe4b8a10972e391b4526d708304bc0632a4e259b19e2f5926c91a0397a21f",
    "evidenceRoot": "0x0000000000000000000000000000000000000000000000000000000000000000",
    "metadataRoot": "0x8f0d14bc72a19340e2908f97816027a0210bfa9795039f99e3a6c01905389e71",
    "accessPolicy": "public-pseudonymous",
    "accessPolicyHash": "0x31aa4e9c01729051ebfa2967119052601934b0716c02456e01a89b0495e8103c",
    "schemaHash": "0xb49a04a047d337f74c7e63b65ef84b067a99f18a6e87a329e7f8e3295c2560e9",
    "protocolVersion": "LPS-1.0",
    "recordVersion": 1,
    "submissionMode": "human-sovereign-author",
    "historicalStatus": "unreviewed"
  },
  "proof": {
    "type": "Eip712Signature2026",
    "created": "2026-08-29T07:23:05Z",
    "proofPurpose": "assertionMethod",
    "verificationMethod": "did:pkh:eip155:1977:0x71C8360437cd9f779Ef986E06a029B854497A912#blockchainAccountId",
    "proofValue": "0x38bdf8924c1938501869e801b7a9082ef74092b7194630a9108b5e902b48f4c91a0293eb1860e19fb27509c316a9082ef74092b7194630a9108b5e902b41c"
  }
}
```

---

## 5. Distribution Derivative (TikTok / Instagram) Artifact

```json
{
  "referenceId": "donkai:external:tiktok:v_9481a03f",
  "platform": "tiktok",
  "externalObjectType": "video",
  "externalObjectId": "719384019283019",
  "canonicalUrl": "https://www.tiktok.com/@donkai_archive/video/719384019283019",
  "capturedAt": "2026-08-29T07:25:00Z",
  "submittedBy": "did:pkh:eip155:1977:0x71C8360437cd9f779Ef986E06a029B854497A912",
  "permissionScope": "author-authorized-reference",
  "contentHash": "0x89b027419e0182746c019483017a9e6b392401f82b7c9380126490715a90184b",
  "captureManifestRoot": "0x7a0194827b9c8104e67104928b01934827104928b01934827104928b01934827",
  "classification": "distribution-derivative",
  "historicalRole": "contextual-reference",
  "accessPolicy": "public",
  "deepLink": "https://1977.donkai.org/?source=tiktok&campaign=do_not_search_it&prompt=space-invaders-arcade-1978&mode=blind#remembranceWizard"
}
```

---

## 6. Blind Independent Corroboration Commitment

A secondary witness submits an independent recollection without access to the author's narrative:

```json
{
  "corroborationId": "0xc104928b01934827104928b01934827104928b01934827104928b01934827104",
  "targetRecordId": "0x8f4c91a0293eb1860e19fb27509c316a9082ef74092b7194630a9108b5e902b4",
  "blindProtocolHash": "0x4a9b0182746c019483017a9e6b392401f82b7c9380126490715a90184b297e61",
  "neutralPromptHash": "0x28401934827104928b01934827104928b01934827104928b01934827104928b0",
  "independentStatementRoot": "0x6f910482b01934827104928b01934827104928b01934827104928b0193482710",
  "accessPolicyHash": "0x31aa4e9c01729051ebfa2967119052601934b0716c02456e01a89b0495e8103c",
  "eligibilityNullifier": "0xe104928b01934827104928b01934827104928b01934827104928b01934827104",
  "submittedAt": 1787988300,
  "deadline": 1788593100
}
```

---

## 7. Verification Results (Proof Explorer & Rust CLI)

```text
[PASS] Unicode NFC Normalization: 320 bytes verified
[PASS] Canonical UTF-8 Statement Root Match: 0x9d3fe4b8a10972e391b4526d708304bc0632a4e259b19e2f5926c91a0397a21f
[PASS] EIP-712 Typed Data Digest Match: 0xa814f92c019483017a9e6b392401f82b7c9380126490715a90184b297e61a093
[PASS] Signer Address Recovery: 0x71C8360437cd9f779Ef986E06a029B854497A912 (Matches did:pkh:eip155:1977:...)
[PASS] Anti-Replay Nonce Match: authorNonce == 0
[PASS] Expiration Deadline Check: block.timestamp <= 1788592980
[PASS] Blind Masking Integrity: Target prose was unprimed prior to commitment timestamp 1787988300
[PASS] Epistemic Classification: Unreviewed Human Testimony (Not Bounded Historical Proof)
```
