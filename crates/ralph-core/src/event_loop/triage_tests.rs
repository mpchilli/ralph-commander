#[cfg(test)]
mod tests {
    use super::super::*;
    use ralph_proto::{RoutingMode, TriageDecision, HatId, Event};

    #[test]
    fn test_triage_routing_simple() {
        let yaml = r#"
hats:
  planner:
    name: "Planner"
    triggers: ["task.start"]
  simple-executor:
    name: "Simple Executor"
    triggers: ["triage.decision"]
"#;
        let config: RalphConfig = serde_yaml::from_str(yaml).unwrap();
        let mut event_loop = EventLoop::new(config);

        // "Fix typo" should be triaged as Simple
        event_loop.initialize("Fix typo in README.md");

        // Verify triage decision event was published
        let events = event_loop.bus.peek_pending(&HatId::new("ralph")).unwrap();
        let has_decision = events.iter().any(|e| e.topic.as_str() == "triage.decision");
        assert!(has_decision);

        // Verify mode is Simple
        assert_eq!(event_loop.bus.routing_mode(), Some(RoutingMode::Simple));

        // In Simple mode, task.start should NOT be routed to planner
        let planner_pending = event_loop.bus.peek_pending(&HatId::new("planner"));
        assert!(planner_pending.is_none() || planner_pending.unwrap().is_empty());

        // Simple executor should receive triage.decision
        let simple_pending = event_loop.bus.peek_pending(&HatId::new("simple-executor"));
        assert!(simple_pending.is_some() && !simple_pending.unwrap().is_empty());
    }

    #[test]
    fn test_triage_routing_full() {
        let yaml = r#"
hats:
  planner:
    name: "Planner"
    triggers: ["task.start"]
  simple-executor:
    name: "Simple Executor"
    triggers: ["triage.decision"]
"#;
        let config: RalphConfig = serde_yaml::from_str(yaml).unwrap();
        let mut event_loop = EventLoop::new(config);

        // "Add feature" should be triaged as Full
        event_loop.initialize("Implement a complex new feature with database integration");

        // Verify mode is Full
        assert_eq!(event_loop.bus.routing_mode(), Some(RoutingMode::Full));

        // In Full mode, task.start SHOULD be routed to planner
        let planner_pending = event_loop.bus.peek_pending(&HatId::new("planner"));
        assert!(planner_pending.is_some() && !planner_pending.unwrap().is_empty());

        // Simple executor should NOT receive triage.decision (filtered out in Full mode)
        let simple_pending = event_loop.bus.peek_pending(&HatId::new("simple-executor"));
        assert!(simple_pending.is_none() || simple_pending.unwrap().is_empty());
    }

    #[test]
    fn test_simple_path_flow() {
        let yaml = r#"
hats:
  planner:
    name: "Planner"
    triggers: ["task.start"]
  tea:
    name: "TEA"
    triggers: ["triage.decision"]
    publishes: ["test.strategy"]
  simple-executor:
    name: "Simple Executor"
    triggers: ["test.strategy"]
"#;
        let config: RalphConfig = serde_yaml::from_str(yaml).unwrap();
        let mut event_loop = EventLoop::new(config);

        // 1. Triage a simple task
        event_loop.initialize("Fix typo");
        
        // 2. Verify TEA is triggered by triage.decision
        let tea_pending = event_loop.bus.peek_pending(&HatId::new("tea")).unwrap();
        assert!(tea_pending.iter().any(|e| e.topic.as_str() == "triage.decision"));

        // 3. Simulate TEA emitting test.strategy
        let strategy_event = Event::new("test.strategy", "Verify fix with one test").with_source("tea");
        event_loop.bus.publish(strategy_event);

        // 4. Verify Simple Executor is triggered by test.strategy
        let executor_pending = event_loop.bus.peek_pending(&HatId::new("simple-executor")).unwrap();
        assert!(executor_pending.iter().any(|e| e.topic.as_str() == "test.strategy"));

        // 5. Verify Planner was NOT triggered
        let planner_pending = event_loop.bus.peek_pending(&HatId::new("planner"));
        assert!(planner_pending.is_none() || planner_pending.unwrap().is_empty());
    }

    #[test]
    fn test_triage_audit_state() {
        let config = RalphConfig::default();
        let mut event_loop = EventLoop::new(config);

        event_loop.initialize("Fix typo in README");

        // Verify decision is stored in state
        let decision = event_loop.state().triage_decision.as_ref().expect("Decision should be in state");
        assert_eq!(decision.mode, RoutingMode::Simple);
        assert!(decision.reason.contains("keywords"));
    }
}
