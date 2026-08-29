use crate::error::{Lps1Error, Result};
use crate::proof::{MerkleProof, ProofNode};
use sha2::{Digest, Sha256};

/// Computes a typed, domain-separated leaf hash.
pub fn hash_leaf(object_type: &str, canonical_bytes: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    let prefix = format!("DONKAI:LPS1:LEAF:{}:v1:", object_type.to_uppercase());
    hasher.update(prefix.as_bytes());
    hasher.update(canonical_bytes);
    let result = hasher.finalize();
    let mut out = [0u8; 32];
    out.copy_from_slice(&result);
    out
}

/// Computes an internal node hash from left and right children with domain separation.
pub fn hash_internal_node(left: &[u8; 32], right: &[u8; 32]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(b"DONKAI:LPS1:NODE:v1:");
    hasher.update(left);
    hasher.update(right);
    let result = hasher.finalize();
    let mut out = [0u8; 32];
    out.copy_from_slice(&result);
    out
}

/// Computes a domain-separated bundle root from the raw Merkle tree root.
pub fn hash_bundle_root(bundle_type: &str, tree_root: &[u8; 32]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    let prefix = format!("DONKAI:LPS1:ROOT:{}:v1:", bundle_type.to_uppercase());
    hasher.update(prefix.as_bytes());
    hasher.update(tree_root);
    let result = hasher.finalize();
    let mut out = [0u8; 32];
    out.copy_from_slice(&result);
    out
}

#[derive(Debug, Clone)]
pub struct MerkleTree {
    pub leaves: Vec<[u8; 32]>,
    pub layers: Vec<Vec<[u8; 32]>>,
    pub root: [u8; 32],
}

/// Domain-separated constant for unpaired odd Merkle leaves:
/// SHA-256("DONKAI:LPS1:EMPTY_MERKLE_LEAF:v1")
pub const EMPTY_LEAF_CONSTANT: [u8; 32] = [
    0x24, 0x11, 0x98, 0x59, 0xf7, 0x7f, 0x04, 0x12,
    0x8f, 0x1d, 0xae, 0xf6, 0xd8, 0xe2, 0x0e, 0xec,
    0x9a, 0x79, 0x7f, 0x78, 0x1d, 0xf0, 0x34, 0x33,
    0x9b, 0xc0, 0xe9, 0x98, 0xa4, 0xd7, 0x00, 0xdc,
];

/// Empty evidence bundle root constant (32 zero bytes)
pub const EMPTY_EVIDENCE_ROOT: [u8; 32] = [0u8; 32];

impl MerkleTree {
    /// Builds a Merkle tree from a list of 32-byte leaf hashes.
    /// Uses Option B: Unpaired odd leaves are paired with EMPTY_LEAF_CONSTANT.
    pub fn build(leaves: Vec<[u8; 32]>) -> Result<Self> {
        if leaves.is_empty() {
            return Err(Lps1Error::Validation(
                "Cannot build Merkle tree from empty leaves".into(),
            ));
        }

        let mut layers = Vec::new();
        layers.push(leaves.clone());

        let mut current_layer = leaves.clone();
        while current_layer.len() > 1 {
            let mut next_layer = Vec::new();
            for i in (0..current_layer.len()).step_by(2) {
                let left = current_layer[i];
                let right = if i + 1 < current_layer.len() {
                    current_layer[i + 1]
                } else {
                    // Option B: Pair odd leaf with EMPTY_LEAF_CONSTANT
                    EMPTY_LEAF_CONSTANT
                };
                let parent = hash_internal_node(&left, &right);
                next_layer.push(parent);
            }
            layers.push(next_layer.clone());
            current_layer = next_layer;
        }

        let root = current_layer[0];
        Ok(Self {
            leaves,
            layers,
            root,
        })
    }

    /// Generates a Merkle inclusion proof for the leaf at `leaf_index`.
    pub fn generate_proof(&self, leaf_index: usize) -> Result<MerkleProof> {
        if leaf_index >= self.leaves.len() {
            return Err(Lps1Error::InvalidLeafIndex {
                index: leaf_index,
                size: self.leaves.len(),
            });
        }

        let leaf_hash = self.leaves[leaf_index];
        let mut siblings = Vec::new();
        let mut idx = leaf_index;

        for layer in &self.layers[0..self.layers.len() - 1] {
            let is_right_child = idx % 2 == 1;
            let sibling_idx = if is_right_child {
                idx - 1
            } else if idx + 1 < layer.len() {
                idx + 1
            } else {
                idx // odd node duplicated
            };

            let sibling_hash = layer[sibling_idx];
            siblings.push(ProofNode {
                hash: sibling_hash,
                is_left: is_right_child,
            });

            idx /= 2;
        }

        Ok(MerkleProof {
            leaf_index: leaf_index as u32,
            leaf_hash,
            siblings,
            root: self.root,
        })
    }
}
