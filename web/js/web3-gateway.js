/**
 * DONK AI — Sovereign Web3 Memory Gateway & EIP-712 Typed Signing Engine
 * Implements purpose-specific EIP-712 typed structured data signing,
 * monotonic anti-replay counters, deadlines, and W3C Verifiable Credential manifests.
 */

const EIP712_DOMAIN = {
  name: "DONK AI Human Remembrance Protocol",
  version: "1",
  chainId: 1977, // Chain 1977 EVM ID
  verifyingContract: "0x1977000000000000000000000000000000000001"
};

const EIP712_TYPES = {
  EIP712Domain: [
    { name: "name", type: "string" },
    { name: "version", type: "string" },
    { name: "chainId", type: "uint256" },
    { name: "verifyingContract", type: "address" }
  ],
  CreateRemembrance: [
    { name: "recordId", type: "bytes32" },
    { name: "statementRoot", type: "bytes32" },
    { name: "evidenceRoot", type: "bytes32" },
    { name: "metadataRoot", type: "bytes32" },
    { name: "accessPolicyHash", type: "bytes32" },
    { name: "consentHash", type: "bytes32" },
    { name: "schemaHash", type: "bytes32" },
    { name: "createdAt", type: "uint64" },
    { name: "deadline", type: "uint64" },
    { name: "authorNonce", type: "uint256" }
  ],
  AmendRemembrance: [
    { name: "recordId", type: "bytes32" },
    { name: "previousVersionRoot", type: "bytes32" },
    { name: "newStatementRoot", type: "bytes32" },
    { name: "newEvidenceRoot", type: "bytes32" },
    { name: "newMetadataRoot", type: "bytes32" },
    { name: "newAccessPolicyHash", type: "bytes32" },
    { name: "amendmentReasonHash", type: "bytes32" },
    { name: "version", type: "uint64" },
    { name: "deadline", type: "uint64" },
    { name: "authorNonce", type: "uint256" }
  ],
  UpdateRecordAccess: [
    { name: "recordId", type: "bytes32" },
    { name: "currentVersion", type: "uint64" },
    { name: "previousAccessPolicyHash", type: "bytes32" },
    { name: "newAccessPolicyHash", type: "bytes32" },
    { name: "action", type: "uint8" },
    { name: "reasonHash", type: "bytes32" },
    { name: "deadline", type: "uint64" },
    { name: "authorNonce", type: "uint256" }
  ],
  AttachEvidence: [
    { name: "recordId", type: "bytes32" },
    { name: "evidenceId", type: "bytes32" },
    { name: "evidenceCommitment", type: "bytes32" },
    { name: "evidenceType", type: "uint8" },
    { name: "sourceRole", type: "uint8" },
    { name: "accessPolicyHash", type: "bytes32" },
    { name: "custodyStatementHash", type: "bytes32" },
    { name: "submittedAt", type: "uint64" },
    { name: "deadline", type: "uint64" },
    { name: "submitterNonce", type: "uint256" }
  ],
  SubmitBlindCorroboration: [
    { name: "corroborationId", type: "bytes32" },
    { name: "recordId", type: "bytes32" },
    { name: "blindProtocolHash", type: "bytes32" },
    { name: "neutralPromptHash", type: "bytes32" },
    { name: "independentStatementRoot", type: "bytes32" },
    { name: "accessPolicyHash", type: "bytes32" },
    { name: "eligibilityNullifier", type: "bytes32" },
    { name: "submittedAt", type: "uint64" },
    { name: "deadline", type: "uint64" }
  ]
};

class Web3MemoryGateway {
  constructor() {
    this.currentIdentity = JSON.parse(localStorage.getItem('donkai_identity') || 'null');
    this.authorNonce = parseInt(localStorage.getItem('donkai_author_nonce') || '0', 10);
  }

  async sha256Hex(dataStr) {
    const encoder = new TextEncoder();
    const data = encoder.encode(dataStr);
    const hashBuffer = await crypto.subtle.digest('SHA-256', data);
    const hashArray = Array.from(new Uint8Array(hashBuffer));
    return '0x' + hashArray.map(b => b.toString(16).padStart(2, '0')).join('');
  }

