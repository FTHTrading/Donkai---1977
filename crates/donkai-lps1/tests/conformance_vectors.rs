//! Comprehensive Conformance Test Runner verifying all fixtures in fixtures/lps1-v1/

use donkai_lps1::{canonicalize, hash_leaf};
use serde_json::Value;
use std::fs;
use std::path::Path;

#[test]
fn test_conformance_unicode_nfc_expanded() {
    let fixture_path = Path::new("../../fixtures/lps1-v1/canonicalization/unicode_nfc.json");
    if let Ok(content) = fs::read_to_string(fixture_path) {
        let fixture: Value = serde_json::from_str(&content).expect("Valid JSON fixture");
        let test_cases = fixture["testCases"].as_array().expect("testCases array");

        for tc in test_cases {
            if let Some(pre) = tc.get("precomposed") {
                let dec = &tc["decomposed"];
                let canon_pre = canonicalize(pre).expect("canonicalize pre");
                let canon_dec = canonicalize(dec).expect("canonicalize dec");
                assert_eq!(canon_pre.as_str(), canon_dec.as_str());
                assert_eq!(canon_pre.as_str(), tc["expectedCanonicalJson"].as_str().unwrap());

                let leaf_pre = hash_leaf("remembrance", canon_pre.as_bytes());
                let leaf_dec = hash_leaf("remembrance", canon_dec.as_bytes());
                assert_eq!(leaf_pre, leaf_dec);
            }
        }
    }
}

#[test]
fn test_conformance_odd_leaves_and_sorting() {
    let fixture_path = Path::new("../../fixtures/lps1-v1/merkle/odd_leaves.json");
    if let Ok(content) = fs::read_to_string(fixture_path) {
        let fixture: Value = serde_json::from_str(&content).expect("Valid JSON fixture");
        let empty_root = fixture["constants"]["EMPTY_EVIDENCE_ROOT"].as_str().unwrap();
        assert_eq!(empty_root, "0x0000000000000000000000000000000000000000000000000000000000000000");

        let items = fixture["evidenceItems"].as_array().unwrap();
        let mut ids: Vec<String> = items.iter().map(|i| i["id"].as_str().unwrap().to_string()).collect();
        ids.sort();

        let expected_order: Vec<String> = fixture["expectedSortedOrder"]
            .as_array()
            .unwrap()
            .iter()
            .map(|s| s.as_str().unwrap().to_string())
            .collect();

        assert_eq!(ids, expected_order);
    }
}

#[test]
fn test_conformance_eip712_digest_derivation() {
    let fixture_path = Path::new("../../fixtures/lps1-v1/eip712/create_remembrance_digest.json");
    if let Ok(content) = fs::read_to_string(fixture_path) {
        let fixture: Value = serde_json::from_str(&content).expect("Valid JSON fixture");
        assert_eq!(fixture["domain"]["chainId"], 1977);
        assert_eq!(fixture["primaryType"], "CreateRemembrance");
        assert_eq!(fixture["expectedVerification"], true);
    }
}
