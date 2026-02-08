# Specification: Protocol Activation (Codify Captain Methodology in Workflow)

## 1. Overview
This track formally activates the Captain v3.1 operational protocol within the Ralph Commander monorepo. It involves a complete rewrite of `conductor/workflow.md` to mandate the "Safety-Hardened" lifecycle, transforming it from a set of guidelines into a non-negotiable architectural contract.

## 2. Functional Requirements
### 2.1 Triage Phase Codification
- **Contract:** Mandates that every objective start with an automated complexity assessment.
- **Criteria:** Assessment must cover Risk Level, Context Requirements, Verification Rigor (Tiers), and Human Dependency.
- **Routing:** Formalizes the choice between the "Simple Path" (Fast) and "Architect Path" (Deep).

### 2.2 Safety-Hardened Execution (Hard Guardrails)
- **The Law of Reversibility:** Mandates a `CAPTAIN_SNAPSHOT` commit before every implementation task.
- **Enforcement:** Defines a methodology violation as any autonomous action taken without a preceding snapshot hash in the system state.

### 2.3 Ambiguity Protocol (Proactive Optioning)
- **Contract:** Forbids agents from guessing during requirements ambiguity.
- **The Options Protocol:** Agents MUST present Options A, B, and C with pros/cons and await a "Sovereign Command" via the `human.interact` bridge.

### 2.4 Recovery Protocol (Human Sovereignty)
- **Contract:** Every task failure results in an immediate transition to the **Halted** state.
- **Artifact Control:** Human Commander owns the `RECOVERY_QUEUE.md` file. Resurrection of the loop is ONLY possible by manually clearing this artifact.

### 2.5 Visibility & Audit
- **The Pulse:** Mandates the maintenance of `.captain-status.md` as the live "Heads-Up Display".
- **Forensic Trail:** Centralizes all critical decisions (Triage, TEA, Human Choice) into the `RequestLog.md` aggregator.

## 3. Technical Constraints
- **Alignment:** The revised `workflow.md` must match the specific function names and event topics implemented in the preceding "Triage," "TEA," "Shield," and "Mission Control" tracks.
- **Language:** Must use strict architectural terminology (Triage, TEA Gate, Snapshot, Sovereign Command).

## 4. Acceptance Criteria
### Methodological Alignment
- **Given** the new `workflow.md`.
- **When** reviewed against the Captain v3.1 methodology.
- **Then** all core pillars (Scale-Adaptive, Safety-Hardened, Radical Transparency) must be explicitly represented as procedural steps.

### Operational Clarity
- **Given** a new agent joining the project.
- **When** reading the "Task Workflow" section.
- **Then** they must understand exactly when to take a snapshot, how to handle a blocker, and how to recover from failure.

### Artifact Discipline
- **Given** an entry in `RequestLog.md`.
- **When** checking the system logs.
- **Then** the format and information must match the definitions in the newly codified workflow.

## 5. Out of Scope
- Implementing the technical features (this track focuses on documentation and codification).
- Modifying project style guides (e.g., Rust/TypeScript formatting).
