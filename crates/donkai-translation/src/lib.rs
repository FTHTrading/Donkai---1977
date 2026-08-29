use donkai_lps1::canonicalize;
use donkai_lps1::merkle::hash_leaf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TranslationBundle {
    #[serde(rename = "type")]
    pub record_type: String,
    pub original_statement_root: String,
    pub target_language: String,
    pub translated_narrative: String,
    pub translator_identity: String,
    pub human_attestation: String,
    pub ai_assistance_notes: Option<String>,
    pub timestamp: String,
}

impl TranslationBundle {
    pub fn new_derivative(
        original_root: impl Into<String>,
        target_lang: impl Into<String>,
        translated_text: impl Into<String>,
        translator: impl Into<String>,
        attestation: impl Into<String>,
    ) -> Self {
        Self {
            record_type: "donkai.translation.v1".to_string(),
            original_statement_root: original_root.into(),
            target_language: target_lang.into(),
            translated_narrative: translated_text.into(),
            translator_identity: translator.into(),
            human_attestation: attestation.into(),
            ai_assistance_notes: None,
            timestamp: "2026-08-29T06:36:00Z".to_string(),
        }
    }

    pub fn compute_root(&self) -> Result<[u8; 32], donkai_lps1::Lps1Error> {
        let canon = canonicalize(self)?;
        Ok(hash_leaf("translation", canon.as_bytes()))
    }
}
