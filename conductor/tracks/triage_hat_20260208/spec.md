# Specification: Triage Hat & Simple Path Routing

## 1. Overview
This track implements the "Brain" of Ralph Commander 3.1: the **Triage Hat**. Its purpose is to analyze incoming `UserTask` events and dynamically route them to the most efficient execution path based on complexity and risk.

## 2. Functional Requirements
- **Triage Hat (Rust):** A new event-driven component in the `ralph-core` or `ralph-cli` crates.
- **Complexity Analysis:** Heuristic or LLM-driven classification of tasks into `Simple` (e.g., typos, documentation, single-line fixes) or `Full` (e.g., new features, refactoring).
- **Routing Engine:** 
    - `Mode::Simple`: Routes directly to a streamlined implementation loop, bypassing `ArchitectHat`.
    - `Mode::Full`: Routes to the standard `ArchitectHat` for full planning.
- **Artifact Generation:** Triage decisions must be recorded and accessible to subsequent hats.

## 3. Technical Constraints
- **Event Bus:** Must utilize Ralph's existing `tokio::sync::broadcast` event system.
- **Safety:** Every triage decision must be logged to the machine-readable audit trail.
- **Integration:** Must be backward compatible with existing hats (Planner, Coder, Reviewer).

## 4. Acceptance Criteria
- Incoming tasks are correctly categorized in 90% of test cases.
- `Simple Path` tasks complete without generating a full `plan.md` or `spec.md`.
- `Full Path` tasks continue to use the established architectural protocol.
- All decisions are recorded in Git Notes as per the Workflow.
