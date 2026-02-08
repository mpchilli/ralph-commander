# Workflow: Ralph Commander 3.1 (Captain Protocol)

This document defines the mandatory operational lifecycle for all autonomous agents in the Ralph Commander environment. Adherence to these protocols is non-negotiable and enforced by the system's "Safety-Hardened" core.

---

## 1. Project Inception & Triage (Scale-Adaptive)
Every new objective MUST pass through the **Triage Phase** before any planning or implementation occurs.
- **Complexity Assessment:** An automated LLM analysis evaluates the task across the following mandatory dimensions:
    - **Impact Surface:** Analysis of core systems, dependencies, and files touched.
    - **Ambiguity Index:** A metric (1-10) predicting the likelihood of hitting the "Ambiguity Protocol".
    - **Testing Tier:** Pre-selection of the Safety Tier (T1/T2/T3) required for verification based on risk.
    - **Context Requirements:** Depth of architectural knowledge needed.
- **Routing Decision:**
    - **Simple Path:** For minor fixes (typos, docs, UI tweaks) with low Impact and Ambiguity. Routes directly to TEA then to Execution.
    - **Architect Path (Full):** For features, refactors, or high-risk changes. Routes to the **Architect Hat** for full specification.

## 2. Planning & Strategy (Methodological Rigor)
- **Architect Hat:** Generates a comprehensive `spec.md` and `plan.md`. The plan must be an atomic breakdown of verifiable tasks.
- **TEA Hat (Test Architect):** Analyzes the plan and defines the **Risk-Based Testing Strategy**.
    - **Safety Tiers:** TEA assigns a Tier (1, 2, or 3) based on the Heuristic Matrix.
    - **Quality Gates:** Emits a `test.strategy` event defining mandatory coverage, test categories, and hard release gates (e.g., zero lint warnings).

## 3. Execution Loop (The Law of Reversibility)
The implementation phase operates under a "Save-Game" philosophy.
- **CAPTAIN_SNAPSHOT:** The system MUST trigger an atomic git commit (staging all changes) exactly once **BEFORE every high-level Task** defined in the implementation plan. No autonomous tool call is permitted without a preceding snapshot hash.
- **Backpressure Enforcement:** The system automatically validates all `build.done` attempts against the active `test.strategy`. If a quality gate is violated, the loop synthesizes a `build.blocked` event and refuses to progress.

## 4. Ambiguity Protocol (Proactive Optioning)
Agents are strictly FORBIDDEN from guessing or making strategic assumptions.
- **The Options Protocol:** If an agent encounters architectural ambiguity, technical blockers, or safety guardrail violations, it MUST immediately halt implementation.
- **Sovereign Command:** The agent emits a `human.interact` event presenting structured **Options A, B, and C**.
- **Option Requirements:** Each option MUST include:
    - **Technical Trade-offs:** Comparison of performance, complexity, and maintainability.
    - **Risk Profile:** Explicit potential for breaking changes or security regressions.
    - **Estimated Effort:** Implementation cost in time/complexity.
- **Decision Enforcement:** The user's choice is injected as a mandatory **Override Instruction** in the next prompt.

## 5. Recovery Protocol (Human Sovereignty)
Failure is handled as a strategic pivot, not a crash.
- **The Halted State:** Upon task failure, the system MUST write the Task ID, Failure Reason, and a **Rollback Command** (referencing the last `CAPTAIN_SNAPSHOT`) to `RECOVERY_QUEUE.md`.
- **Sovereignty Block:** The EventLoop enters a physical block. Implementation is disabled until the Human Commander manually reviews the failure and **clears the RECOVERY_QUEUE.md artifact**.

## 6. Visibility & Audit (Radical Transparency)
- **The HUD:** The system MUST maintain a real-time "Heads-Up Display" in `.captain-status.md`, showing the Current Objective, Active Hat, Risk Tier, and Safety Status.
- **Lifecycle Hooks:** Updates to the HUD MUST be triggered at specific lifecycle transitions: Task Start, Phase Complete, and Error Triggered.
- **Forensic Audit:** All critical events (Triage, TEA, Human Choice, Snapshots, Halts) are centralized in the linear, append-only **RequestLog.md** aggregator.

## 7. Phase Completion Verification
At the end of every major Phase defined in a track's Implementation Plan, the system MUST halt for **Manual User Verification**.
- **Checkpointing:** If the user verifies the phase artifacts, a git checkpoint is created.
- **Reversion:** If rejected, the system provides an option to revert to the Last Atomic Snapshot.
