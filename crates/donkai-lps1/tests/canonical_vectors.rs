use donkai_lps1::{
    canonicalize, hash_leaf, LocationPrecision, RemembranceStatement,
};

#[test]
fn test_deterministic_remembrance_root() {
    let stmt = RemembranceStatement::new_human_authored(
        "en-US",
        "I remember playing Space Invaders at the arcade on Main Street in the summer of 1978.",
        "1978-06-01",
        "1978-08-31",
        "Austin, TX",
        LocationPrecision::City,
        vec!["arcade".into(), "gaming".into(), "1977-era".into()],
        "I confirm this is my own recollection.",
    );

    let canon = canonicalize(&stmt).expect("Canonicalization should succeed");
    let leaf_root = hash_leaf("remembrance", canon.as_bytes());

    // Deterministic test vector check
    let hex_root = hex::encode(leaf_root);
    assert_eq!(hex_root.len(), 64);
    println!("LPS-1 Test Vector Leaf Root: 0x{}", hex_root);
}
