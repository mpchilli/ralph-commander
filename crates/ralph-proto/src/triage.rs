use serde::{Deserialize, Serialize};

/// The routing mode decided by the Triage Hat.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RoutingMode {
    /// Simple path: direct implementation without full planning.
    Simple,
    /// Full path: standard architectural planning (Architect Hat).
    Full,
}

/// The result of a triage analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriageDecision {
    /// The complexity class assigned to the task.
    pub mode: RoutingMode,
    /// Human-readable reason for the decision.
    pub reason: String,
    /// Confidence score (0.0 to 1.0).
    pub confidence: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialization() {
        let decision = TriageDecision {
            mode: RoutingMode::Simple,
            reason: "Small fix".to_string(),
            confidence: 0.95,
        };
        let json = serde_json::to_string(&decision).unwrap();
        let deserialized: TriageDecision = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.mode, RoutingMode::Simple);
        assert_eq!(deserialized.reason, "Small fix");
        assert_eq!(deserialized.confidence, 0.95);
    }

    #[test]
    fn test_routing_mode_serialization() {
        let simple = RoutingMode::Simple;
        let full = RoutingMode::Full;
        
        assert_eq!(serde_json::to_string(&simple).unwrap(), "\"Simple\"");
        assert_eq!(serde_json::to_string(&full).unwrap(), "\"Full\"");
    }
}
