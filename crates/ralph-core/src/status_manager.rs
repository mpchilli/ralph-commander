use std::fs;
use std::path::{Path, PathBuf};
use serde::Serialize;
use crate::event_loop::LoopState;
use tracing::{warn, debug};

/// Machine-readable status structure.
#[derive(Debug, Serialize)]
pub struct CaptainStatus {
    pub objective: String,
    pub active_task: ActiveTaskInfo,
    pub health: OrchestrationHealth,
    pub safety: SafetyStatus,
}

#[derive(Debug, Serialize)]
pub struct ActiveTaskInfo {
    pub id: String,
    pub title: String,
    pub hat: String,
    pub risk_tier: String,
}

#[derive(Debug, Serialize)]
pub struct OrchestrationHealth {
    pub iteration: u32,
    pub max_iterations: u32,
    pub elapsed_seconds: u64,
    pub cumulative_cost: f64,
}

#[derive(Debug, Serialize)]
pub struct SafetyStatus {
    pub last_snapshot_sha: String,
    pub is_halted: bool,
    pub recovery_queue_blocked: bool,
}

/// Manages real-time visibility artifacts.
pub struct StatusManager {
    workspace_root: PathBuf,
}

impl StatusManager {
    pub fn new(workspace_root: &Path) -> Self {
        Self {
            workspace_root: workspace_root.to_path_buf(),
        }
    }

    /// Updates the visibility artifacts based on current loop state.
    pub fn update(
        &self,
        state: &LoopState,
        objective: &str,
        max_iterations: u32,
        recovery_blocked: bool,
    ) {
        let status = CaptainStatus {
            objective: objective.to_string(),
            active_task: ActiveTaskInfo {
                id: "pending".to_string(), // Placeholder until task tracking is more granular
                title: "In Progress".to_string(),
                hat: state.last_hat.as_ref().map(|h| h.to_string()).unwrap_or_else(|| "ralph".to_string()),
                risk_tier: state.active_strategy.as_ref().map(|s| format!("{:?}", s.tier)).unwrap_or_else(|| "Unknown".to_string()),
            },
            health: OrchestrationHealth {
                iteration: state.iteration,
                max_iterations,
                elapsed_seconds: state.elapsed().as_secs(),
                cumulative_cost: state.cumulative_cost,
            },
            safety: SafetyStatus {
                last_snapshot_sha: state.last_snapshot_sha.clone().unwrap_or_else(|| "None".to_string()),
                is_halted: state.is_halted,
                recovery_queue_blocked: recovery_blocked,
            },
        };

        // Write JSON
        let json_path = self.workspace_root.join(".captain-status.json");
        if let Ok(json) = serde_json::to_string_pretty(&status) {
            let _ = fs::write(json_path, json);
        }

        // Write Markdown
        let md_path = self.workspace_root.join(".captain-status.md");
        let md_content = self.format_markdown(&status);
        let _ = fs::write(md_path, md_content);
    }

    fn format_markdown(&self, status: &CaptainStatus) -> String {
        format!(
            "# 🧑‍✈️ Captain's Mission Control


             ## 🎯 Current Objective

             {}


             ## 🏗️ Active Task

             - **Hat:** {}

             - **Risk Tier:** {}


             ## 🩺 Orchestration Health

             - **Iteration:** {} / {}

             - **Elapsed Time:** {}s

             - **Total Cost:** ${:.4}


             ## 🛡️ Safety HUD

             - **Last Snapshot SHA:** `{}`

             - **Status:** {}
",
            status.objective,
            status.active_task.hat,
            status.active_task.risk_tier,
            status.health.iteration,
            status.health.max_iterations,
            status.health.elapsed_seconds,
            status.health.cumulative_cost,
            status.safety.last_snapshot_sha,
            if status.safety.is_halted || status.safety.recovery_queue_blocked { "🚨 HALTED (Recovery Required)" } else { "✅ OK" }
        )
    }
}
