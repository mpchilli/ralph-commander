# Implementation Plan: Captain's Mission Control (Visibility & Status)

## Phase 1: Status Dashboard Infrastructure
- [x] Task: Implement the `StatusManager` component in `ralph-core`.
    - [x] Logic to collect data for Objective, Active Task, Health, and Safety.
    - [x] Logic to write and format the `.captain-status.md` markdown dashboard.
    - [x] Logic to serialize and write the `.captain-status.json` machine dashboard.
- [x] Task: Integrate `StatusManager` into the `EventLoop` iteration start hook.
- [x] Task: Write Tests: Verify that both status files are correctly updated with mocked loop state.
- [x] Task: Conductor - User Manual Verification 'Phase 1: Status Dashboards' (Protocol in workflow.md)

## Phase 2: Forensic Log Centralization
- [x] Task: Update the `AuditLogger` (from previous track) to handle the `RequestLog.md` table format.
- [x] Task: Implement "Aggregator" logic to subscribe to and centralize:
    - [x] `triage.decision` events.
    - [x] `test.strategy` events (TEA results).
    - [x] `human.interact` / `human.response` pairs.
    - [x] Safety halt/recovery events.
- [x] Task: Write Tests: Verify that a sequence of diverse events results in a correctly formatted markdown table in `RequestLog.md`.
- [x] Task: Conductor - User Manual Verification 'Phase 2: Forensic Logging' (Protocol in workflow.md)

## Phase 3: Captain's CLI Experience
- [x] Task: Modify the CLI output logic in `crates/ralph-cli/src/display.rs`.
    - [x] Implement the `print_captain_header(task_id, hat, risk_tier)` function.
- [x] Task: Ensure the `EventLoop` provides the necessary metadata (TaskID, Active Hat, Risk Tier) to the CLI layer at the start of each implementation step.
- [x] Task: Write Tests: Verify the header format and visibility in terminal-style output.
- [x] Task: Conductor - User Manual Verification 'Phase 3: CLI Visibility' (Protocol in workflow.md)

## Phase 4: Integration & Full Visibility
- [x] Task: Perform an end-to-end run of a "Simple Path" and "Full Path" task.
- [x] Task: Verify that `RequestLog.md`, `.captain-status.md`, and the CLI headers all synchronize accurately during the run.
- [x] Task: Conductor - User Manual Verification 'Phase 4: Final Integration' (Protocol in workflow.md)