# Specification: Captain's Mission Control (Visibility & Status)

## 1. Overview
This track implements the "Mission Control" layer of Ralph Commander 3.1. It provides real-time visibility into the system's internal state through a Markdown dashboard, a machine-readable JSON status file, a centralized forensic log, and high-visibility CLI headers.

## 2. Functional Requirements
### 2.1 `.captain-status.md` (Human Dashboard)
- **Update Frequency:** Must be updated at the start of every orchestration iteration.
- **Content Sections:**
    - **Current Objective:** High-level goal of the run.
    - **Active Task:** ID, Title, and current sub-step.
    - **Orchestration Health:** Iteration count (Current/Max), Elapsed Time, and Total Cost (USD).
    - **Safety HUD:** Last Atomic Snapshot SHA and Recovery Queue Status (OK/HALTED).

### 2.2 `.captain-status.json` (Machine Dashboard)
- **Goal:** Provide a machine-readable mirror of the Markdown dashboard for IDE extensions and external monitors.
- **Schema:** Standardized JSON object containing all fields from the human dashboard.

### 2.3 `RequestLog.md` Aggregator
- **Structure:** Unified Append-Only Table.
- **Columns:** `Timestamp | Event Type | Correlation ID | Details`.
- **Centralization:** Automatically captures and appends:
    - Triage Decisions (Mode, Reason, Confidence).
    - TEA Gate Results (Safety Tier, Strategy).
    - Human Decisions (Question, Selected Option).
    - Safety Halts (Failure Reason, Rollback Command).

### 2.4 Captain's CLI Header
- **Format:** `[TaskID] | [Hat] | [Risk Tier]` (e.g., `task-001 | Executor | Tier 2`).
- **Placement:** Printed at the start of each implementation step in the CLI output.

## 3. Technical Constraints
- **Performance:** Status updates must be non-blocking and have negligible impact on iteration time.
- **Persistence:** Logs must be append-only to preserve forensic integrity.
- **Decoupling:** The status manager should be an observer/middleware component within the `EventLoop`.

## 4. Acceptance Criteria
### Real-Time Dashboard
- **Given** an active orchestration loop.
- **When** an iteration starts.
- **Then** `.captain-status.md` and `.captain-status.json` must be updated with the latest state.

### Forensic Centralization
- **Given** a human interaction event followed by a task completion.
- **When** checking `RequestLog.md`.
- **Then** both the interaction and the completion must be recorded as linear rows in the table.

### At-a-Glance Visibility
- **Given** the start of a new task.
- **When** viewing the CLI output.
- **Then** the "Captain's Header" must be clearly visible before the agent's implementation logs.

## 5. Out of Scope
- Implementation of the Ralph Web Dashboard (this track focuses on local file-based visibility).
- Historical trending of cost/time metrics (handled by `LoopHistory`).
