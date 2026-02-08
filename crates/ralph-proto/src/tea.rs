use serde::{Deserialize, Serialize};

/// Safety tiers for risk-based verification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SafetyTier {
    /// Tier 1: High Rigor (95% coverage, integration tests, zero lint warnings).
    Tier1,
    /// Tier 2: Standard (80% coverage, unit tests, zero error-level linting).
    Tier2,
    /// Tier 3: Minimal (Smoke test or single unit test, linting optional).
    Tier3,
}

/// A proportional, risk-based testing strategy defined by the TEA Hat.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestStrategy {
    /// The safety tier assigned to the task.
    pub tier: SafetyTier,
    /// Minimum test coverage required (percentage).
    pub min_coverage: f32,
    /// Test categories that must pass (e.g., "unit", "integration", "lint").
    pub mandatory_categories: Vec<String>,
    /// Specific conditions that block completion.
    pub hard_gates: Vec<String>,
    /// Human-readable reasoning for the strategy.
    pub reason: String,
}

impl TestStrategy {
    /// Creates a new default strategy for a given tier.
    pub fn for_tier(tier: SafetyTier, reason: impl Into<String>) -> Self {
        match tier {
            SafetyTier::Tier1 => Self {
                tier,
                min_coverage: 95.0,
                mandatory_categories: vec!["unit".into(), "integration".into(), "lint".into(), "security".into()],
                hard_gates: vec!["zero_lint_warnings".into(), "specs_verified".into()],
                reason: reason.into(),
            },
            SafetyTier::Tier2 => Self {
                tier,
                min_coverage: 80.0,
                mandatory_categories: vec!["unit".into(), "lint".into()],
                hard_gates: vec!["zero_lint_errors".into()],
                reason: reason.into(),
            },
            SafetyTier::Tier3 => Self {
                tier,
                min_coverage: 0.0,
                mandatory_categories: vec!["smoke".into()],
                hard_gates: vec![],
                reason: reason.into(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strategy_serialization() {
        let strategy = TestStrategy::for_tier(SafetyTier::Tier1, "Core logic change");
        let json = serde_json::to_string(&strategy).unwrap();
        let deserialized: TestStrategy = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.tier, SafetyTier::Tier1);
        assert_eq!(deserialized.min_coverage, 95.0);
        assert!(deserialized.mandatory_categories.contains(&"security".to_string()));
    }
}
