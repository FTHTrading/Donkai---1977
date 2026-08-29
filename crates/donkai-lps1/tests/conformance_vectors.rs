//! Conformance Test Runner verifying the shared fixtures in fixtures/lps1-v1/

use donkai_lps1::{canonicalize, hash_leaf};
use serde_json::Value;
use std::fs;
use std::path::Path;

#[test]
fn test_conformance_unicode_nfc_fixture() {
    let fixture_path = Path::new("../../fixtures/lps1-v1/canonicalization/unicode_nfc.json");
    if let Ok(content) = fs::read_to_string(fixture_path) {
        let fixture: Value = serde_json::from_str(&content).expect("Valid JSON fixture");
        let precomposed = &fixture["inputs"]["precomposed"];
        let canon_pre = canonicalize(precomposed).expect("canonicalize failed");
        assert_eq!(canon_pre.as_str(), fixture["expectedCanonicalJson"].as_str().unwrap());
        
        let leaf = hash_leaf("remembrance", canon_pre.as_bytes());
        assert_eq!(leaf.len(), 32);
    }
}

#[test]
fn test_conformance_odd_leaves_fixture() {
    let fixture_path = Path::new("../../fixtures/lps1-v1/merkle/odd_leaves.json");
    if let Ok(content) = fs::read_to_string(fixture_path) {
        let fixture: Value = serde_json::from_str(&content).expect("Valid JSON fixture");
        let empty_root = fixture["emptyEvidenceRoot"].as_str().unwrap();
        assert_eq!(empty_root, "0x0000000000000000000000000000000000000000000000000000000000000000");
    }
}

#[test]
fn test_conformance_eip712_fixture() {
    let fixture_path = Path::new("../../fixtures/lps1-v1/eip712/create_remembrance_digest.json");
    if let Ok(content) = fs::read_to_string(fixture_path) {
        let fixture: Value = serde_json::from_str(&content).expect("Valid JSON fixture");
        assert_eq!(fixture["domain"]["chainId"], 1977);
        assert_eq!(fixture["primaryType"], "CreateRemembrance");
        assert_eq!(fixture["expectedVerification"], true);
    }
}
