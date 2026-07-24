//! Proof-of-Stubbornness + Asinine Fault Tolerance (AFT).
//! The "asinine" name is satirical; the math is strict 2/3 BFT.

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ValidatorTier {
    /// Sold within 30 days — voting weight = 0
    PaperHand,
    /// Held through 30% drawdown — voting weight = 1x
    Stubborn,
    /// Held through 90% drawdown — voting weight = 10x
    UltraAsinine,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DonkaiValidator {
    pub node_id: String,
    pub staked_donk: u128,
    pub blocks_unmoved: u64,
    pub tier: ValidatorTier,
}

impl DonkaiValidator {
    /// weight = staked * blocks_unmoved * tier_multiplier, saturating on overflow.
    pub fn calculate_vote_weight(&self) -> u128 {
        let multiplier: u128 = match self.tier {
            ValidatorTier::PaperHand    => return 0,
            ValidatorTier::Stubborn     => 1,
            ValidatorTier::UltraAsinine => 10,
        };
        self.staked_donk
            .saturating_mul(self.blocks_unmoved as u128)
            .saturating_mul(multiplier)
    }
}

pub struct AsinineFaultTolerance;

impl AsinineFaultTolerance {
    /// Strict 2/3 supermajority: `agreeing * 3 >= total * 2`.
    /// Uses saturating multiplication so pathological weights cannot panic the network.
    pub fn has_supermajority(total_weight: u128, agreeing_weight: u128) -> bool {
        if total_weight == 0 { return false; }
        let lhs = agreeing_weight.saturating_mul(3);
        let rhs = total_weight.saturating_mul(2);
        lhs >= rhs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paperhand_weight_is_zero() {
        let v = DonkaiValidator {
            node_id: "p".into(), staked_donk: 1_000_000, blocks_unmoved: 500,
            tier: ValidatorTier::PaperHand,
        };
        assert_eq!(v.calculate_vote_weight(), 0);
    }

    #[test]
    fn ultra_asinine_multiplies_by_ten() {
        let v = DonkaiValidator {
            node_id: "u".into(), staked_donk: 100, blocks_unmoved: 10,
            tier: ValidatorTier::UltraAsinine,
        };
        assert_eq!(v.calculate_vote_weight(), 10_000);
    }

    #[test]
    fn aft_requires_two_thirds_supermajority() {
        assert!( AsinineFaultTolerance::has_supermajority(300, 200));  // exactly 2/3
        assert!( AsinineFaultTolerance::has_supermajority(300, 201));  // above 2/3
        assert!(!AsinineFaultTolerance::has_supermajority(300, 199));  // just below
        assert!(!AsinineFaultTolerance::has_supermajority(300, 100));  // 1/3
        assert!(!AsinineFaultTolerance::has_supermajority(  0,   0));  // empty network
    }

    #[test]
    fn overflow_saturates_gracefully() {
        let huge_stake = u128::MAX;
        let huge_blocks = u64::MAX;
        // Must not panic
        let _ = AsinineFaultTolerance::has_supermajority(huge_stake, huge_stake);
        let v = DonkaiValidator {
            node_id: "big".into(), staked_donk: huge_stake, blocks_unmoved: huge_blocks,
            tier: ValidatorTier::UltraAsinine,
        };
        let _ = v.calculate_vote_weight();
    }
}
