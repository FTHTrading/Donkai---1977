<div align="center">
  <img src="web/img/donkai-mark.svg" alt="Donkai · 1977" height="64"/>
</div>

<div align="center">

# Donkai Network · Chain 1977

**A satirical Layer-1 blockchain that prices human obstinacy directly on-chain.**

[![site](https://img.shields.io/badge/site-live-c8d4e0?style=for-the-badge&labelColor=1c1c26)](https://fthtrading.github.io/Donkai---1977/)
[![tests](https://img.shields.io/badge/rust_tests-28%2F28-d8d8de?style=for-the-badge&labelColor=1c1c26)](#-08-verification)
[![contracts](https://img.shields.io/badge/solidity-3_compile_clean-e0d8c8?style=for-the-badge&labelColor=1c1c26)](#-05-solidity-contracts)
[![license](https://img.shields.io/badge/license-MIT_OR_Apache--2.0-d4b8b0?style=for-the-badge&labelColor=1c1c26)](#-09-license)
[![chain](https://img.shields.io/badge/chain-1977-a0a4ae?style=for-the-badge&labelColor=1c1c26)](#-01-overview)

</div>

---

## Table of Contents

| § | Section | Purpose |
|---|---|---|
| 01 | [Overview](#-01-overview) | What Donkai is and what ships in v0.1.0 |
| 02 | [Quick Start](#-02-quick-start) | Clone, test, run — end to end |
| 03 | [Repository Layout](#-03-repository-layout) | Where every file lives |
| 04 | [Rust Workspace](#-04-rust-workspace) | The six crates + test results |
| 05 | [Solidity Contracts](#-05-solidity-contracts) | Three contracts + bytecode sizes |
| 06 | [Ownership Through LPS-1](#-06-ownership-through-lps-1) | Why documents, not token IDs, are the primary key |
| 07 | [Front-Desk Site](#-07-front-desk-site-donkaiorg) | donkai.org — architecture and pages |
| 08 | [Verification](#-08-verification) | How to prove every claim in this README |
| 09 | [License](#-09-license) | Dual MIT / Apache-2.0 |

---

## ![01](https://img.shields.io/badge/-01-f4f4f8?style=for-the-badge&labelColor=1c1c26) Overview

Donkai Network (Chain ID `1977` — the Atari VCS launch year) is an EVM-compatible Layer-1 blockchain wrapping real infrastructure inside satirical framing.

**The real infrastructure:**

- **LPS-1 provenance** — paragraph SHA-256 → binary Merkle tree → 58 enumerated audit checks → Polygon 137 + Bitcoin OpenTimestamps anchors
- **Proof-of-Stubbornness consensus** — validator vote weight = `stake × blocks_unmoved × tier`; strict 2/3 BFT with `agreeing × 3 ≥ total × 2` saturating u128 arithmetic (Asinine Fault Tolerance is the marketing name; the math is real)
- **Post-quantum signatures** — ML-DSA-87 (FIPS 204) validator sigs, ML-KEM-1024 (FIPS 203) key exchange via `pqcrypto-*`
- **PolicyGuard** — deterministic multi-signature adjudication for AI agent actions; D0–D5 risk tiers, D4/D5 require a cryptographically-verified 2-of-N quorum against a whitelisted validator set (**verified, not counted**)
- **Native CIDv1** — raw + dag-pb/UnixFS in pure Rust, plus a blocking Kubo RPC pin client

**The satirical framing:**

- Five historical eras (`PreDigital` → `PhysicalArcade` → `PaperTicket` → `DigitalMicrotrans` → `AgenticYield`) map every wallet's behavior to fifty years of programmed dopamine
- Format-war staking arena for canonical rivalries (Betamax/VHS, Mac/Wintel, HD-DVD/Blu-ray)
- Two-zone museum of relic NFTs (Zone 1: Behavioral Dopamine, Zone 2: Format & Distribution Wars)
- Six-chapter narrative — *The Quarter, The Ticket Roll, The Beanie, The Horse Armor, The First Signature, The Return*

---

## ![02](https://img.shields.io/badge/-02-c8d4e0?style=for-the-badge&labelColor=1c1c26) Quick Start

```bash
# 1. Clone
git clone https://github.com/FTHTrading/Donkai---1977.git
cd Donkai---1977

# 2. Rust workspace — 28/28 tests, warning-free
cargo test --workspace
cargo run                       # boot the reference node

# 3. Solidity contracts — 3 contracts, all under EIP-170 24KB
cd contracts
git clone --depth 1 --branch v5.0.2  https://github.com/OpenZeppelin/openzeppelin-contracts.git lib/openzeppelin-contracts
git clone --depth 1 --branch v1.9.4  https://github.com/foundry-rs/forge-std.git             lib/forge-std
git clone --depth 1                  https://github.com/TokenySolutions/T-REX.git            lib/T-REX
forge build --sizes

# 4. Static site — no build step, any HTTP server
cd ../web
python -m http.server 8080      # or:  npx serve
open http://localhost:8080
```

Or just visit **https://fthtrading.github.io/Donkai---1977/**.

---

## ![03](https://img.shields.io/badge/-03-d8d8de?style=for-the-badge&labelColor=1c1c26) Repository Layout

```text
donkai-core/
├── Cargo.toml                          # Rust workspace root
├── WHITEPAPER.md                       # shipped-vs-not-shipped split
├── LICENSE-MIT · LICENSE-APACHE        # dual license
├── crates/
│   ├── donkai-lps1/                    # paragraph Merkle + 58 audit checks
│   ├── donkai-pqc/                     # ML-DSA-87 + ML-KEM-1024
│   ├── donkai-policyguard/             # sig-verified quorum adjudication
│   ├── donkai-consensus/               # PoS + 2/3 BFT (AFT)
│   ├── donkai-ipfs/                    # native CIDv1 + Kubo RPC pin
│   └── donkai-node/                    # reference boot console
├── contracts/
│   ├── foundry.toml
│   ├── remappings.txt
│   ├── src/
│   │   ├── DonkaiTimelineRegistry.sol  # ERC-721 relic NFT + LPS-1 binding
│   │   ├── DonkaiFormatWar.sol         # 2-sided staking + pull-payment
│   │   ├── DonkaiAnchorMirror.sol      # Polygon 137 anchor log
│   │   ├── libraries/DonkaiErrors.sol
│   │   └── interfaces/external/        # thin T-REX ABI stubs
│   └── lib/                            # forge deps (git-ignored)
├── web/                                # donkai.org static site
│   ├── index.html · stories.html
│   ├── css/main.css   · css/liquid.css
│   ├── js/main.js     · js/liquid-scene.js
│   └── img/donkai-mark.svg · img/favicon.svg
└── .github/workflows/pages.yml         # auto-deploy to GitHub Pages
```

---

## ![04](https://img.shields.io/badge/-04-e0d8c8?style=for-the-badge&labelColor=1c1c26) Rust Workspace

| Crate | Purpose | Key Types |
|---|---|---|
| `donkai-lps1` | Paragraph-split SHA-256 binary Merkle tree, O(log n) inclusion proofs, 58 discrete named audit checks | `Lps1MerkleTree`, `Lps1MerkleProof`, `Lps1AuditReport` |
| `donkai-pqc` | ML-DSA-87 signatures (FIPS 204) + ML-KEM-1024 KEM (FIPS 203) via `pqcrypto-*` | `MlDsaKeypair`, `MlKemKeypair` |
| `donkai-policyguard` | Multi-signature adjudication verifying each ML-DSA sig against a whitelisted validator set | `AgentProposal`, `EvalOutcome`, `PolicyGuardEvaluator` |
| `donkai-consensus` | Proof-of-Stubbornness validator tiers + strict 2/3 BFT with saturating u128 math | `DonkaiValidator`, `ValidatorTier`, `AsinineFaultTolerance` |
| `donkai-ipfs` | Native CIDv1 (raw + dag-pb/UnixFS) + blocking Kubo RPC pin client | `compute_raw_cidv1`, `compute_dagpb_cidv1`, `pin_to_kubo` |
| `donkai-node` | Reference boot binary wiring the five modules into a runnable node | — |

**Test coverage — `cargo test --workspace`:**

```text
donkai_consensus    4 passed / 0 failed
donkai_ipfs         7 passed / 0 failed
donkai_lps1         6 passed / 0 failed   (includes full 58-check manifest verification)
donkai_policyguard  6 passed / 0 failed
donkai_pqc          5 passed / 0 failed
                   ───────────────────
TOTAL              28 passed / 0 failed   warning-free build
```

---

## ![05](https://img.shields.io/badge/-05-d4b8b0?style=for-the-badge&labelColor=1c1c26) Solidity Contracts

| Contract | Runtime Size | Description |
|---|---|---|
| `DonkaiTimelineRegistry.sol` | **8,825 B** | ERC-721 relic NFT with mandatory LPS-1 Merkle root binding, 5-era enum, Pasqually-AI stubbornness oracle role, optional ERC-3643 identity gate |
| `DonkaiFormatWar.sol` | **7,873 B** | Two-sided historical rivalry staking with pull-payment pro-rata claims, optional ERC-3643 modular-compliance gate on ERC-20 stakes |
| `DonkaiAnchorMirror.sol` | small | Minimal Polygon 137 commitment log for cross-chain LPS-1 root provenance |

All compile with **solc 0.8.24 Cancun + via_ir**, well under the EIP-170 24KB limit.

**Dependencies (vendored via `git clone` into `contracts/lib/`, git-ignored):**

- OpenZeppelin Contracts **v5.0.2**
- forge-std **v1.9.4**
- T-REX 4.1.6 (referenced by version in `package.json`; used only for interface reference — thin local ABI stubs live under `src/interfaces/external/` to sidestep T-REX's `solc =0.8.17` pin, which is incompatible with OZ v5's `^0.8.20`)

---

## ![06](https://img.shields.io/badge/-06-c8d4e0?style=for-the-badge&labelColor=1c1c26) Ownership Through LPS-1

Traditional NFTs bind a wallet to a token ID. **Donkai binds a wallet to a document.**

The LPS-1 Merkle root of your text *is* your ownership token — cryptographic proof that this exact string of characters, in this exact order, was minted by you and no one else.

**Flow:**

1. **Write** — story, receipt, provenance chain, personal artifact.
2. **Paragraph hash** — each `\n\n`-separated block becomes a SHA-256 leaf.
3. **Merkle tree** — leaves pair upward until a single 32-byte root emerges.
4. **On-chain claim** — `DonkaiTimelineRegistry.mintRelic(title, ipfsCID, lps1MerkleRoot, era)` binds the root to your address. Duplicate roots revert (`DonkaiErrors.DuplicateMerkleRoot`).
5. **Provenance forever** — anyone with the original document can regenerate the exact root and verify your claim in `<1s`. Change one character and the root shatters.

The mint desk on [donkai.org/#mint](https://fthtrading.github.io/Donkai---1977/#mint) computes the LPS-1 root and IPFS CIDv1 in-browser using SubtleCrypto — **byte-identical** to what the Rust `donkai-lps1` and `donkai-ipfs` crates produce on-chain. Preview generates a portable ownership certificate.

---

## ![07](https://img.shields.io/badge/-07-b8b8c2?style=for-the-badge&labelColor=1c1c26) Front-Desk Site (donkai.org)

Deployed via GitHub Actions to **https://fthtrading.github.io/Donkai---1977/**.

| Page | Contents |
|---|---|
| `index.html` | 3D chrome-morphing Three.js hero, five-era timeline, "How it works" pillars, LPS-1 ownership explainer, mint desk with in-browser derivation + certificate preview, format war arena, two-zone museum (10 seed relics), stack registry |
| `stories.html` | **Six Chapters** narrative: The Quarter (1985) · The Ticket Roll (1994) · The Beanie (1998) · The Horse Armor (2006) · The First Signature (2016) · The Return (2026) |

**Aesthetic:** pure liquid metal — chrome, platinum, mercury, pearl. No neons. Iridescent chrome text on the hero, glass-morphism panels with backdrop blur, chrome-shine sweep on hover, dark obsidian background with muted metal accents.

**Zero build step.** Static HTML/CSS/JS; Three.js loaded via CDN import map. Auto-deploys on every push to `main` touching `web/**` or `.github/workflows/pages.yml`.

---

## ![08](https://img.shields.io/badge/-08-a0a4ae?style=for-the-badge&labelColor=1c1c26) Verification

```bash
# Rust
cargo check --workspace --all-targets   # zero warnings
cargo test  --workspace                 # 28/28 pass
cargo run                               # boots reference node — prints LPS-1 audit + PQC + consensus + PolicyGuard

# Solidity
cd contracts && forge build --sizes     # 3 contracts, all under EIP-170 24KB

# Site
curl -s -o /dev/null -w "HTTP %{http_code}\n" https://fthtrading.github.io/Donkai---1977/
```

**Current tag:** [`v0.1.0`](https://github.com/FTHTrading/Donkai---1977/releases/tag/v0.1.0)

---

## ![09](https://img.shields.io/badge/-09-7c7c88?style=for-the-badge&labelColor=1c1c26) License

Dual-licensed under **[MIT](./LICENSE-MIT)** or **[Apache-2.0](./LICENSE-APACHE)** at your option.

Satirical infrastructure. Not investment advice. Not FIPS 140-3 module certified (though the primitives — ML-DSA-87, ML-KEM-1024 — are NIST-standardized).
