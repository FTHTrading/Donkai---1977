use crate::error::{Lps1Error, Result};
use crate::merkle::hash_internal_node;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProofNode {
    #[serde(with = "hex_serde")]
    pub hash: [u8; 32],
    pub is_left: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MerkleProof {
    pub leaf_index: u32,
    #[serde(with = "hex_serde")]
    pub leaf_hash: [u8; 32],
    pub siblings: Vec<ProofNode>,
    #[serde(with = "hex_serde")]
    pub root: [u8; 32],
}

impl MerkleProof {
    /// Verifies the Merkle inclusion proof against its embedded or expected root.
    pub fn verify(&self, expected_root: Option<&[u8; 32]>) -> Result<bool> {
        let mut current = self.leaf_hash;

        for sibling in &self.siblings {
            current = if sibling.is_left {
                hash_internal_node(&sibling.hash, &current)
            } else {
                hash_internal_node(&current, &sibling.hash)
            };
        }

        let target_root = expected_root.unwrap_or(&self.root);
        if &current == target_root {
            Ok(true)
        } else {
            Err(Lps1Error::ProofVerification(format!(
                "Calculated root 0x{} does not match expected root 0x{}",
                hex::encode(current),
                hex::encode(target_root)
            )))
        }
    }
}

pub mod hex_serde {
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(bytes: &[u8; 32], serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&hex::encode(bytes))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> std::result::Result<[u8; 32], D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let clean = s.strip_prefix("0x").unwrap_or(&s);
        let bytes = hex::decode(clean).map_err(serde::de::Error::custom)?;
        if bytes.len() != 32 {
            return Err(serde::de::Error::custom("Expected 32 bytes for hash"));
        }
        let mut arr = [0u8; 32];
        arr.copy_from_slice(&bytes);
        Ok(arr)
    }
}
