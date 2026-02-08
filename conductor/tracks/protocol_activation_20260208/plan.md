# Implementation Plan - Protocol Activation: Codify Captain Methodology

This plan outlines the total overhaul of the `conductor/workflow.md` file to codify the Captain v3.1 methodology as a non-negotiable architectural contract.

## Phase 1: Preparation & Drafting
- [x] Task: Conductor - Audit Existing Workflow and Sources
    - [x] Identify all legacy/permissive language in `conductor/workflow.md` that must be removed.
    - [x] Cross-reference Captain Sources (74, 15, 83, 87, 95, 82) to ensure terminology alignment.
- [x] Task: Draft "Triage Phase" Mandates
    - [x] Write the section requiring mandatory complexity assessment before planning.
    - [x] Define assessment criteria: Impact Surface, Ambiguity Index (1-10), and Testing Tier (T1/T2/T3).
- [x] Task: Draft "Safety-Hardened Execution" (Snapshots)
    - [x] Write the mandate for `CAPTAIN_SNAPSHOT` exactly once before every Task.
    - [x] Define the requirement for the snapshot hash before any tool call is permitted.
- [x] Task: Draft "Ambiguity & Recovery" Protocols
    - [x] Write the "Proactive Optioning" (Options A/B/C) mandate including Trade-offs, Risk Profile, and Effort.
    - [x] Write the "Recovery Protocol" section mandating `RECOVERY_QUEUE.md` usage for failures.
- [x] Task: Draft "Visibility & Audit" Requirements
    - [x] Write the mandate for `.captain-status.md` (HUD) updated via Lifecycle Hooks.
    - [x] Write the requirement for `RequestLog.md` as the central forensic audit aggregator.
- [x] Task: Conductor - User Manual Verification 'Preparation & Drafting' (Protocol in workflow.md)

## Phase 2: Finalization & Integration
- [x] Task: Assemble and Refine `conductor/workflow.md`
    - [x] Combine all drafted sections into a cohesive, "law-like" document.
    - [x] Ensure the tone is non-negotiable and safety-hardened.
- [ ] Task: Verify Cross-Document Consistency
    - [ ] Ensure `conductor/product.md` and `conductor/workflow.md` are perfectly aligned on Captain v3.1 principles.
- [ ] Task: Conductor - User Manual Verification 'Finalization & Integration' (Protocol in workflow.md)
