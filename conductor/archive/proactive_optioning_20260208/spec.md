# Specification: Captain's Proactive Optioning (Human-in-the-Loop)

## 1. Overview
This track implements the **Proactive Optioning** protocol, a key component of Captain's "Safety-Hardened" methodology within Ralph Commander 3.1. It ensures that agents never resolve ambiguity autonomously. Instead, they must halt, present structured strategic choices (Options A, B, C), and wait for a "Sovereign Command" from the Human Commander.

## 2. Functional Requirements
### 2.1 Ambiguity Detection (Executor Hat)
- **Triggers:** The Executor must trigger the protocol when:
    1. It detects internal uncertainty (e.g., "I am unsure which implementation to choose").
    2. A critical tool fails (e.g., `replace`, `write_file`) with a technical blocker.
    3. A tool call violates a security or policy guardrail.
- **Goal:** Shift from autonomous implementation to "Mission Control" mode.

### 2.2 Proactive Optioning Protocol
- **Options Generation:** The agent must generate 2-3 distinct solutions (Options A, B, C) with:
    - Clear descriptions.
    - Pros/Cons for each path.
    - Technical impact summary.
- **Event Emission:** Emits a `human.interact` event with a JSON payload containing these structured options.

### 2.3 Human Bridge & Sovereign Input
- **System Halt:** The `EventLoop` must intercept the `human.interact` event and enter a **Paused** state.
- **CLI/TUI Display:** The system must render the Options A, B, C in a standardized, interactive menu.
- **Input Blocking:** The implementation loop must REMAIN BLOCKED until the user explicitly selects an option.

### 2.4 Resumption & Sovereignty
- **Instruction Injection:** Upon selection, the system must inject the user's choice as a mandatory **Override Instruction** in the next prompt (e.g., `[HUMAN DECISION: Use Option B]`).
- **Audit Logging:** Every options request and the resulting human decision must be logged to `RequestLog.md` for the permanent forensic trail.

## 3. Technical Constraints
- **Decoupling:** Proactive Optioning must utilize Ralph's native `human.interact` event bus for multi-interface compatibility (CLI, Dashboard, Telegram).
- **Hard Constraints:** Human decisions must be treated as immutable requirements by the agent.

## 4. Acceptance Criteria
### Ambiguity Handling
- **Given** an ambiguous requirement (e.g., "Implement a search function").
- **When** the agent is unsure whether to use a local or remote index.
- **Then** it must emit `human.interact` with structured Options A and B.

### Sovereign Blocking
- **Given** an active `human.interact` session.
- **When** checking the system state.
- **Then** the `EventLoop` must be halted and implementation progress must be zero.

### Decision Enforcement
- **Given** a human selection of "Option B".
- **When** the agent resumes work.
- **Then** its reasoning must explicitly reference the selection and execute the logic defined in that option.

## 5. Out of Scope
- Automated rollback of snapshots (Handled by the "Safety Net" track).
- Multi-user voting on options (Single Human Commander only).
