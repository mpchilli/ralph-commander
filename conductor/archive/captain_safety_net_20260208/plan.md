# Implementation Plan: Captain's Safety Net (Atomic Snapshots & Recovery Queue)

## Phase 1: Atomic Snapshot Infrastructure
- [x] Task: Create a utility in `crates/ralph-core/src/git_ops.rs` for creating `CAPTAIN_SNAPSHOT` commits.
    - [x] Implement `create_atomic_snapshot(task_id: &str)` using `git add -A` and `git commit`.
- [x] Task: Integrate the snapshot trigger into `EventLoop::initialize_with_topic`.
    - [x] Ensure the snapshot happens immediately before the `task.start` event is published.
- [x] Task: Write Tests: Verify that a git commit is created with the correct prefix when a loop starts.
- [x] Task: Conductor - User Manual Verification 'Phase 1: Atomic Snapshots' (Protocol in workflow.md)

## Phase 2: Recovery Queue & Halted State
- [x] Task: Implement the `RecoveryQueue` module in `ralph-core`.
    - [x] Logic to create and append to `RECOVERY_QUEUE.md`.
    - [x] Logic to check if the file is non-empty.
- [x] Task: Update the `EventLoop` logic to handle task failures.
    - [x] On failure, write the Task ID, Reason, and Rollback Command (retrieving the Last Snapshot SHA) to `RECOVERY_QUEUE.md`.
    - [x] Update `LoopState` to include a `Halted` variant or boolean flag.
- [x] Task: Write Tests: Verify that a task failure correctly populates the markdown file.
- [x] Task: Conductor - User Manual Verification 'Phase 2: Recovery Queue' (Protocol in workflow.md)

## Phase 3: Sovereignty Check & Interactive Block
- [x] Task: Implement the startup sovereignty check in `EventLoop::new` and `EventLoop::initialize`.
    - [x] If `RECOVERY_QUEUE.md` is detected and non-empty, refuse to start.
- [x] Task: Implement the iteration-level block.
    - [x] Modify the main loop to check the queue before every iteration.
    - [x] If blocked, display the "RECOVERY REQUIRED" message and wait/poll until the file is cleared.
    - [x] **Tip:** Implement a simple polling mechanism (e.g., every 2 seconds) so that the `EventLoop` automatically resumes as soon as the user saves an empty `RECOVERY_QUEUE.md` file.
- [x] Task: Write Tests: Mock a populated `RECOVERY_QUEUE.md` and verify that the system blocks execution.
- [x] Task: Conductor - User Manual Verification 'Phase 3: Sovereignty Block' (Protocol in workflow.md)

## Phase 4: Audit Logging (RequestLog.md)
- [x] Task: Implement the `AuditLogger` component.
    - [x] Subscribe to loop halt and recovery events.
    - [x] Log machine-readable entries to `RequestLog.md`.
- [x] Task: Write Tests: Verify that halt events are accurately recorded in the log file.
- [x] Task: Conductor - User Manual Verification 'Phase 4: Audit & Finalization' (Protocol in workflow.md)