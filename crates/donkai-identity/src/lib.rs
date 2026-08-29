use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CredentialType {
    #[serde(rename = "ethereum-eoa")]
    EthereumEoa,
    #[serde(rename = "webauthn-passkey")]
    WebauthnPasskey,
    #[serde(rename = "did-key")]
    DidKey,
    #[serde(rename = "human-pass-sbt")]
    HumanPassSbt,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdentityAttestation {
    pub credential_type: CredentialType,
    pub identifier_hash: String,
    pub proof_or_signature: String,
    pub issued_at: String,
    pub non_transferable: bool,
}

impl IdentityAttestation {
    pub fn new_passkey(id_hash: impl Into<String>, sig: impl Into<String>) -> Self {
        Self {
            credential_type: CredentialType::WebauthnPasskey,
            identifier_hash: id_hash.into(),
            proof_or_signature: sig.into(),
            issued_at: "2026-08-29T06:36:00Z".into(),
            non_transferable: true,
        }
    }
}
