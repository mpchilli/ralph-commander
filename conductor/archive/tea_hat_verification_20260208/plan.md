# Implementation Plan: TEA Hat (Test Architect) for Risk-Based Verification

## Phase 1: Foundation & Data Models
- [x] Task: Define the `TestStrategy` struct and `SafetyTier` enum in `ralph-proto`.
- [x] Task: Create the `test.strategy` event type and ensure it can carry thresholds, categories, and gates.
- [x] Task: Write Tests: Verify serialization/deserialization of the `test.strategy` payload.
- [x] Task: Conductor - User Manual Verification 'Phase 1: Foundation' (Protocol in workflow.md)

## Phase 2: TEA Hat Core & Heuristic Matrix
- [x] Task: Implement the `TEAHat` struct in `ralph-core`.
- [x] Task: Implement the Heuristic Matrix:
    - [x] Create a default `tea_matrix.yml` configuration defining mappings for core modules.
    - [x] Implement the logic to parse `plan.md` or `triage.decision` to extract module names and complexity.
    - [x] Implement the tier lookup logic.
- [x] Task: Write Tests: Verify that specific module inputs result in the correct safety tier and strategy.
- [x] Task: Conductor - User Manual Verification 'Phase 2: TEA Hat Core' (Protocol in workflow.md)

## Phase 3: Execution Loop Integration (Backpressure)
- [x] Task: Update the Execution Loop (Ralph) to listen for `test.strategy` events.
- [x] Task: Implement the "Gatekeeper" logic:
    - [x] Store the active strategy in the loop state.
    - [x] Intercept `build.done` events and validate them against the active strategy (coverage, linting, etc.).
    - [x] **Tip:** Ensure the `build.blocked` synthesis includes a clear, human-readable message (e.g., "Blocked: Coverage is 82%, required 95%") for the Ralph Web Dashboard.
- [x] Task: Write Tests: Verify that implementation is blocked when a strategy gate (e.g., coverage threshold) is not met.
- [x] Task: Conductor - User Manual Verification 'Phase 3: Backpressure Integration' (Protocol in workflow.md)

## Phase 4: Audit Trail & Git Notes Integration
- [x] Task: Extend the Git Notes logger to include TEA results (selected tier, passing/failing gates).
- [x] Task: Ensure the `task.complete` logic includes the final TEA verification summary.
- [x] Task: Write Tests: Verify that the final commit contains both triage and TEA metadata in the Git Notes.
- [x] Task: Conductor - User Manual Verification 'Phase 4: Audit & Finalization' (Protocol in workflow.md)
