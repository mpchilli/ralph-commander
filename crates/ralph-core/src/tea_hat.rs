use ralph_proto::{SafetyTier, TestStrategy};
use tracing::info;
use std::collections::HashMap;

/// The TEA Hat (Test Architect) designs risk-based testing strategies.
pub struct TEAHat {
    /// Heuristic matrix for mapping modules to tiers.
    matrix: HashMap<String, SafetyTier>,
}

impl TEAHat {
    /// Creates a new TEA Hat with default heuristic matrix.
    pub fn new() -> Self {
        let mut matrix = HashMap::new();
        // Tier 1: High Rigor
        matrix.insert("auth".into(), SafetyTier::Tier1);
        matrix.insert("core".into(), SafetyTier::Tier1);
        matrix.insert("security".into(), SafetyTier::Tier1);
        matrix.insert("database".into(), SafetyTier::Tier1);
        
        // Tier 2: Standard (Default)
        matrix.insert("api".into(), SafetyTier::Tier2);
        matrix.insert("backend".into(), SafetyTier::Tier2);
        matrix.insert("logic".into(), SafetyTier::Tier2);
        
        // Tier 3: Minimal
        matrix.insert("docs".into(), SafetyTier::Tier3);
        matrix.insert("readme".into(), SafetyTier::Tier3);
        matrix.insert("ui".into(), SafetyTier::Tier3);
        matrix.insert("frontend".into(), SafetyTier::Tier3);

        Self { matrix }
    }

    /// Determines the testing strategy based on the plan or triage decision.
    pub fn design_strategy(&self, context: &str) -> TestStrategy {
        info!("Designing risk-based test strategy for context: {}", context);

        let context_lower = context.to_lowercase();
        
        // Identify the tier based on keywords in context
        let mut selected_tier = SafetyTier::Tier2; // Default to Standard
        
        // Check for high rigor indicators
        for (module, tier) in &self.matrix {
            if context_lower.contains(module) {
                if *tier == SafetyTier::Tier1 {
                    selected_tier = SafetyTier::Tier1;
                    break;
                }
                if *tier == SafetyTier::Tier3 && selected_tier == SafetyTier::Tier2 {
                    selected_tier = SafetyTier::Tier3;
                }
            }
        }

        // Refine based on explicit complexity mentions
        if context_lower.contains("simple") || context_lower.contains("minor") {
            selected_tier = SafetyTier::Tier3;
        } else if context_lower.contains("complex") || context_lower.contains("refactor") {
            selected_tier = SafetyTier::Tier1;
        }

        TestStrategy::for_tier(selected_tier, format!("Determined tier {:?} based on context analysis", selected_tier))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_design_strategy_high_rigor() {
        let tea = TEAHat::new();
        let strategy = tea.design_strategy("Update authentication logic for JWT tokens");
        assert_eq!(strategy.tier, SafetyTier::Tier1);
        assert_eq!(strategy.min_coverage, 95.0);
    }

    #[test]
    fn test_design_strategy_minimal() {
        let tea = TEAHat::new();
        let strategy = tea.design_strategy("Fix typo in README.md");
        assert_eq!(strategy.tier, SafetyTier::Tier3);
    }
}
