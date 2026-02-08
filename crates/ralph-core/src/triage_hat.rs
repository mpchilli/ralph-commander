use ralph_proto::{RoutingMode, TriageDecision};
use tracing::info;

/// The Triage Hat analyzes incoming tasks and decides the routing path.
pub struct TriageHat;

impl TriageHat {
    /// Creates a new Triage Hat.
    pub fn new() -> Self {
        Self
    }

    /// Analyzes a task description and makes a triage decision.
    ///
    /// Currently uses a robust heuristic engine. In production, this can be
    /// extended to use a dedicated LLM call for complexity analysis.
    pub fn analyze(&self, task_description: &str) -> TriageDecision {
        info!("Analyzing task for triage: {}", task_description);

        let desc_lower = task_description.to_lowercase();

        // Simple heuristic: search for "simple" keywords or low complexity indicators
        let simple_keywords = [
            "typo", "documentation", "readme", "comment", "rename", "format", "indent",
            "spelling", "grammar", "license", "ignore", "changelog", "todo"
        ];

        let has_simple_kw = simple_keywords.iter().any(|&kw| {
            desc_lower.contains(kw)
        });

        // Heuristics for "full" planning path
        let full_keywords = [
            "feature", "implement", "refactor", "design", "architecture", "database",
            "api", "endpoint", "ui", "component", "integration", "test", "fix bug",
            "logic", "module", "system", "service", "rewrite", "optimize"
        ];

        let has_full_kw = full_keywords.iter().any(|&kw| {
            desc_lower.contains(kw)
        });

        // Complexity score (simulated)
        let mut confidence = 0.75;
        let is_very_short = task_description.len() < 40;
        let is_very_long = task_description.len() > 200;

        if has_simple_kw && !has_full_kw {
            confidence = 0.9;
            TriageDecision {
                mode: RoutingMode::Simple,
                reason: "Task contains 'simple' keywords and no complex indicators".to_string(),
                confidence,
            }
        } else if has_full_kw || is_very_long {
            confidence = 0.85;
            TriageDecision {
                mode: RoutingMode::Full,
                reason: if has_full_kw {
                    "Task contains complex keywords (e.g., feature, refactor)".to_string()
                } else {
                    "Task description is substantial, suggesting complexity".to_string()
                },
                confidence,
            }
        } else if is_very_short {
            confidence = 0.8;
            TriageDecision {
                mode: RoutingMode::Simple,
                reason: "Task description is very short and contains no complex indicators".to_string(),
                confidence,
            }
        } else {
            // Default to Full for safety if ambiguous
            TriageDecision {
                mode: RoutingMode::Full,
                reason: "Task is ambiguous; defaulting to Full Planning Path for safety".to_string(),
                confidence: 0.6,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triage_simple_typo() {
        let triage = TriageHat::new();
        let decision = triage.analyze("Fix a typo in the README.md");
        assert_eq!(decision.mode, RoutingMode::Simple);
    }

    #[test]
    fn test_triage_full_feature() {
        let triage = TriageHat::new();
        let decision = triage.analyze("Implement a new authentication system using OAuth2 and JWT tokens with refresh cycles.");
        assert_eq!(decision.mode, RoutingMode::Full);
    }
}
