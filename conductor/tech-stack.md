# Technology Stack: Ralph Commander

## Core Engine & CLI (Rust)
- **Runtime:** `tokio` (1.x) for high-performance asynchronous orchestration.
- **CLI/TUI:** `clap` (4.x) for command-line parsing and `ratatui` (0.30+) for the terminal-based "Mission Control" interface.
- **Serialization:** `serde` with `serde_json` and `serde_yaml` for immutable audit logs and configuration.
- **Protocols:** Custom event-driven architecture using `tokio::sync::broadcast` or similar for the "Hat System."
- **Integrations:** `teloxide` for the Ralph RObot Telegram interface and sovereignty protocols.
- **Visibility:** Live Markdown and JSON dashboards (`.captain-status.md/json`) for real-time state monitoring.

## Web Dashboard & Backend (Node.js/TypeScript)
- **Backend:** Node.js (>=22.0.0) with TypeScript, providing a REST/WebSocket API for the real-time dashboard.
- **Frontend:** Modern SPA (likely React with Vite) for the "Adaptive Command Center" visual interface.
- **Package Management:** npm workspaces for managing the multi-package monorepo.

## Infrastructure & Safety
- **Version Control:** Git (mandatory) to support "Atomic Snapshots" and the "Recovery Protocol."
- **Audit Trail:** Git Notes for attaching machine-readable triage and quality summaries to commits.
- **Containerization:** Docker & Docker Compose for reproducible execution environments.
- **CI/CD:** GitHub Actions for automated testing, linting, and "TEA Gate" verification.
- **Code Quality:** `clippy` and `rustfmt` for Rust; `eslint` and `prettier` for TypeScript.
