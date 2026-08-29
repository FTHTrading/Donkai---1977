# DONK AI: Federated Social Memory Gateway Specification (LPS-1 Hub & Spoke)

**Document Identifier:** `FEDERATED-SOCIAL-ADAPTER-v1.0`  
**Status:** Canonical Architecture Specification  
**Governing Principle:** *Social platforms distribute and capture memory signals; DONK AI preserves the canonical, consent-controlled, portable record.*

---

## 1. Hub-and-Spoke Architecture

```
┌────────────────────────────────────────────────────────┐
│                   DONK AI LPS-1 CORE                   │
│  • Canonical memory record (Original Language NFC)      │
│  • Consent & Client-Side AES-GCM-256 Encryption        │
│  • EIP-712 Typed Signatures & Sovereign DIDs           │
│  • Evidence Manifests & Proof Trees                    │
│  • Blind Corroboration Engine                          │
│  • Append-Only Review & Lineage Graph                  │
└───────────────────────────┬────────────────────────────┘
                            │
   ┌────────────────────────┼────────────────────────┬────────────────────────┐
   │                        │                        │                        │
┌──▼────────────────┐   ┌───▼────────────────┐   ┌───▼────────────────┐   ┌───▼────────────────┐
│  TikTok Memory    │   │  Instagram Cards   │   │  YouTube Oral      │   │  Open Social       │
│  Lens (15–60s)    │   │  & Artifact Visual │   │  History Studio    │   │  Farcaster / Lens  │
│  • Free Voice/Text│   │  • Carousel Cards  │   │  • Long-form Video │   │  • Base On-Chain   │
│  • Sound Recall   │   │  • Reels & Stories │   │  • Time-coded Audio│   │  • Bluesky / AT    │
│  • Blind Dilemmas │   │  • Museum Displays │   │  • Transcript Hash │   │  • Social Cards    │
└──┬────────────────┘   └───┬────────────────┘   └───┬────────────────┘   └───┬────────────────┘
   │                        │                        │                        │
   └────────────────────────┴─────────┬──────────────┴────────────────────────┘
                                      │
                         [LPS-1 External Reference]
```

---

## 2. Connected Platform Matrix

| Connected System | Native Interaction | Captured Signal | DONK AI Protocol Action | Release Phase |
| :--- | :--- | :--- | :--- | :--- |
| **TikTok Memory Lens** | Short vertical recall challenge (15–60s) | Spoken/typed free recall, sound recognition, dilemma choices | Creates draft remembrance or blind corroboration commitment | **Phase 1** |
| **Instagram Memory Cards** | Reels, Stories, carousels, DM cards | Visual memory comparison, artifact inspection | Generates shareable derivative card linked to canonical record | **Phase 1** |
| **Farcaster / Base Social** | Wallet-native casts, open frames, attestations | Wallet-linked contributions, curator discovery | Native EVM signatures, non-transferable contributor attestations | **Phase 1** |
| **YouTube Oral History** | Long-form video, Shorts, caption tracks | Rich first-person testimony, physical artifact demos | Anchors video/transcript hash manifest to LPS-1 record | **Phase 2** |
| **Discord / Telegram** | Small-group witness rooms, voice stages | Multi-witness oral histories, reviewer sync | Creates encrypted session manifests and individual signed records | **Phase 2** |
| **Bluesky / AT Protocol** | Open social graph, custom feeds | Public discourse references, academic discussion | Signed reference cards and DID-linked public discussions | **Phase 2** |
| **Reddit Evidence Forum** | Long-form community investigation | Source discovery, disagreement maps, leads | Adds candidate external references (not automatic evidence) | **Phase 3** |
| **Facebook Archive** | Groups, reunions, family history | Multi-generational local accounts | Imports participant-authorized references for family archives | **Phase 3** |
| **LinkedIn Knowledge** | Professional history, archive networks | Institutional partnerships, reviewer onboarding | Onboards authenticated museum and university curators | **Phase 3** |

---

## 3. The `SocialMemoryAdapter` Interface Standard

Every platform integration adheres to the strict typed interface to guarantee privacy, consent, and non-tokenized truth:

