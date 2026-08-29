//! PolicyGuard — deterministic multi-signature adjudication for AI agent actions.
//! Rule: "Model proposes, PolicyGuard disposes." Every signature is cryptographically
//! verified against a whitelisted validator set; array length is not the gate.

use donkai_pqc::MlDsaKeypair;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RiskTier {
    /// Read-only, audit log indexing.
    D0Passive,
    /// Linear spot transfer.
    D1Linear,
    /// Rate-limited vault rebalancing.
    D2Bounded,
    /// Speculative dynamic yield routing.
    D3Speculative,
    /// Uncapped algorithmic minting.
    D4Degenerate,
    /// Core-parameter mutation.
    D5Systemic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedApproval {
    pub signer_public_key: Vec<u8>,
    pub signature: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentProposal {
    pub agent_id: String,
    /// Canonical bytes each signer signed.
    pub proposed_action_bytes: Vec<u8>,
    pub risk_tier: RiskTier,
    pub approvals: Vec<SignedApproval>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvalOutcome {
    Approved { valid_signatures: usize, required: usize },
    RejectedBelowQuorum { valid: usize, required: usize },
    RejectedUnknownSigner { signer: Vec<u8> },
    RejectedInvalidSignature { signer: Vec<u8> },
    RejectedDuplicateSigner { signer: Vec<u8> },
}

pub struct PolicyGuardEvaluator;

impl PolicyGuardEvaluator {
    /// Evaluate a proposal against a whitelisted validator set with a configurable base quorum.
    /// D0/D1 → no signatures required.
    /// D2/D3 → `base_quorum` valid signatures required.
    /// D4/D5 → `max(2, base_quorum)` valid signatures required (2-of-N minimum for high-risk).
    pub fn evaluate(
        proposal: &AgentProposal,
        validator_set: &[Vec<u8>],
        base_quorum: usize,
    ) -> EvalOutcome {
        let required = match proposal.risk_tier {
            RiskTier::D0Passive | RiskTier::D1Linear => 0,
            RiskTier::D2Bounded | RiskTier::D3Speculative => base_quorum,
            RiskTier::D4Degenerate | RiskTier::D5Systemic => base_quorum.max(2),
        };

        if required == 0 {
            return EvalOutcome::Approved { valid_signatures: 0, required: 0 };
        }

        let mut seen: Vec<&Vec<u8>> = Vec::new();
        let mut valid = 0usize;

        for approval in &proposal.approvals {
            if !validator_set.iter().any(|pk| pk == &approval.signer_public_key) {
                return EvalOutcome::RejectedUnknownSigner {
                    signer: approval.signer_public_key.clone(),
                };
            }
            if seen.iter().any(|pk| **pk == approval.signer_public_key) {
                return EvalOutcome::RejectedDuplicateSigner {
                    signer: approval.signer_public_key.clone(),
                };
            }
            if !MlDsaKeypair::verify(
                &approval.signer_public_key,
                &proposal.proposed_action_bytes,
                &approval.signature,
            ) {
                return EvalOutcome::RejectedInvalidSignature {
                    signer: approval.signer_public_key.clone(),
                };
            }
            seen.push(&approval.signer_public_key);
            valid += 1;
        }

        if valid >= required {
            EvalOutcome::Approved { valid_signatures: valid, required }
        } else {
            EvalOutcome::RejectedBelowQuorum { valid, required }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_signed(msg: &[u8]) -> (Vec<u8>, SignedApproval) {
        let kp = MlDsaKeypair::generate();
        let sig = kp.sign(msg);
        let pk = kp.public_key_bytes();
        (pk.clone(), SignedApproval { signer_public_key: pk, signature: sig })
    }

    #[test]
    fn d0_no_signatures_needed() {
        let proposal = AgentProposal {
            agent_id: "audit-agent".into(),
            proposed_action_bytes: b"read: balances".to_vec(),
            risk_tier: RiskTier::D0Passive,
            approvals: vec![],
        };
        let outcome = PolicyGuardEvaluator::evaluate(&proposal, &[], 0);
        assert!(matches!(outcome, EvalOutcome::Approved { .. }));
    }

    #[test]
    fn d4_requires_two_valid_signatures() {
        let action = b"MINT_DONK_USD amount=1_000_000";
        let (pk1, a1) = make_signed(action);
        let (pk2, a2) = make_signed(action);
        let proposal = AgentProposal {
            agent_id: "rwa-oracle".into(),
            proposed_action_bytes: action.to_vec(),
            risk_tier: RiskTier::D4Degenerate,
            approvals: vec![a1, a2],
        };
        let validators = vec![pk1, pk2];
        let outcome = PolicyGuardEvaluator::evaluate(&proposal, &validators, 0);
        match outcome {
            EvalOutcome::Approved { valid_signatures, required } => {
                assert_eq!(valid_signatures, 2);
                assert_eq!(required, 2);
            }
            other => panic!("expected Approved, got {:?}", other),
        }
    }

    #[test]
    fn d4_rejects_below_quorum() {
        let action = b"MINT_DONK_USD amount=1_000_000";
        let (pk1, a1) = make_signed(action);
        let proposal = AgentProposal {
            agent_id: "rwa-oracle".into(),
            proposed_action_bytes: action.to_vec(),
            risk_tier: RiskTier::D4Degenerate,
            approvals: vec![a1],
        };
        let validators = vec![pk1];
        let outcome = PolicyGuardEvaluator::evaluate(&proposal, &validators, 0);
        assert!(matches!(outcome,
            EvalOutcome::RejectedBelowQuorum { valid: 1, required: 2 }));
    }

    #[test]
    fn rejects_unknown_signer() {
        let action = b"mint";
        let (pk1, a1) = make_signed(action);
        let (_pk_stranger, a_stranger) = make_signed(action);
        let proposal = AgentProposal {
            agent_id: "rwa-oracle".into(),
            proposed_action_bytes: action.to_vec(),
            risk_tier: RiskTier::D4Degenerate,
            approvals: vec![a1, a_stranger],
        };
        let validators = vec![pk1];
        let outcome = PolicyGuardEvaluator::evaluate(&proposal, &validators, 0);
        assert!(matches!(outcome, EvalOutcome::RejectedUnknownSigner { .. }));
    }

    #[test]
    fn rejects_tampered_signature() {
        let action = b"mint";
        let (pk1, mut a1) = make_signed(action);
        if let Some(byte) = a1.signature.get_mut(0) { *byte ^= 0xFF; }
        let (pk2, a2) = make_signed(action);
        let proposal = AgentProposal {
            agent_id: "rwa-oracle".into(),
            proposed_action_bytes: action.to_vec(),
            risk_tier: RiskTier::D4Degenerate,
            approvals: vec![a1, a2],
        };
        let validators = vec![pk1, pk2];
        let outcome = PolicyGuardEvaluator::evaluate(&proposal, &validators, 0);
        assert!(matches!(outcome, EvalOutcome::RejectedInvalidSignature { .. }));
    }

    #[test]
    fn rejects_duplicate_signer() {
        let action = b"mint";
        let kp = MlDsaKeypair::generate();
        let sig = kp.sign(action);
        let pk = kp.public_key_bytes();
        let a1 = SignedApproval { signer_public_key: pk.clone(), signature: sig.clone() };
        let a2 = SignedApproval { signer_public_key: pk.clone(), signature: sig };
        let proposal = AgentProposal {
            agent_id: "rwa-oracle".into(),
            proposed_action_bytes: action.to_vec(),
            risk_tier: RiskTier::D4Degenerate,
            approvals: vec![a1, a2],
        };
        let validators = vec![pk];
        let outcome = PolicyGuardEvaluator::evaluate(&proposal, &validators, 0);
        assert!(matches!(outcome, EvalOutcome::RejectedDuplicateSigner { .. }));
    }
}
