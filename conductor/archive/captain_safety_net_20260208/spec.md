# Specification: Captain's Safety Net (Atomic Snapshots & Recovery Queue)

## 1. Overview
This track implements the "Safety-Hardened" core of Ralph Commander 3.1 by integrating Captain's safety protocols. It ensures 100% recoverability through automated **Atomic Snapshots** and enforces human sovereignty via a **Recovery Queue** that physically blocks the execution loop upon failure.

## 2. Functional Requirements
### 2.1 Atomic Snapshots (Pre-Execution)
- **Trigger:** The `EventLoop` must trigger a git commit immediately before dispatching any `task.start` event.
- **Scope:** Performs `git add -A` followed by a commit with the message format: `CAPTAIN_SNAPSHOT: Pre-execution for task <TASK_ID>`.
- **Goal:** Create a "save game" state that captures all staged and unstaged changes.

### 2.2 Recovery Queue (Failure Handling)
- **Artifact:** Creation of a `RECOVERY_QUEUE.md` file in the project root.
- **Failure Logic:** If a task fails (e.g., `build.failed`, `test.failed`), the system must:
    1. Write failure details to `RECOVERY_QUEUE.md` (Task ID, Title, Failure Reason, Last Snapshot SHA, and a Rollback Command like `git reset --hard <SHA>`).
    2. Immediately transition the loop to a **Halted** state.

### 2.3 Sovereignty Check (Blocking Logic)
- **Startup Check:** The `EventLoop` must check for the existence and content of `RECOVERY_QUEUE.md` on startup.
- **Iteration Check:** Before every iteration, the system must verify the queue is empty.
- **Blocking Behavior:** If the file is not empty, the system must enter an **Interactive Block**. The UI must display "RECOVERY REQUIRED" and refuse to execute any tool calls or implement any logic until the human manually clears the file.

### 2.4 Audit Logging
- **RequestLog.md:** Every halt event and recovery transition must be logged to a machine-readable `RequestLog.md` for permanent forensic audit.

## 3. Technical Constraints
- **Law of Reversibility:** No agent side-effects are permitted without a successful snapshot commit.
- **Decoupling:** The safety check must be a middleware-like component in the `EventLoop` that triggers regardless of the active "Hat."

## 4. Acceptance Criteria
### Atomic Snapshots
- **Given** a workspace with uncommitted changes.
- **When** a `task.start` event is emitted.
- **Then** a new git commit must exist with the `CAPTAIN_SNAPSHOT` prefix before the agent acts.

### Sovereignty Block
- **Given** a non-empty `RECOVERY_QUEUE.md`.
- **When** the user runs `ralph start`.
- **Then** the system must display "RECOVERY REQUIRED" and refuse to enter the execution phase.

### Recovery Flow
- **Given** a failed task recorded in the queue.
- **When** the human commander manually deletes the content of `RECOVERY_QUEUE.md`.
- **Then** the `EventLoop` must automatically detect the clearance and resume the sovereignty check.

## 5. Out of Scope
- Automated rollback execution (The human must execute the command provided in the queue).
- Integration with external issue trackers (GitHub/Jira).
