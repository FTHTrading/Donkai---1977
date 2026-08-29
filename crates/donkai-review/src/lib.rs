use donkai_lps1::canonicalize;
use donkai_lps1::merkle::hash_leaf;
use donkai_lps1::schema::{ReviewAssessment, SupportClassification};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewRubric {
    pub version: String,
    pub title: String,
    pub criteria: Vec<String>,
    pub required_contemporaneous_tier: bool,
    pub min_independent_witnesses: usize,
}

impl ReviewRubric {
    pub fn standard_v0_1() -> Self {
        Self {
            version: "v0.1.0".to_string(),
            title: "DONK AI Bounded Historical Support Rubric v0.1".to_string(),
            criteria: vec![
                "Submission Provenance Integrity".to_string(),
                "Contemporaneous Artifact Corroboration".to_string(),
                "Independent Blind Recall Threshold".to_string(),
                "Conflict & Inconsistency Analysis".to_string(),
            ],
            required_contemporaneous_tier: false,
            min_independent_witnesses: 3,
        }
    }
}

pub fn compute_assessment_root(assessment: &ReviewAssessment) -> Result<[u8; 32], donkai_lps1::Lps1Error> {
    let canon = canonicalize(assessment)?;
    Ok(hash_leaf("review_assessment", canon.as_bytes()))
}
