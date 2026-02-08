# Implementation Plan: Triage Hat & Simple Path Routing

## Phase 1: Foundation & Proto Definitions
- [x] Task: Define `TriageDecision` payload and `RoutingMode` enum in `ralph-proto`.
- [x] Task: Update `Event` struct to include an optional `triage_context` field or ensure it's compatible with the event bus metadata.
- [x] Task: Write Tests: Verify serialization/deserialization of `triage.decision` events.
- [x] Task: Conductor - User Manual Verification 'Phase 1: Foundation' (Protocol in workflow.md)

## Phase 2: Triage Hat Implementation
- [x] Task: Create the `TriageHat` struct in `ralph-core`.
- [x] Task: Implement the LLM Analysis logic:
    - [x] Define the prompt template for complexity analysis.
    - [x] **Verification:** Ensure the prompt includes a specific instruction to default to `Mode::Full` (Architect Path) if the confidence score is below the threshold (e.g., < 0.8).
    - [x] Implement the parsing of the LLM response into a `TriageDecision`.
- [x] Task: Write Tests: Mock LLM responses to verify correct classification into `Simple` vs `Full`.
- [x] Task: Conductor - User Manual Verification 'Phase 2: Triage Hat' (Protocol in workflow.md)

## Phase 3: Dynamic Routing & Event Bus Updates
- [x] Task: Modify the `EventBus` subscriber filtering logic to respect `RoutingMode`.
- [x] Task: Implement the "Simple Path" routing:
    - [x] Ensure `task.start` events in `Simple` mode bypass `ArchitectHat`.
    - [x] Ensure they trigger the `TEAHat` for a minimalist strategy.
- [x] Task: Write Tests: Verify that `ArchitectHat` is NOT triggered when `triage.decision` is `Simple`.
- [x] Task: Conductor - User Manual Verification 'Phase 3: Dynamic Routing' (Protocol in workflow.md)

## Phase 4: Minimalist TEA & Simple Path Executor
- [x] Task: Update `TEAHat` to detect `Simple` mode and emit a lightweight `verification.requirement`.
- [x] Task: Implement/Configure the streamlined Execution Loop for simple tasks.
- [x] Task: Write Tests: End-to-end "Simple Path" execution from triage to completion.
- [x] Task: Conductor - User Manual Verification 'Phase 4: Simple Path execution' (Protocol in workflow.md)

## Phase 5: Audit Trail & Git Notes
- [x] Task: Implement the Git Notes logger that subscribes to `triage.decision` and `task.complete`.
- [x] Task: Format the triage data into a machine-readable string for the `git notes add` command.
- [x] Task: Write Tests: Verify that `git notes show` displays the triage decision after a loop finishes.
- [x] Task: Conductor - User Manual Verification 'Phase 5: Audit Trail' (Protocol in workflow.md)