use crate::canonical::canonicalize;
use crate::error::Result;
use crate::merkle::{hash_bundle_root, hash_leaf, MerkleTree};
use crate::proof::hex_serde;
use crate::schema::MemoryRecord;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Commitment {
    pub schema: String,
    pub algorithm: String,
    #[serde(with = "hex_serde")]
    pub root: [u8; 32],
    #[serde(rename = "leafCount")]
    pub leaf_count: u32,
    #[serde(rename = "canonicalByteLength")]
    pub canonical_byte_length: u64,
}

impl Commitment {
    pub fn from_canonical(object_type: &str, canonical_bytes: &[u8]) -> Result<Self> {
        let leaf = hash_leaf(object_type, canonical_bytes);
        Ok(Self {
            schema: format!("donkai.{}.v1", object_type.to_lowercase()),
            algorithm: "SHA-256".to_string(),
            root: leaf,
            leaf_count: 1,
            canonical_byte_length: canonical_bytes.len() as u64,
        })
    }

    pub fn root_hex(&self) -> String {
        format!("0x{}", hex::encode(self.root))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryRecordCommitment {
    pub memory_id: Option<String>,
    #[serde(with = "hex_serde")]
    pub statement_root: [u8; 32],
    #[serde(with = "hex_serde")]
    pub context_root: [u8; 32],
    #[serde(with = "hex_serde")]
    pub consent_root: [u8; 32],
    #[serde(with = "hex_serde")]
    pub evidence_bundle_root: [u8; 32],
    #[serde(with = "hex_serde")]
    pub version_graph_root: [u8; 32],
    #[serde(with = "hex_serde")]
    pub bundle_root: [u8; 32],
    pub committed_at: String,
}

impl MemoryRecordCommitment {
    pub fn from_record(record: &MemoryRecord) -> Result<Self> {
        let stmt_canon = canonicalize(&record.statement)?;
        let ctx_canon = canonicalize(&record.context)?;
        let consent_canon = canonicalize(&record.consent)?;

        let stmt_root = hash_leaf("remembrance", stmt_canon.as_bytes());
        let ctx_root = hash_leaf("context", ctx_canon.as_bytes());
        let consent_root = hash_leaf("consent", consent_canon.as_bytes());

        let evidence_root = if let Some(ref ev) = record.evidence {
            let ev_canon = canonicalize(ev)?;
            hash_leaf("evidence", ev_canon.as_bytes())
        } else {
            [0u8; 32]
        };

        let ver_canon = canonicalize(&record.version)?;
        let ver_root = hash_leaf("version", ver_canon.as_bytes());

        // Construct 5-leaf Merkle tree for the bundle
        let leaves = vec![stmt_root, ctx_root, consent_root, evidence_root, ver_root];
        let tree = MerkleTree::build(leaves)?;
        let bundle_root = hash_bundle_root("memory_bundle", &tree.root);

        let now = "2026-08-29T06:36:00Z".to_string();

        Ok(Self {
            memory_id: record.evidence.as_ref().map(|e| e.memory_id.clone()),
            statement_root: stmt_root,
            context_root: ctx_root,
            consent_root,
            evidence_bundle_root: evidence_root,
            version_graph_root: ver_root,
            bundle_root,
            committed_at: now,
        })
    }
}