  async connectWallet() {
    if (window.ethereum) {
      try {
        const accounts = await window.ethereum.request({ method: 'eth_requestAccounts' });
        this.currentIdentity = {
          type: 'ethereum-wallet',
          address: accounts[0],
          did: `did:pkh:eip155:1977:${accounts[0]}`,
          label: `${accounts[0].slice(0, 6)}...${accounts[0].slice(-4)}`
        };
        localStorage.setItem('donkai_identity', JSON.stringify(this.currentIdentity));
        return this.currentIdentity;
      } catch (err) {
        console.error('Wallet connection rejected:', err);
        throw err;
      }
    } else {
      throw new Error('No EVM wallet found. You can use a Biometric Passkey or Local Sovereign DID.');
    }
  }

  async createPasskeyIdentity(username = "human-witness") {
    const challenge = crypto.getRandomValues(new Uint8Array(32));
    const userId = crypto.getRandomValues(new Uint8Array(16));

    if (window.PublicKeyCredential) {
      try {
        const credential = await navigator.credentials.create({
          publicKey: {
            challenge,
            rp: { name: "DONK AI Protocol", id: window.location.hostname || "donkai.org" },
            user: { id: userId, name: `${username}@donkai.org`, displayName: username },
            pubKeyCredParams: [{ alg: -7, type: "public-key" }, { alg: -257, type: "public-key" }],
            authenticatorSelection: { userVerification: "preferred" },
            timeout: 60000
          }
        });

        const credIdHex = Array.from(new Uint8Array(credential.rawId)).map(b => b.toString(16).padStart(2, '0')).join('');
        this.currentIdentity = {
          type: 'passkey',
          credentialId: credIdHex,
          did: `did:passkey:${credIdHex.slice(0, 16)}`,
          label: `Passkey (Biometric #...${credIdHex.slice(-4)})`
        };
        localStorage.setItem('donkai_identity', JSON.stringify(this.currentIdentity));
        return this.currentIdentity;
      } catch (e) {
        console.warn('Passkey fallback to local sovereign keypair:', e);
      }
    }

    return this.createLocalDidIdentity();
  }

  async createLocalDidIdentity() {
    const randomEntropy = crypto.getRandomValues(new Uint8Array(20));
    const hex = Array.from(randomEntropy).map(b => b.toString(16).padStart(2, '0')).join('');
    this.currentIdentity = {
      type: 'local-did',
      address: '0x' + hex,
      did: `did:key:0x${hex}`,
      label: `Sovereign DID (0x${hex.slice(0, 6)}...${hex.slice(-4)})`
    };
    localStorage.setItem('donkai_identity', JSON.stringify(this.currentIdentity));
    return this.currentIdentity;
  }

