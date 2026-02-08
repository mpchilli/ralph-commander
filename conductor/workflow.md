# Workflow: Ralph Commander 3.1

## 1. Project Inception & Triage (BMad)
Every new request must first pass through the **Triage Hat**.
- **Simple Path:** For bug fixes or minor tweaks, route directly to TEA for test design then to the Execution Loop.
- **Full Planning Path:** For new features or architectural changes, route to the **Architect Hat** for a full specification and plan.

## 2. Planning & Strategy
- **Architect Hat:** Generates `spec.md` and `plan.md`.
- **TEA Hat (Test Architect):** Analyzes the plan and defines the **Risk-Based Testing Strategy**. TEA establishes the specific coverage requirements and release gates for the task based on its criticality.

## 3. Ambiguity Protocol (Captain)
At any point during planning or implementation, if an agent encounters technical blockers or architectural ambiguity, it **must** halt.
- Present structured **A/B/C Options** to the human commander.
- Wait for explicit approval before proceeding.

## 4. Execution Loop (Ralph)
- **Atomic Snapshot:** Before any tool execution, the system must create a `CAPTAIN_SNAPSHOT` git commit.
- **Implementation:** The agent iterates through the plan.
- **Backpressure:** The TEA Hat applies "Backpressure," rejecting any code that does not meet the task-specific test strategy.
- **Recovery:** If a task enters a failed state, it is moved to the **Recovery Queue** for human intervention.

## 5. Audit & Release Gate
- **Git Notes:** Upon completion of a task, a machine-readable summary (triage decisions, TEA results) is attached to the commit via Git Notes.
- **Sovereignty Protocol:** Upon passing final TEA verification, the system triggers a release gate. The user must explicitly select the semantic version increment (Patch/Minor/Major) before the release is tagged.

## 6. Phase Completion Verification and Checkpointing Protocol
At the end of every Phase, the system MUST halt for **Manual User Verification**.
- Protocol: The user reviews the Phase artifacts.
- Action: If verified, a git checkpoint is created. If rejected, the system reverts to the Last Atomic Snapshot.
