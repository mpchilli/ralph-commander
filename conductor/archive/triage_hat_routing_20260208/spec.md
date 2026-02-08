# Specification: Triage Hat & Simple Path Routing

## 1. Overview
This track implements the core "Scale-Adaptive Intelligence" of Ralph Commander 3.1. It introduces the **Triage Hat**, which uses LLM analysis to dynamically route incoming tasks to either a streamlined **Simple Path** or the standard **Full Planning Path**.

## 2. Functional Requirements
### 2.1 Triage Hat (Rust)
- **Task Analysis:** Analyzes the `UserTask` or initial prompt using an LLM call to assess risk, complexity, and scope.
- **Decision Engine:** Categorizes tasks into `Mode::Simple` (low-risk, minor fixes) or `Mode::Full` (features, refactors, high-risk).
- **Event Emission:** Publishes a `triage.decision` event containing:
    - `mode`: The selected routing path.
    - `reason`: The LLM's justification for the choice.
    - `confidence`: A score (0.0 - 1.0) indicating the LLM's certainty.

### 2.2 Dynamic Routing (Event Bus)
- **Subscriber Filtering:** The Event Bus must be updated to respect the `triage.decision`.
- **Full Path:** Routes to `ArchitectHat` for full planning (standard protocol).
- **Simple Path:** Bypasses `ArchitectHat` and routes directly to the **TEA Hat** for a minimalist strategy.

### 2.3 Minimalist TEA Integration
- **Simple Strategy:** For `Mode::Simple`, the TEA Hat generates a single, high-level verification requirement (e.g., "Must pass one targeted regression test") instead of a full test matrix.
- **Backpressure:** Enforces this minimal requirement before the Execution Loop can signal completion.

### 2.4 Simple Path Executor
- **Streamlined Loop:** A specialized implementation state that focuses on direct application of the fix followed immediately by the TEA-defined verification.

## 3. Technical Constraints
- **Auditability:** Every `triage.decision` must be formatted for ingestion into the Git Notes audit trail.
- **Decoupling:** The Triage Hat must not have hardcoded dependencies on the simple executor or the architect; it interacts solely through the event bus.

## 4. Acceptance Criteria
### LLM Classification
- **Given** a simple "Fix typo in README" request.
- **When** the Triage Hat analyzes the task.
- **Then** it must emit `mode: Simple` with a confidence score > 0.8.

### Path Bypassing
- **Given** a task triaged as `Simple`.
- **When** the implementation begins.
- **Then** the `ArchitectHat` must NOT be triggered, and the system should move directly to TEA for strategy.

### Audit Logging
- **Given** a completed task.
- **When** checking the Git Notes of the final commit.
- **Then** the triage decision, reason, and confidence score must be visible.

## 5. Out of Scope
- Implementation of complex, multi-step TEA strategies (Full Path only).
- Refactoring the existing Architect Hat logic.