```typescript
export type SocialPlatform = 
  | "tiktok"
  | "instagram"
  | "youtube"
  | "farcaster"
  | "lens"
  | "bluesky"
  | "discord"
  | "telegram"
  | "reddit"
  | "facebook"
  | "linkedin";

export type ReferenceClassification =
  | "canonical-remembrance"
  | "independent-corroboration"
  | "source-evidence"
  | "institutional-attestation"
  | "participant-provided-artifact"
  | "public-discussion-reference"
  | "distribution-derivative"
  | "editorial-illustration"
  | "synthetic-visualization";

export interface ExternalReference {
  referenceId: string;                     // e.g. "donkai:external:tiktok:v_9481a0"
  platform: SocialPlatform;
  externalObjectType: "video" | "image" | "post" | "cast" | "thread" | "audio";
  externalObjectId: string;
  canonicalUrl: string;
  capturedAt: string;                      // ISO-8601 UTC
  submittedBy: string;                     // DID e.g. "did:pkh:eip155:1977:0x..."
  permissionScope: "author-authorized-reference" | "public-citation";
  contentHash: string;                     // SHA-256 of media/text bytes
  captureManifestRoot: string;             // Merkle root of capture metadata
  classification: ReferenceClassification; // Defaults to "distribution-derivative"
  historicalRole: "contextual-reference" | "supporting-candidate" | "conflicting-candidate";
  accessPolicy: "public" | "restricted" | "reviewer-only";
}

export interface SocialMemoryAdapter {
  platform: SocialPlatform;
  createPrompt(input: NeutralPromptInput): Promise<PromptReference>;
  launchContributionFlow(input: ContributionLaunch): Promise<DeepLink>;
  publishDerivative(input: AuthorizedDerivative): Promise<ExternalReference>;
  importReference(input: UserAuthorizedImport): Promise<ExternalReference>;
  verifyReference(input: ExternalReference): Promise<VerificationResult>;
}
```

---

## 4. Platform Specifications

### A. TikTok Memory Lens
- **Format:** 20–35 second vertical video template.
  - `0–3s`: *"DO NOT SEARCH THIS."*
  - `3–8s`: *"What do you remember?"*
  - `8–18s`: Neutral sound, video, or visual prompt (e.g. 1978 Space Invaders coin drop).
  - `18–25s`: *"Record your answer before reading the comments."*
  - `25–35s`: *"Seal your memory at 1977.donkai.org — Your recollection is an authentic human signal."*
- **Deep Link Schema:**
  ```text
  https://1977.donkai.org/?campaign=tiktok_arcade_1978&prompt=coin_sound_v1&mode=blind&source=tiktok
  ```
  *(Never passes PII, memory narrative, wallet keys, or private payload in URL parameters).*

### B. Instagram Memory Cards & Visual Culture
- **Formats:** 1080x1920 Story Cards, 1080x1350 Carousel Dilemmas, and 9:16 Reels.
- **Labels:** Explicit mandatory badge: `SYNTHETIC ARCHIVAL VISUALIZATION — NOT SOURCE EVIDENCE` on any illustrative imagery.
- **Before-You-Look Hook:** Directs users to record their unprompted recollection on the gateway prior to viewing source documentation.

### C. Farcaster / Base Social Layer
- **Open Frames:** 1-click launch of blind memory capture directly within Warpcast / Farcaster clients.
- **Attestations:** Non-transferable EAS (Ethereum Attestation Service) badges for verified reviewers, translators, and contributors.
- **Zero Truth Tokens:** No prediction market betting or tokenized truth auctions.

---

## 5. Non-Negotiable Protocol Safeguards

1. **Decoupled Ownership:** A social platform account is never the sole owner of a memory record. The user holds their portable LPS-1 manifest.
2. **No Popularity As Truth:** Likes, retweets, shares, upvotes, or follower counts can never alter a record's historical support classification.
3. **Client-Side Privacy:** Sensitive records are encrypted with AES-GCM-256 in the browser before off-chain upload; raw text is never published to public calldata.
4. **Append-Only Revisions:** Corrections, amendments, and access revocations produce immutable version events rather than retroactive overwrites.
