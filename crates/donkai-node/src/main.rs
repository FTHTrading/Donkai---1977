//! Donkai Network — Layer-1 reference node (Chain ID 1977).
//! Boots the LPS-1, PQC, PolicyGuard, IPFS, and consensus subsystems and prints a status console.

use donkai_lps1::{Lps1MerkleTree, run_all_58_checks};
use donkai_consensus::{DonkaiValidator, ValidatorTier, AsinineFaultTolerance};
use donkai_policyguard::{AgentProposal, RiskTier, PolicyGuardEvaluator, SignedApproval, EvalOutcome};
use donkai_pqc::MlDsaKeypair;
use donkai_ipfs::{compute_raw_cidv1, compute_dagpb_cidv1, DEFAULT_KUBO_API};

const CHAIN_ID: u64 = 1977;

fn main() {
    println!("========================================================================");
    println!(" DONKAI NETWORK  |  Chain ID {}  |  Proof-of-Stubbornness + AFT 2/3", CHAIN_ID);
    println!("========================================================================\n");

    // 1. LPS-1 provenance
    let document = "\
Genesis Anchor: The Pet Rock (1975) taught markets that packaging beats utility.\n\n\
Second Anchor: Betamax (1976) had superior specs; VHS won on distribution.\n\n\
Third Anchor: Beanie Babies (1999) proved secondary-market speculation predates crypto by decades.";
    let tree = Lps1MerkleTree::build_from_document(document);
    let report = run_all_58_checks(&tree, document);

    println!("[LPS-1]        root       = 0x{}", hex::encode(tree.root_hash));
    println!("[LPS-1]        leaves     = {}", tree.leaves.len());
    println!("[LPS-1]        audit      = {} / {} checks passed  (all_passed = {})",
             report.passed_count, report.total, report.all_passed());
    if !report.all_passed() {
        for f in report.failures() {
            println!("[LPS-1]        FAIL #{}: {}  {}",
                     f.id, f.name, f.note.as_deref().unwrap_or(""));
        }
    }
    if let Some(proof) = tree.generate_proof(1) {
        let ok = Lps1MerkleTree::verify_inclusion(&tree.root_hash, &proof);
        println!("[LPS-1]        proof(#1)  = path_len={} verified={}", proof.audit_path.len(), ok);
    }
    println!();

    // 2. IPFS CIDs
    let payload = b"Donkai Genesis Manifest v0.1.0";
    println!("[IPFS]         raw CIDv1     = {}", compute_raw_cidv1(payload));
    println!("[IPFS]         dag-pb CIDv1  = {}", compute_dagpb_cidv1(payload));
    println!("[IPFS]         Kubo endpoint = {}  (not contacted)\n", DEFAULT_KUBO_API);

    // 3. PQC keygen + signature roundtrip
    let kp = MlDsaKeypair::generate();
    let msg = b"donkai:validator-attestation:block=1";
    let sig = kp.sign(msg);
    let verified = MlDsaKeypair::verify(&kp.public_key_bytes(), msg, &sig);
    println!("[PQC]          ml-dsa-87    pk_len={} sig_len={} verified={}\n",
             kp.public_key_bytes().len(), sig.len(), verified);

    // 4. Consensus + AFT
    let v = DonkaiValidator {
        node_id: "donkai1val_ultra_asinine_01".into(),
        staked_donk: 1_000_000u128 * 10u128.pow(18),
        blocks_unmoved: 50_000,
        tier: ValidatorTier::UltraAsinine,
    };
    let weight = v.calculate_vote_weight();
    let has_supermajority = AsinineFaultTolerance::has_supermajority(weight, weight);
    println!("[CONSENSUS]    node       = {}", v.node_id);
    println!("[CONSENSUS]    weight     = {}", weight);
    println!("[CONSENSUS]    2/3 BFT    = {}\n", has_supermajority);

    // 5. PolicyGuard with two real ML-DSA signatures over the same action
    let action = b"MINT_DONK_USD::amount=500000::to=treasury";
    let kp1 = MlDsaKeypair::generate();
    let kp2 = MlDsaKeypair::generate();
    let proposal = AgentProposal {
        agent_id: "agent_rwa_oracle".into(),
        proposed_action_bytes: action.to_vec(),
        risk_tier: RiskTier::D4Degenerate,
        approvals: vec![
            SignedApproval { signer_public_key: kp1.public_key_bytes(), signature: kp1.sign(action) },
            SignedApproval { signer_public_key: kp2.public_key_bytes(), signature: kp2.sign(action) },
        ],
    };
    let validator_set = vec![kp1.public_key_bytes(), kp2.public_key_bytes()];
    let outcome = PolicyGuardEvaluator::evaluate(&proposal, &validator_set, 0);
    let outcome_line = match &outcome {
        EvalOutcome::Approved { valid_signatures, required } =>
            format!("APPROVED  ({}-of-{} required)", valid_signatures, required),
        EvalOutcome::RejectedBelowQuorum { valid, required } =>
            format!("REJECTED  below quorum ({} valid / {} required)", valid, required),
        EvalOutcome::RejectedUnknownSigner { .. } =>
            "REJECTED  unknown signer".to_string(),
        EvalOutcome::RejectedInvalidSignature { .. } =>
            "REJECTED  invalid signature".to_string(),
        EvalOutcome::RejectedDuplicateSigner { .. } =>
            "REJECTED  duplicate signer".to_string(),
    };
    println!("[POLICYGUARD]  tier=D4Degenerate action=MINT_DONK_USD -> {}", outcome_line);
}
