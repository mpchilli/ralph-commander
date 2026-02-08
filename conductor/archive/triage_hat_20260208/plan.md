# Implementation Plan: Triage Hat & Simple Path

## Phase 1: Foundation & Triage Hat Scaffolding
- [x] Task: Define `TriageDecision` event and `RoutingMode` enum in `ralph-proto`.
- [x] Task: Create `TriageHat` struct and implement the `Hat` trait in `ralph-core`.
- [x] Task: Implement basic heuristic classifier (e.g., regex-based detection of "simple" keywords).
- [x] Task: Conductor - User Manual Verification 'Phase 1: Foundation' (Protocol in workflow.md)

## Phase 2: Routing Logic & Simple Path Implementation
- [x] Task: Update the central Event Router to respect `TriageDecision` for routing.
- [x] Task: Implement the `SimplePathExecutor` that combines coding and verification without planning.
- [x] Task: Write Tests: Verify that a "fix typo" request skips the Architect Hat.
- [x] Task: Write Tests: Verify that a "add new feature" request still triggers the Architect Hat.
- [x] Task: Conductor - User Manual Verification 'Phase 2: Routing & Simple Path' (Protocol in workflow.md)

## Phase 3: TEA Integration & Audit Trail
- [x] Task: Integrate TEA Hat to design minimal test strategies for the Simple Path.
- [x] Task: Implement Git Notes logging for the `TriageDecision`.
- [x] Task: Final end-to-end verification of the Scale-Adaptive workflow.
- [x] Task: Conductor - User Manual Verification 'Phase 3: Integration & Audit' (Protocol in workflow.md)
