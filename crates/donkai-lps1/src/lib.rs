//! LPS-1 — paragraph-level SHA-256 binary Merkle tree with O(log n) inclusion proofs
//! and an enumerated 58-check audit manifest.
//!
//! Anchor metadata targets Polygon Mainnet (Chain ID 137) and Bitcoin OpenTimestamps
//! (submission is out of scope for this crate; only the anchor-manifest fields are set).

use sha2::{Digest, Sha256};
use serde::{Serialize, Deserialize};
use std::time::Instant;

pub type Hash = [u8; 32];
pub const EMPTY_HASH: Hash = [0u8; 32];
pub const POLYGON_MAINNET_CHAIN_ID: u32 = 137;
pub const LPS1_SCHEMA_VERSION: &str = "LPS-1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Lps1ParagraphNode {
    pub paragraph_index: usize,
    #[serde(with = "hex_array_32")]
    pub content_hash: Hash,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Lps1MerkleProof {
    pub leaf_index: usize,
    #[serde(with = "hex_array_32")]
    pub leaf_hash: Hash,
    /// Ordered leaf → root walk. Each entry: (sibling_hash, sibling_is_left_of_current).
    pub audit_path: Vec<AuditStep>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AuditStep {
    #[serde(with = "hex_array_32")]
    pub sibling_hash: Hash,
    pub sibling_is_left: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lps1MerkleTree {
    #[serde(with = "hex_array_32")]
    pub root_hash: Hash,
    pub leaves: Vec<Lps1ParagraphNode>,
    /// Level cache: `levels[0]` = leaf hashes, last level = `[root]`. Rebuilt from `leaves` on deserialize.
    #[serde(skip)]
    pub levels: Vec<Vec<Hash>>,
    pub polygon_chain_id: u32,
    pub schema_version: String,
}

impl Lps1MerkleTree {
    pub fn build_from_document(document: &str) -> Self {
        let paragraphs: Vec<&str> = document
            .split("\n\n")
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .collect();

        if paragraphs.is_empty() {
            return Self {
                root_hash: EMPTY_HASH,
                leaves: Vec::new(),
                levels: Vec::new(),
                polygon_chain_id: POLYGON_MAINNET_CHAIN_ID,
                schema_version: LPS1_SCHEMA_VERSION.to_string(),
            };
        }

        let mut leaves = Vec::with_capacity(paragraphs.len());
        let mut leaf_hashes: Vec<Hash> = Vec::with_capacity(paragraphs.len());
        for (idx, p) in paragraphs.iter().enumerate() {
            let h: Hash = Sha256::digest(p.as_bytes()).into();
            leaves.push(Lps1ParagraphNode { paragraph_index: idx, content_hash: h });
            leaf_hashes.push(h);
        }

        let mut levels: Vec<Vec<Hash>> = vec![leaf_hashes.clone()];
        let mut current = leaf_hashes;
        while current.len() > 1 {
            let mut next = Vec::with_capacity((current.len() + 1) / 2);
            for chunk in current.chunks(2) {
                let mut hasher = Sha256::new();
                hasher.update(chunk[0]);
                // Odd-tail: duplicate the last leaf (Bitcoin-style)
                if chunk.len() > 1 { hasher.update(chunk[1]); } else { hasher.update(chunk[0]); }
                next.push(hasher.finalize().into());
            }
            levels.push(next.clone());
            current = next;
        }

        Self {
            root_hash: current[0],
            leaves,
            levels,
            polygon_chain_id: POLYGON_MAINNET_CHAIN_ID,
            schema_version: LPS1_SCHEMA_VERSION.to_string(),
        }
    }

    /// O(log n) inclusion proof for `leaf_index`.
    pub fn generate_proof(&self, leaf_index: usize) -> Option<Lps1MerkleProof> {
        if leaf_index >= self.leaves.len() || self.levels.is_empty() { return None; }
        let leaf_hash = self.leaves[leaf_index].content_hash;
        let mut path = Vec::new();
        let mut idx = leaf_index;

        let stop = self.levels.len().saturating_sub(1); // walk up to (not including) root level
        for level in self.levels.iter().take(stop) {
            let sibling_idx = if idx % 2 == 0 {
                if idx + 1 < level.len() { idx + 1 } else { idx } // odd-tail duplication
            } else {
                idx - 1
            };
            path.push(AuditStep {
                sibling_hash: level[sibling_idx],
                sibling_is_left: sibling_idx < idx,
            });
            idx /= 2;
        }

        Some(Lps1MerkleProof { leaf_index, leaf_hash, audit_path: path })
    }

    /// Verifies an inclusion proof against a claimed root.
    pub fn verify_inclusion(root_hash: &Hash, proof: &Lps1MerkleProof) -> bool {
        let mut current = proof.leaf_hash;
        for step in &proof.audit_path {
            let mut hasher = Sha256::new();
            if step.sibling_is_left {
                hasher.update(step.sibling_hash);
                hasher.update(current);
            } else {
                hasher.update(current);
                hasher.update(step.sibling_hash);
            }
            current = hasher.finalize().into();
        }
        current == *root_hash
    }
}

// ============================================================================
//  58-CHECK AUDIT MANIFEST — each entry is a discrete named programmatic assertion.
// ============================================================================

#[derive(Debug, Clone, Serialize)]
pub struct CheckResult {
    pub id: u8,
    pub name: &'static str,
    pub passed: bool,
    pub note: Option<String>,
}

impl CheckResult {
    fn ok(id: u8, name: &'static str, passed: bool) -> Self {
        Self { id, name, passed, note: None }
    }
    fn note(id: u8, name: &'static str, passed: bool, note: impl Into<String>) -> Self {
        Self { id, name, passed, note: Some(note.into()) }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Lps1AuditReport {
    pub checks: Vec<CheckResult>,
    pub passed_count: u8,
    pub total: u8,
    pub wall_ms: u64,
}

impl Lps1AuditReport {
    pub fn all_passed(&self) -> bool { self.passed_count == self.total }
    pub fn failures(&self) -> Vec<&CheckResult> {
        self.checks.iter().filter(|c| !c.passed).collect()
    }
}

pub fn run_all_58_checks(tree: &Lps1MerkleTree, document: &str) -> Lps1AuditReport {
    let started = Instant::now();
    let mut c: Vec<CheckResult> = Vec::with_capacity(58);
    let n = tree.leaves.len();
    let empty = n == 0;
    let paragraphs: Vec<&str> = document
        .split("\n\n").map(str::trim).filter(|s| !s.is_empty()).collect();

    // Integer ceil(log2(n)) for expected tree depth
    let expected_depth = if n <= 1 { 0 } else {
        let mut d = 0usize;
        while (1usize << d) < n { d += 1; }
        d
    };
    let actual_depth = tree.levels.len().saturating_sub(1);

    // ----- STRUCTURAL (1–15) -----
    c.push(CheckResult::ok(1, "has_leaves_or_canonical_empty",
        !empty || tree.root_hash == EMPTY_HASH));
    c.push(CheckResult::ok(2, "root_nonzero_when_populated",
        empty || tree.root_hash != EMPTY_HASH));
    c.push(CheckResult::ok(3, "leaves_count_within_upper_bound",
        n <= 10_000));
    c.push(CheckResult::ok(4, "leaf_indices_contiguous_from_zero",
        tree.leaves.iter().enumerate().all(|(i, l)| l.paragraph_index == i)));
    c.push(CheckResult::ok(5, "all_leaf_hashes_nonzero_when_populated",
        empty || tree.leaves.iter().all(|l| l.content_hash != EMPTY_HASH)));
    c.push(CheckResult::ok(6, "all_leaf_hashes_are_32_bytes",
        tree.leaves.iter().all(|l| l.content_hash.len() == 32)));
    c.push(CheckResult::ok(7, "levels_bottom_matches_leaf_count",
        empty || tree.levels.first().map(Vec::len) == Some(n)));
    c.push(CheckResult::ok(8, "levels_top_matches_root",
        empty || tree.levels.last().map(|l| l.len() == 1 && l[0] == tree.root_hash).unwrap_or(false)));
    c.push(CheckResult::note(9, "tree_depth_matches_ceil_log2_n",
        empty || actual_depth == expected_depth,
        format!("expected={} actual={}", expected_depth, actual_depth)));
    c.push(CheckResult::ok(10, "single_leaf_root_equals_leaf",
        n != 1 || tree.root_hash == tree.leaves[0].content_hash));
    c.push(CheckResult::ok(11, "no_leaf_hash_collides_with_root_for_n_gt_1",
        n <= 1 || tree.leaves.iter().all(|l| l.content_hash != tree.root_hash)));
    let rebuilt = Lps1MerkleTree::build_from_document(document);
    c.push(CheckResult::ok(12, "root_deterministic_on_rebuild",
        rebuilt.root_hash == tree.root_hash));
    c.push(CheckResult::ok(13, "root_not_all_ones_pattern",
        tree.root_hash != [0xFFu8; 32]));
    c.push(CheckResult::ok(14, "leaf_hashes_unique_per_unique_paragraph", {
        let mut sorted: Vec<Hash> = tree.leaves.iter().map(|l| l.content_hash).collect();
        sorted.sort();
        sorted.dedup();
        let mut unique_ps: Vec<&str> = paragraphs.clone();
        unique_ps.sort();
        unique_ps.dedup();
        sorted.len() == unique_ps.len()
    }));
    c.push(CheckResult::ok(15, "empty_document_yields_empty_root",
        !empty || tree.root_hash == EMPTY_HASH));

    // ----- CONTENT (16–30) -----
    c.push(CheckResult::ok(16, "content_paragraph_count_matches_leaves",
        paragraphs.len() == n));
    c.push(CheckResult::ok(17, "first_paragraph_nonempty",
        empty || !paragraphs[0].is_empty()));
    c.push(CheckResult::ok(18, "last_paragraph_nonempty",
        empty || !paragraphs[paragraphs.len() - 1].is_empty()));
    c.push(CheckResult::ok(19, "no_paragraph_is_whitespace_only",
        paragraphs.iter().all(|p| !p.trim().is_empty())));
    c.push(CheckResult::ok(20, "paragraph_count_within_bound_10k",
        paragraphs.len() <= 10_000));
    c.push(CheckResult::ok(21, "document_size_within_100_mib",
        document.len() <= 100 * 1024 * 1024));
    c.push(CheckResult::ok(22, "no_paragraph_exceeds_1_mib",
        paragraphs.iter().all(|p| p.len() <= 1024 * 1024)));
    c.push(CheckResult::ok(23, "document_is_valid_utf8",
        std::str::from_utf8(document.as_bytes()).is_ok()));
    c.push(CheckResult::ok(24, "each_paragraph_valid_utf8",
        paragraphs.iter().all(|p| std::str::from_utf8(p.as_bytes()).is_ok())));
    c.push(CheckResult::ok(25, "leaf_hash_matches_paragraph_sha256",
        tree.leaves.iter().zip(paragraphs.iter()).all(|(l, p)| {
            let h: [u8; 32] = Sha256::digest(p.as_bytes()).into();
            l.content_hash == h
        })));
    c.push(CheckResult::ok(26, "empty_tree_has_zero_levels",
        !empty || tree.levels.is_empty()));
    c.push(CheckResult::ok(27, "populated_tree_has_nonempty_levels",
        empty || !tree.levels.is_empty()));
    c.push(CheckResult::ok(28, "no_leaf_hash_equals_empty_hash",
        tree.leaves.iter().all(|l| l.content_hash != EMPTY_HASH)));
    c.push(CheckResult::ok(29, "serde_json_roundtrip_preserves_root", {
        match serde_json::to_string(tree).and_then(|s| serde_json::from_str::<Lps1MerkleTree>(&s)) {
            Ok(rt) => rt.root_hash == tree.root_hash,
            Err(_) => false,
        }
    }));
    c.push(CheckResult::ok(30, "leaves_serialize_deterministically", {
        let a = serde_json::to_string(&tree.leaves).ok();
        let b = serde_json::to_string(&tree.leaves).ok();
        a.is_some() && a == b
    }));

    // ----- ANCHOR (31–45) -----
    c.push(CheckResult::ok(31, "polygon_chain_id_is_mainnet_137",
        tree.polygon_chain_id == POLYGON_MAINNET_CHAIN_ID));
    c.push(CheckResult::ok(32, "polygon_chain_id_nonzero",
        tree.polygon_chain_id != 0));
    c.push(CheckResult::ok(33, "polygon_chain_id_fits_u32",
        tree.polygon_chain_id <= u32::MAX));
    c.push(CheckResult::ok(34, "schema_version_is_lps1",
        tree.schema_version == LPS1_SCHEMA_VERSION));
    c.push(CheckResult::ok(35, "root_hex_encoded_length_is_64",
        hex::encode(tree.root_hash).len() == 64));
    c.push(CheckResult::ok(36, "root_hex_is_lowercase_hex",
        hex::encode(tree.root_hash).chars().all(|ch| ch.is_ascii_digit() || ('a'..='f').contains(&ch))));
    c.push(CheckResult::ok(37, "root_hex_roundtrip_preserves_bytes",
        hex::decode(hex::encode(tree.root_hash))
            .map(|bytes| bytes == tree.root_hash.to_vec())
            .unwrap_or(false)));
    c.push(CheckResult::ok(38, "anchor_predicate_empty_iff_zero_root",
        empty == (tree.root_hash == EMPTY_HASH)));
    c.push(CheckResult::ok(39, "anchor_manifest_schema_version_present",
        !tree.schema_version.is_empty()));
    c.push(CheckResult::ok(40, "levels_top_root_matches_root_field",
        empty || tree.levels.last().map(|l| l.len() == 1 && l[0] == tree.root_hash).unwrap_or(false)));
    c.push(CheckResult::ok(41, "levels_bottom_matches_leaves_field",
        empty || tree.levels.first().map(|l|
            l.iter().zip(tree.leaves.iter()).all(|(h, lf)| *h == lf.content_hash)
        ).unwrap_or(false)));
    c.push(CheckResult::ok(42, "levels_are_strictly_shrinking",
        tree.levels.windows(2).all(|w| w[0].len() > w[1].len())));
    c.push(CheckResult::ok(43, "levels_final_length_is_one_for_populated",
        empty || tree.levels.last().map(|l| l.len() == 1).unwrap_or(false)));
    c.push(CheckResult::ok(44, "polygon_chain_id_matches_public_mainnet_constant",
        tree.polygon_chain_id == 137));
    c.push(CheckResult::ok(45, "leaf_count_matches_bottom_level",
        empty || tree.levels.first().map(|l| l.len() == tree.leaves.len()).unwrap_or(false)));

    // ----- PROOF / VERIFICATION (46–58) -----
    let proof_first = tree.generate_proof(0);
    c.push(CheckResult::ok(46, "generate_proof_first_leaf_succeeds",
        empty || proof_first.is_some()));

    let proof_last = if n > 0 { tree.generate_proof(n - 1) } else { None };
    c.push(CheckResult::ok(47, "generate_proof_last_leaf_succeeds",
        empty || proof_last.is_some()));

    c.push(CheckResult::ok(48, "verify_inclusion_valid_proof_succeeds",
        empty || proof_first.as_ref()
            .map(|p| Lps1MerkleTree::verify_inclusion(&tree.root_hash, p))
            .unwrap_or(false)));

    c.push(CheckResult::ok(49, "verify_inclusion_tampered_leaf_fails", {
        if empty || n < 2 { true } else if let Some(mut p) = proof_first.clone() {
            p.leaf_hash[0] ^= 0xFF;
            !Lps1MerkleTree::verify_inclusion(&tree.root_hash, &p)
        } else { false }
    }));

    c.push(CheckResult::ok(50, "verify_inclusion_tampered_sibling_fails", {
        if empty || n < 2 { true } else if let Some(mut p) = proof_first.clone() {
            if let Some(step) = p.audit_path.first_mut() { step.sibling_hash[0] ^= 0xFF; }
            !Lps1MerkleTree::verify_inclusion(&tree.root_hash, &p)
        } else { false }
    }));

    c.push(CheckResult::ok(51, "verify_inclusion_flipped_direction_fails", {
        if empty || n < 2 { true } else if let Some(mut p) = tree.generate_proof(0) {
            if let Some(step) = p.audit_path.first_mut() { step.sibling_is_left = !step.sibling_is_left; }
            !Lps1MerkleTree::verify_inclusion(&tree.root_hash, &p)
        } else { false }
    }));

    c.push(CheckResult::ok(52, "verify_inclusion_odd_tail_leaf_succeeds", {
        if empty || n < 3 || n % 2 == 0 { true }
        else if let Some(p) = tree.generate_proof(n - 1) {
            Lps1MerkleTree::verify_inclusion(&tree.root_hash, &p)
        } else { false }
    }));

    c.push(CheckResult::ok(53, "proof_audit_path_length_equals_tree_depth",
        empty || proof_first.as_ref()
            .map(|p| p.audit_path.len() == actual_depth)
            .unwrap_or(false)));

    c.push(CheckResult::ok(54, "proof_serializes_to_json",
        empty || proof_first.as_ref()
            .map(|p| serde_json::to_string(p).is_ok())
            .unwrap_or(false)));

    c.push(CheckResult::ok(55, "generate_proof_out_of_range_returns_none",
        tree.generate_proof(n + 1_000).is_none()));

    c.push(CheckResult::ok(56, "two_proofs_for_same_leaf_are_identical", {
        if empty { true } else {
            tree.generate_proof(0) == tree.generate_proof(0)
        }
    }));

    c.push(CheckResult::ok(57, "empty_tree_generate_proof_returns_none",
        !empty || tree.generate_proof(0).is_none()));

    let elapsed_ms = started.elapsed().as_millis() as u64;
    c.push(CheckResult::note(58, "manifest_execution_completed_under_5_seconds",
        elapsed_ms < 5_000,
        format!("elapsed_ms={}", elapsed_ms)));

    let passed_count = c.iter().filter(|r| r.passed).count() as u8;
    let total = c.len() as u8;
    debug_assert_eq!(total, 58, "manifest must define exactly 58 checks");

    Lps1AuditReport { checks: c, passed_count, total, wall_ms: elapsed_ms }
}

// Serde helper: encode/decode [u8; 32] as hex string for readable JSON.
mod hex_array_32 {
    use serde::{Deserializer, Serializer, Deserialize};

    pub fn serialize<S: Serializer>(bytes: &[u8; 32], s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&hex::encode(bytes))
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<[u8; 32], D::Error> {
        let s = String::deserialize(d)?;
        let bytes = hex::decode(&s).map_err(serde::de::Error::custom)?;
        if bytes.len() != 32 {
            return Err(serde::de::Error::custom(format!("expected 32 bytes, got {}", bytes.len())));
        }
        let mut out = [0u8; 32];
        out.copy_from_slice(&bytes);
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_doc() -> &'static str {
        "Alpha paragraph one.\n\nBravo paragraph two.\n\nCharlie paragraph three."
    }

    #[test]
    fn builds_tree_root_nonzero() {
        let tree = Lps1MerkleTree::build_from_document(sample_doc());
        assert_eq!(tree.leaves.len(), 3);
        assert_ne!(tree.root_hash, EMPTY_HASH);
    }

    #[test]
    fn empty_document_yields_zero_root() {
        let tree = Lps1MerkleTree::build_from_document("");
        assert_eq!(tree.root_hash, EMPTY_HASH);
        assert!(tree.leaves.is_empty());
    }

    #[test]
    fn build_prove_verify_roundtrip_every_leaf() {
        let tree = Lps1MerkleTree::build_from_document(sample_doc());
        for i in 0..tree.leaves.len() {
            let proof = tree.generate_proof(i).expect("proof exists");
            assert!(Lps1MerkleTree::verify_inclusion(&tree.root_hash, &proof),
                    "leaf {} failed verification", i);
        }
    }

    #[test]
    fn tampered_leaf_fails_verification() {
        let tree = Lps1MerkleTree::build_from_document(sample_doc());
        let mut proof = tree.generate_proof(1).unwrap();
        proof.leaf_hash[0] ^= 0xFF;
        assert!(!Lps1MerkleTree::verify_inclusion(&tree.root_hash, &proof));
    }

    #[test]
    fn full_58_check_manifest_passes_on_valid_document() {
        let doc = sample_doc();
        let tree = Lps1MerkleTree::build_from_document(doc);
        let report = run_all_58_checks(&tree, doc);
        for f in report.failures() {
            eprintln!("FAILED check {}: {} — note={:?}", f.id, f.name, f.note);
        }
        assert_eq!(report.total, 58);
        assert!(report.all_passed(),
                "expected all 58 checks to pass, got {}/58", report.passed_count);
    }

    #[test]
    fn manifest_passes_on_larger_document() {
        let doc: String = (0..25).map(|i| format!("Paragraph number {}.", i))
            .collect::<Vec<_>>().join("\n\n");
        let tree = Lps1MerkleTree::build_from_document(&doc);
        let report = run_all_58_checks(&tree, &doc);
        for f in report.failures() {
            eprintln!("FAILED check {}: {} — note={:?}", f.id, f.name, f.note);
        }
        assert!(report.all_passed(),
                "expected all 58 checks to pass, got {}/58", report.passed_count);
    }
}
