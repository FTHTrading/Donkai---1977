use donkai_lps1::canonicalize;
use donkai_lps1::merkle::hash_leaf;
use donkai_lps1::schema::{CorroborationCommitment, DiscoveryContext, VisibilityMode};
use sha2::{Digest, Sha256};

pub struct BlindCorroborationEngine;

impl BlindCorroborationEngine {
    /// Step 3: Seals an independent recall record locally with a random nonce/salt
    pub fn seal_recall(narrative: &str, salt: &[u8]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(b"DONKAI:LPS1:BLIND_CORROBORATION:v1:");
        hasher.update(salt);
        hasher.update(narrative.as_bytes());
        let result = hasher.finalize();
        let mut out = [0u8; 32];
        out.copy_from_slice(&result);
        out
    }

    /// Creates a formal CorroborationCommitment object
    pub fn create_commitment(
        memory_id: impl Into<String>,
        discovery: DiscoveryContext,
        sealed_root: [u8; 32],
        credential_ref: Option<String>,
    ) -> CorroborationCommitment {
        CorroborationCommitment {
            record_type: "donkai.corroboration.v1".to_string(),
            memory_id: memory_id.into(),
            discovery_context: discovery,
            sealed_recall_root: format!("0x{}", hex::encode(sealed_root)),
            commitment_time: "2026-08-29T06:36:00Z".to_string(),
            visibility: VisibilityMode::AggregateOnly,
            eligibility_credential_ref: credential_ref,
        }
    }

    /// Verifies revealed testimony against the committed sealed root
    pub fn verify_reveal(narrative: &str, salt: &[u8], expected_sealed_root: &[u8; 32]) -> bool {
        let calculated = Self::seal_recall(narrative, salt);
        &calculated == expected_sealed_root
    }
}
