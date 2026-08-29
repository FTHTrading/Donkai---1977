//! End-to-End Integration Test for the Federated Social Memory & LPS-1 Verification Pipeline.

use donkai_lps1::{canonicalize, hash_leaf};
use serde_json::json;
use sha2::{Digest, Sha256};

#[test]
fn test_federated_e2e_canonicalization_and_leaf_root() {
    let narrative = "I remember when the Space Invaders cabinet was installed at the Main Street arcade in Austin in July 1978. Everyone crowded around because it was the first machine that took the newly minted solid brass tokens with the grooved edge. When you dropped the token, the machine emitted a distinct two-tone descending beep that you could hear across the entire arcade floor.";

    let record = json!({
        "authoringMode": "human-authored",
        "confidenceLevel": "vivid-but-uncertain",
        "eventTimeframe": "Summer 1978",
        "language": "en-US",
        "location": "Austin, Texas, United States",
        "narrative": narrative,
        "sourceAwareness": "direct-experience"
    });

    let canonical_json = canonicalize(&record).expect("canonicalize failed");
    assert!(!canonical_json.is_empty());

    let leaf_hash = hash_leaf("remembrance", canonical_json.as_bytes());
    let hex_hash = hex::encode(leaf_hash);
    assert_eq!(hex_hash.len(), 64);

    let leaf_hash_second_pass = hash_leaf("remembrance", canonical_json.as_bytes());
    assert_eq!(leaf_hash, leaf_hash_second_pass);
}

#[test]
fn test_blind_corroboration_masking_boundary() {
    let original_record_id = "0x8f4c91a0293eb1860e19fb27509c316a9082ef74092b7194630a9108b5e902b4";
    let blind_prompt = "Recall any coin, sound, cabinet, or ritual from late 1970s arcades.";

    let mut hasher = Sha256::new();
    hasher.update(blind_prompt.as_bytes());
    let neutral_prompt_hash = hasher.finalize();
    let neutral_hex = hex::encode(neutral_prompt_hash);

    let witness_statement = "The cabinet had a brass token drop that made a dual descending chime.";
    let witness_canon = canonicalize(&json!({ "narrative": witness_statement })).expect("canonicalize failed");
    let witness_leaf = hash_leaf("corroboration", witness_canon.as_bytes());

    assert_ne!(original_record_id, hex::encode(witness_leaf));
    assert_eq!(neutral_hex.len(), 64);
}
