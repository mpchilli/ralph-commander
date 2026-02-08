# Product Guidelines: Ralph Commander

## Command Protocol: Structured Sovereignty & Total Auditability
- **Structured Interaction:** Communication must never be open-ended during blockers. The system must use "Proactive Optioning" (presenting clear A/B/C paths) via the Telegram/CLI interface to resolve ambiguity, ensuring the human remains the ultimate authority.
- **Terminology Discipline:** Documentation, logs, and agent responses must use strict architectural terms (Triage, TEA Gate, Snapshot) rather than generic AI descriptions. This ensures the user always knows exactly which "Hat" is currently active.
- **Immutable Audit:** Every triage decision, test strategy, and recovery event must be committed to a machine-readable log (`RequestLog.md`) and attached to Git Commits, creating a permanent forensic trail of autonomous activity.

## Adaptive Command Center: Dynamic Fidelity with Sovereign Interrupts
- **Dynamic Fidelity:** The interface density must shift based on the active workflow. 
    - **Simple Path:** Minimalist view showing "Fixing -> Verifying".
    - **Full Planning Path:** High-density "Mission Control" view displaying the "Architect's Blueprint," "TEA Test Matrix," and "Kanban Board".
- **Sovereign Interrupts:** The "Ambiguity Protocol" is a Blocking Modal. Whether in CLI, Web interface, or Telegram, the interface must lock and refuse new implementation commands until the user selects a structured Option (A, B, or C).
- **Safety HUD:** A persistent, immutable "Heads-Up Display" visible in all modes must show the Last Atomic Snapshot hash and the Recovery Queue health, providing constant reassurance of system reversibility.

## The Iron Triangle: Decoupled, Gated, and Reversible Architecture
- **Law of Decoupling:** Every functional module (Triage, Architect, TEA) must operate as an isolated "Hat" consuming and emitting distinct events, preventing monolithic logic sprawl and ensuring modularity.
- **Law of Reversibility:** No side-effect (tool use) is permitted without a preceding Atomic Git Snapshot. The system must "save game" before every agent move.
- **Law of Backpressure:** Quality is enforced continuously. The TEA Hat establishes test gates that apply "backpressure," physically rejecting code merges or task completions that do not adhere to the defined risk-based testing strategy.
