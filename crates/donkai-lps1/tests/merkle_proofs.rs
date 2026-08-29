use donkai_lps1::{hash_leaf, MerkleTree};

#[test]
fn test_merkle_tree_and_proof() {
    let leaf1 = hash_leaf("test", b"sample item 1");
    let leaf2 = hash_leaf("test", b"sample item 2");
    let leaf3 = hash_leaf("test", b"sample item 3");
    let leaf4 = hash_leaf("test", b"sample item 4");

    let tree = MerkleTree::build(vec![leaf1, leaf2, leaf3, leaf4]).expect("Tree build failed");
    let proof = tree.generate_proof(2).expect("Proof generation failed");

    assert_eq!(proof.leaf_index, 2);
    assert_eq!(proof.leaf_hash, leaf3);
    assert!(proof.verify(Some(&tree.root)).expect("Verification failed"));
}
