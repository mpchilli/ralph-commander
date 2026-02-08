use serde::{Deserialize, Serialize};

/// A single option choice for proactive optioning.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OptionChoice {
    /// The label for the option (e.g., "A", "B", "C").
    pub id: String,
    /// A clear description of the solution.
    pub description: String,
    /// Pros of this choice.
    pub pros: Vec<String>,
    /// Cons of this choice.
    pub cons: Vec<String>,
    /// Technical impact summary.
    pub impact: String,
}

/// A set of options presented to the user during an ambiguity event.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProactiveOptions {
    /// The specific question or ambiguity being addressed.
    pub question: String,
    /// The list of options available to the user.
    pub options: Vec<OptionChoice>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_options_serialization() {
        let options = ProactiveOptions {
            question: "Should we use UUID or Integer for IDs?".to_string(),
            options: vec![
                OptionChoice {
                    id: "A".to_string(),
                    description: "Use UUIDs".to_string(),
                    pros: vec!["Globally unique".to_string()],
                    cons: vec!["Storage overhead".to_string()],
                    impact: "Requires schema migration".to_string(),
                },
                OptionChoice {
                    id: "B".to_string(),
                    description: "Use Integers".to_string(),
                    pros: vec!["Efficient".to_string()],
                    cons: vec!["Not globally unique".to_string()],
                    impact: "Standard pattern".to_string(),
                },
            ],
        };

        let json = serde_json::to_string(&options).unwrap();
        let deserialized: ProactiveOptions = serde_json::from_str(&json).unwrap();
        assert_eq!(options, deserialized);
    }
}
