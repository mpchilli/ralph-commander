use std::fs;
use std::path::{Path, PathBuf};
use std::io;

/// Manages the RECOVERY_QUEUE.md file for human intervention.
pub struct RecoveryQueue {
    path: PathBuf,
}

impl RecoveryQueue {
    /// Creates a new RecoveryQueue handler.
    pub fn new(workspace_root: &Path) -> Self {
        Self {
            path: workspace_root.join("RECOVERY_QUEUE.md"),
        }
    }

    /// Checks if the recovery queue is non-empty.
    pub fn is_blocked(&self) -> bool {
        if !self.path.exists() {
            return false;
        }
        
        fs::read_to_string(&self.path)
            .map(|content| !content.trim().is_empty())
            .unwrap_or(false)
    }

    /// Records a failure to the recovery queue.
    pub fn record_failure(
        &self,
        task_id: &str,
        reason: &str,
        last_sha: Option<&str>,
    ) -> io::Result<()> {
        let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC");
        let sha_display = last_sha.unwrap_or("unknown");
        let rollback_cmd = if let Some(sha) = last_sha {
            format!("`git reset --hard {}`", sha)
        } else {
            "*No snapshot SHA available for automated rollback.*".to_string()
        };

        let entry = format!(
            "# 🚨 RECOVERY REQUIRED ({})


             ## Failed Task

             - **ID:** {}

             - **Reason:** {}


             ## Recovery Options

             - **Last Safe Snapshot:** `{}`

             - **Rollback Command:** {}


             --- 

             *Human Commander: Please resolve the issue and CLEAR THIS FILE to resume orchestration.*
",
            timestamp, task_id, reason, sha_display, rollback_cmd
        );

        fs::write(&self.path, entry)?;
        Ok(())
    }

    /// Clears the recovery queue.
    pub fn clear(&self) -> io::Result<()> {
        if self.path.exists() {
            fs::write(&self.path, "")?;
        }
        Ok(())
    }
}
