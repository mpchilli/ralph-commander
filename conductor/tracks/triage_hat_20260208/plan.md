# Implementation Plan: Triage Hat & Simple Path

## Phase 1: Foundation & Triage Hat Scaffolding
- [ ] Task: Define `TriageDecision` event and `RoutingMode` enum in `ralph-proto`.
- [ ] Task: Create `TriageHat` struct and implement the `Hat` trait in `ralph-core`.
- [ ] Task: Implement basic heuristic classifier (e.g., regex-based detection of "simple" keywords).
- [ ] Task: Conductor - User Manual Verification 'Phase 1: Foundation' (Protocol in workflow.md)

## Phase 2: Routing Logic & Simple Path Implementation
- [ ] Task: Update the central Event Router to respect `TriageDecision` for routing.
- [ ] Task: Implement the `SimplePathExecutor` that combines coding and verification without planning.
- [ ] Task: Write Tests: Verify that a "fix typo" request skips the Architect Hat.
- [ ] Task: Write Tests: Verify that a "add new feature" request still triggers the Architect Hat.
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Routing & Simple Path' (Protocol in workflow.md)

## Phase 3: TEA Integration & Audit Trail
- [ ] Task: Integrate TEA Hat to design minimal test strategies for the Simple Path.
- [ ] Task: Implement Git Notes logging for the `TriageDecision`.
- [ ] Task: Final end-to-end verification of the Scale-Adaptive workflow.
- [ ] Task: Conductor - User Manual Verification 'Phase 3: Integration & Audit' (Protocol in workflow.md)
