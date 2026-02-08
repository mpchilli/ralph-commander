# Implementation Plan: Captain's Proactive Optioning (Human-in-the-Loop)

## Phase 1: Data Structures & Proto Definitions
- [x] Task: Define the `OptionChoice` and `ProactiveOptions` structs in `ralph-proto`.
    - [x] Include fields for description, pros, cons, and impact.
- [x] Task: Update the `human.interact` event definition to handle the new structured JSON payload.
- [x] Task: Write Tests: Verify that structured options can be serialized and deserialized correctly.
- [x] Task: Conductor - User Manual Verification 'Phase 1: Foundation' (Protocol in workflow.md)

## Phase 2: Executor Hat Intelligence (Prompting)
- [x] Task: Update `crates/ralph-core/src/instructions.rs` to include the "Proactive Optioning" protocol in the default Executor instructions.
    - [x] Teach the agent how to format Options A, B, and C.
- [x] Task: Implement "Need Clarification" detection in the LLM response parser.
- [x] Task: Write Tests: Mock an ambiguous response and verify that the parser identifies it as a request for human interaction.
- [x] Task: Conductor - User Manual Verification 'Phase 2: Agent Intelligence' (Protocol in workflow.md)

## Phase 3: Human Bridge & Blocking UI
- [x] Task: Modify the `EventLoop` to identify structured `human.interact` events.
- [x] Task: Implement the interactive option menu in the CLI/TUI.
    - [x] Render Options A, B, and C with their pros/cons.
    - [x] **Tip:** Ensure the default behavior is **blocking**. The `stdin` capture must suspend the entire orchestration thread (Source 68) to prevent race conditions.
- [x] Task: Update `LoopState` to track the current active interaction session.
- [x] Task: Conductor - User Manual Verification 'Phase 3: Interactive Halt' (Protocol in workflow.md)

## Phase 4: Sovereignty Enforcement & Resumption
- [x] Task: Implement the "Instruction Injection" logic.
    - [x] Capture the user's choice and format it as a mandatory context block: `[HUMAN DECISION: <Choice>]`.
    - [x] Ensure this block is prepended to the next implementation prompt.
- [x] Task: Integrate logging: Write the options presented and the user's selection to `RequestLog.md`.
- [x] Task: Write Tests: Verify the end-to-end flow: Ambiguity -> Pause -> Human Choice -> Resumption with injected instruction.
- [x] Task: Conductor - User Manual Verification 'Phase 4: Decision Enforcement' (Protocol in workflow.md)