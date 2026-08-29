use donkai_lps1::canonicalize;
use donkai_lps1::merkle::hash_leaf;
use donkai_lps1::schema::{EvidenceBundle, EvidenceItem, EvidenceTier, SourceClass, AiDisclosure};
use sha2::{Digest, Sha256};

pub fn hash_artifact_bytes(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("sha256:{}", hex::encode(hasher.finalize()))
}

pub fn compute_bundle_root(bundle: &EvidenceBundle) -> Result<[u8; 32], donkai_lps1::Lps1Error> {
    let canon = canonicalize(bundle)?;
    Ok(hash_leaf("evidence_bundle", canon.as_bytes()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evidence_hashing() {
        let dummy = b"photograph scan 1977 arcade token";
        let h = hash_artifact_bytes(dummy);
        assert!(h.starts_with("sha256:"));
    }
}
