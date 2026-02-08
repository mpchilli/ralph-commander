use std::fs;
use std::path::{Path, PathBuf};
use std::io::Write;

/// Logs security and safety events to RequestLog.md.
pub struct AuditLogger {
    path: PathBuf,
}

impl AuditLogger {
    /// Creates a new AuditLogger.
    pub fn new(workspace_root: &Path) -> Self {
        Self {
            path: workspace_root.join("RequestLog.md"),
        }
    }

    /// Logs a safety event.
    pub fn log_event(&self, event_type: &str, correlation_id: &str, details: &str) {
        let timestamp = chrono::Utc::now().to_rfc3339();
        let entry = format!("| {} | {} | {} | {} |", timestamp, event_type, correlation_id, details);
        
        // Ensure file exists with header if needed
        if !self.path.exists() {
            let _ = fs::write(&self.path, "# Request Log (Audit Trail)\n\n| Timestamp | Event Type | Correlation ID | Details |\n| --- | --- | --- | --- |\n");
        }

        if let Ok(mut file) = fs::OpenOptions::new().append(true).open(&self.path) {
            let _ = writeln!(file, "{}", entry);
        }
    }

    /// Logs a loop halt.
    pub fn log_halt(&self, correlation_id: &str, reason: &str) {
        self.log_event("LOOP_HALTED", correlation_id, reason);
    }

    /// Logs a recovery transition.
    pub fn log_recovery(&self, correlation_id: &str) {
        self.log_event("LOOP_RESUMED", correlation_id, "Recovery queue cleared by human");
    }
}