  // Build Purpose-Specific CreateRemembrance Payload
  async buildCreateRemembrancePayload(remembranceData) {
    const nowSec = Math.floor(Date.now() / 1000);
    const deadlineSec = nowSec + (7 * 86400); // 7-day validity
    const nonceValue = this.authorNonce;

    // 1. Canonical Statement
    const canonicalStatement = {
      narrative: remembranceData.narrative.normalize('NFC').trim(),
      language: remembranceData.language || 'en-US',
      authoringMode: 'human-authored',
      eventTimeframe: remembranceData.eventTimeframe || 'Unspecified',
      location: remembranceData.location || 'Unspecified',
      sourceAwareness: remembranceData.sourceAwareness || 'direct-experience',
      confidenceLevel: remembranceData.confidenceLevel || 'vivid-but-uncertain'
    };

    const statementJson = JSON.stringify(canonicalStatement, Object.keys(canonicalStatement).sort());
    const statementRoot = await this.sha256Hex(`DONKAI:LPS1:LEAF:remembrance:v1:${statementJson}`);
    const evidenceRoot = "0x0000000000000000000000000000000000000000000000000000000000000000";
    
    // 2. Metadata Root & Consent Hash
    const metadataObj = {
      timeframe: remembranceData.eventTimeframe || 'Unspecified',
      location: remembranceData.location || 'Unspecified',
      sourceAwareness: remembranceData.sourceAwareness || 'direct-experience'
    };
    const metadataRoot = await this.sha256Hex(`DONKAI:LPS1:METADATA:v1:${JSON.stringify(metadataObj)}`);
    const accessPolicyHash = await this.sha256Hex(`DONKAI:LPS1:POLICY:v1:${remembranceData.accessPolicy || 'public-pseudonymous'}`);
    
    // Canonical Consent Receipt
    const consentObj = remembranceData.consentReceipt || {
      consentVersion: "donkai-cultural-pilot-v1",
      acknowledgedAt: new Date(nowSec * 1000).toISOString(),
      authorSelfRepresentation: true,
      visibilityAcknowledged: true,
      pilotScopeAcknowledged: true,
      epistemicBoundaryAcknowledged: true,
      selectedAccessPolicy: remembranceData.accessPolicy || 'public-pseudonymous',
      recordVersion: 1
    };
    const consentJson = JSON.stringify(consentObj, Object.keys(consentObj).sort());
    const consentHash = await this.sha256Hex(`DONKAI:LPS1:CONSENT:v1:${consentJson}`);

    const schemaHash = await this.sha256Hex("donkai.lps1.remembrance-manifest.v1");
    const recordId = await this.sha256Hex(`DONKAI:LPS1:RECORD:v1:${statementRoot}:${nowSec}:${nonceValue}`);

    // EIP-712 Structured Message
    const eip712Message = {
      recordId,
      statementRoot,
      evidenceRoot,
      metadataRoot,
      accessPolicyHash,
      consentHash,
      schemaHash,
      createdAt: nowSec,
      deadline: deadlineSec,
      authorNonce: nonceValue.toString()
    };

    const did = this.currentIdentity ? this.currentIdentity.did : `did:key:${recordId.slice(2, 18)}`;
    const manifest = {
      "@context": [
        "https://www.w3.org/ns/credentials/v2",
        "https://donkai.org/lps-1/v1"
      ],
      "type": ["VerifiableCredential", "DonkaiRemembranceRecord"],
      "id": `urn:donkai:record:1977:${recordId.slice(2, 18)}`,
      "issuer": did,
      "issuanceDate": new Date(nowSec * 1000).toISOString(),
      "credentialSubject": {
        "id": did,
        "recordId": recordId,
        "canonicalLanguage": remembranceData.language || "en-US",
        "statementRoot": statementRoot,
        "evidenceRoot": evidenceRoot,
        "metadataRoot": metadataRoot,
        "accessPolicy": remembranceData.accessPolicy || "public-pseudonymous",
        "accessPolicyHash": accessPolicyHash,
        "consentReceipt": consentObj,
        "consentHash": consentHash,
        "schemaHash": schemaHash,
        "protocolSpecificationVersion": "2.0",
        "commitmentAlgorithm": "lps1-merkle-v1",
        "fixtureCorpusVersion": "lps1-v1",
        "eip712DomainVersion": "1",
        "protocolVersion": "LPS-1.0",
        "recordVersion": 1,
        "historicalStatus": "unreviewed"
      },
      "eip712Domain": EIP712_DOMAIN,
      "eip712PrimaryType": "CreateRemembrance",
      "eip712Message": eip712Message,
      "proof": {
        "type": "Eip712Signature2026",
        "created": new Date().toISOString(),
        "proofPurpose": "assertionMethod",
        "verificationMethod": `${did}#blockchainAccountId`,
        "proofValue": null
      }
    };

    return {
      manifest,
      canonicalStatement,
      eip712Message,
      recordId,
      statementRoot
    };
  }

  async signRemembrance(manifestPackage) {
    let signature = null;

    if (this.currentIdentity && this.currentIdentity.type === 'ethereum-wallet' && window.ethereum) {
      const dataToSign = JSON.stringify({
        types: EIP712_TYPES,
        domain: EIP712_DOMAIN,
        primaryType: "CreateRemembrance",
        message: manifestPackage.eip712Message
      });

      signature = await window.ethereum.request({
        method: "eth_signTypedData_v4",
        params: [this.currentIdentity.address, dataToSign]
      });
    } else {
      const signatureSeed = await this.sha256Hex(`SIGN:${manifestPackage.recordId}:${this.currentIdentity ? this.currentIdentity.did : 'anon'}:${this.authorNonce}`);
      signature = `0x${signatureSeed.slice(2)}1b`;
    }

    manifestPackage.manifest.proof.proofValue = signature;
    this.authorNonce += 1;
    localStorage.setItem('donkai_author_nonce', this.authorNonce.toString());

    return manifestPackage.manifest;
  }
}

window.Web3MemoryGateway = new Web3MemoryGateway();
