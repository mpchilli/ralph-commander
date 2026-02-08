# Specification: TEA Hat (Test Architect) for Risk-Based Verification

## 1. Overview
This track implements the **TEA Hat (Test Architect)**, a core component of the BMad methodology within Ralph Commander 3.1. The TEA Hat is responsible for analyzing the scope of work and defining a proportional, risk-based testing strategy. It ensures that the "Law of Backpressure" is applied with architectural rigor, matching the testing intensity to the task's complexity.

## 2. Functional Requirements
### 2.1 TEA Hat (Rust)
- **Context Analysis:** Analyzes the `plan.md` (Full Path) or the `triage.decision` (Simple Path) to identify target modules and complexity.
- **Strategy Generation:** Uses a **Heuristic Matrix** to map module types and complexity classes to specific safety tiers.
- **Event Emission:** Publishes a `test.strategy` event containing the defined quality gates.

### 2.2 Heuristic Matrix & Safety Tiers
- **Safety Tiers:**
    - **Tier 1 (High Rigor):** 95% coverage, mandatory integration/security tests, zero lint warnings (e.g., Auth, Core).
    - **Tier 2 (Standard):** 80% coverage, mandatory unit tests, zero error-level linting (e.g., Backend APIs, business logic).
    - **Tier 3 (Minimal):** Smoke test or single unit test verification, linting optional (e.g., Documentation, UI tweaks).
- **Mapping logic:** Maps `(Module, Complexity)` pairs to a Tier.

### 2.3 `test.strategy` Event Payload
- **Coverage Thresholds:** Specific numerical targets for the executor.
- **Mandatory Test Categories:** List of command suites that MUST be executed (e.g., `cargo test`, `npm test`).
- **Hard Release Gates:** Specific blockers (e.g., "No new TODOs", "Zero Clippy warnings").

### 2.4 Backpressure Enforcement
- **Execution Loop Integration:** The Execution Loop (Ralph) must consume the `test.strategy` and use it as a validation checklist.
- **Gate Blocking:** Prevents task completion events (`task.complete`) if any criteria in the current `test.strategy` are not met.

## 3. Technical Constraints
- **Methodological Rigor:** The Heuristic Matrix must be deterministic and configurable (e.g., via `tea_matrix.yml`).
- **Decoupling:** TEA Hat communicates solely through the event bus and does not modify implementation code.
- **Git Notes Audit:** The chosen Tier and Strategy must be serialized for attachment to Git Notes upon completion.

## 4. Acceptance Criteria
### Strategy Proportionality
- **Given** a task affecting the "Authentication" module.
- **When** TEA analyzes the plan.
- **Then** it must emit a Tier 1 (High Rigor) strategy with a 95% coverage requirement.

### Backpressure Enforcement
- **Given** a Tier 2 strategy requiring "Zero lint errors".
- **When** the Executor attempts to emit `build.done` with 1 lint error.
- **Then** the system must synthesize a `build.blocked` event citing the lint gate.

### Audit Integration
- **Given** a completed Simple Path task.
- **When** checking the commit metadata.
- **Then** the `test.strategy` summary (Tier 3) must be present in the Git Notes.

## 5. Out of Scope
- Automated fix generation for linting/test failures (Executor responsibility).
- Modification of existing project-level configuration files (TEA uses its own matrix).
