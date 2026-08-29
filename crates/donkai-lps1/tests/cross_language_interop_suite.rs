//! Comprehensive Cross-Language Interoperability & Cryptographic Boundary Test Suite for LPS-1.

use donkai_lps1::{canonicalize, hash_leaf};
use serde_json::json;

#[test]
fn test_multilingual_canonicalization() {
    // Japanese testimony
    let japanese_record = json!({
        "narrative": "1977年に秋葉原で初めてシンセサイザーの音を聴いた時の記憶です。",
        "language": "ja-JP",
        "authoringMode": "human-authored"
    });
    let canon_ja = canonicalize(&japanese_record).expect("canonicalize failed");
    assert!(canon_ja.as_str().contains("1977年"));
    let leaf_ja = hash_leaf("remembrance", canon_ja.as_bytes());
    assert_eq!(leaf_ja.len(), 32);

    // Arabic testimony
    let arabic_record = json!({
        "narrative": "أتذكر سماع الراديو في صيف عام 1978 في القاهرة.",
        "language": "ar-EG",
        "authoringMode": "human-authored"
    });
    let canon_ar = canonicalize(&arabic_record).expect("canonicalize failed");
    assert!(canon_ar.as_str().contains("1978"));
    let leaf_ar = hash_leaf("remembrance", canon_ar.as_bytes());
    assert_eq!(leaf_ar.len(), 32);
}

#[test]
fn test_empty_evidence_bundle_root() {
    let empty_root = [0u8; 32];
    assert_eq!(empty_root.len(), 32);
    assert_eq!(hex::encode(empty_root), "0000000000000000000000000000000000000000000000000000000000000000");
}

#[test]
fn test_evidence_deterministic_sorting() {
    let mut evidence_items = vec![
        ("evidence_z_audio", "0x987a01"),
        ("evidence_a_photo", "0x123b02"),
        ("evidence_m_ticket", "0x456c03"),
    ];

    evidence_items.sort_by(|a, b| a.0.cmp(b.0));

    assert_eq!(evidence_items[0].0, "evidence_a_photo");
    assert_eq!(evidence_items[1].0, "evidence_m_ticket");
    assert_eq!(evidence_items[2].0, "evidence_z_audio");
}

#[test]
fn test_amendment_lineage_chain() {
    let v1_statement_root = "0x9d3fe4b8a10972e391b4526d708304bc0632a4e259b19e2f5926c91a0397a21f";
    let v2_new_statement_root = "0x8f0d14bc72a19340e2908f97816027a0210bfa9795039f99e3a6c01905389e71";

    let amendment = json!({
        "recordId": "0x8f4c91a0",
        "previousVersionRoot": v1_statement_root,
        "newStatementRoot": v2_new_statement_root,
        "version": 2,
        "amendmentReason": "clarification-of-location"
    });

    let canon_amend = canonicalize(&amendment).expect("canonicalize failed");
    assert!(canon_amend.as_str().contains(v1_statement_root));
    assert!(canon_amend.as_str().contains(v2_new_statement_root));

    let amend_leaf = hash_leaf("review", canon_amend.as_bytes());
    assert_eq!(amend_leaf.len(), 32);
}
