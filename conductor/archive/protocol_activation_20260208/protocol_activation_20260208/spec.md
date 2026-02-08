# Specification - Protocol Activation: Codify Captain Methodology

## Overview
- **Track ID:** protocol_activation_20260208
- **Title:** Codify Captain Methodology in Workflow
- **Type:** Refactor/Feature
- **Description:** Complete rewrite of `conductor/workflow.md` to mandate the Captain v3.1 lifecycle, transforming it from a set of guidelines into a non-negotiable architectural contract.

## Functional Requirements
1. **Total Overhaul of `conductor/workflow.md`**: Discard legacy permissive language and replace it with the rigid Captain v3.1 lifecycle mandates.
2. **Mandatory Triage Phase**:
    - All tasks must begin with a complexity assessment.
    - Assessment dimensions must include: **Impact Surface** (dependencies), **Ambiguity Index** (1-10), and **Testing Tier** (T1/T2/T3).
3. **Safety-Hardened Execution (Snapshots)**:
    - Mandate a `CAPTAIN_SNAPSHOT` (git commit) exactly once **before every high-level Task** in the implementation plan.
4. **Ambiguity Protocol (Proactive Optioning)**:
    - Agents are forbidden from guessing.
    - Ambiguity must trigger an "Options Protocol" presenting A/B/C choices.
    - Each option must include: **Technical Trade-offs**, **Risk Profile** (breaking changes/security), and **Estimated Effort**.
5. **Recovery Protocol**:
    - Mandate the use of `RECOVERY_QUEUE.md` for tracking task failures, reasons, and rollback commands.
6. **Visibility (HUD)**:
    - Mandate the maintenance of `.captain-status.md` as a real-time display.
    - Updates must be triggered via **Lifecycle Hooks** (e.g., Task Start, Phase Complete, Error Triggered).

## Non-Functional Requirements
- **Rigidity**: The workflow must be written as a "law" rather than a suggestion (Source 74).
- **Consistency**: Terminology must align strictly with Captain v3.1 sources (e.g., "Sovereign Command", "Safety-Hardened").

## Acceptance Criteria
- [ ] `conductor/workflow.md` is rewritten to reflect all six mandates above.
- [ ] The document explicitly references mandatory files: `RECOVERY_QUEUE.md` and `.captain-status.md`.
- [ ] The Triage Phase section defines the specific assessment dimensions (Impact, Ambiguity, Testing Tier).
- [ ] The Snapshot section mandates Task-level granularity.
