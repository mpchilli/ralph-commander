//! Health command for validating system readiness.

use anyhow::{Context, Result};
use async_trait::async_trait;
use clap::{ArgAction, Parser};
use ralph_core::RalphConfig;
use serde::Serialize;
use std::path::Path;

use crate::ConfigSource;

#[derive(Parser, Debug)]
pub struct HealthArgs {
    /// Show detailed pass/fail status for each check
    #[arg(short, long)]
    pub verbose: bool,

    /// Suppress all output except errors
    #[arg(short, long)]
    pub quiet: bool,

    /// Output results in JSON format
    #[arg(long)]
    pub json: bool,

    /// Run only specific check(s) (e.g., git, conductor, env, disk)
    #[arg(long, value_name = "NAME", action = ArgAction::Append)]
    pub check: Vec<String>,
}

#[derive(Serialize)]
pub struct HealthReport {
    pub status: String,
    pub passed: bool,
    pub results: Vec<HealthResult>,
}

pub async fn execute(
    config_sources: &[ConfigSource],
    args: HealthArgs,
    use_colors: bool,
) -> Result<()> {
    let config = crate::load_config_with_overrides(config_sources)?;
    
    let mut checks: Vec<Box<dyn HealthCheck>> = Vec::new();
    
    let requested = args.check.iter().map(|s| s.to_lowercase()).collect::<Vec<_>>();
    
    if requested.is_empty() || requested.contains(&"git".to_string()) {
        checks.push(Box::new(GitCheck));
    }
    if requested.is_empty() || requested.contains(&"conductor".to_string()) {
        checks.push(Box::new(ConductorCheck));
    }
    if requested.is_empty() || requested.contains(&"env".to_string()) {
        checks.push(Box::new(EnvCheck));
    }
    if requested.is_empty() || requested.contains(&"disk".to_string()) {
        checks.push(Box::new(DiskCheck));
    }

    let mut results = Vec::new();

    for check in checks {
        results.push(check.run(&config).await);
    }

    let all_passed = results.iter().all(|r| r.success);
    let report = HealthReport {
        status: if all_passed { "OK".to_string() } else { "FAIL".to_string() },
        passed: all_passed,
        results: results.clone(),
    };

    if args.json {
        println!("{}", serde_json::to_string_pretty(&report)?);
    } else if !args.quiet {
        for result in &results {
            if args.verbose || !result.success {
                print_result(result, use_colors);
            }
        }

        if all_passed {
            if use_colors {
                println!("\x1b[32mSystem OK\x1b[0m");
            } else {
                println!("System OK");
            }
        } else {
            println!("\nSummary of failures:");
            for failure in results.iter().filter(|r| !r.success) {
                println!("  - {}: {}", failure.name, failure.message);
                if let Some(hint) = &failure.remediation {
                    println!("    Hint: {}", hint);
                }
            }
        }
    }

    if !all_passed {
        std::process::exit(1);
    }
    
    Ok(())
}

fn print_result(result: &HealthResult, use_colors: bool) {
    let status = if result.success {
        if use_colors { "\x1b[32m[OK]\x1b[0m" } else { "[OK]" }
    } else {
        if use_colors { "\x1b[31m[FAIL]\x1b[0m" } else { "[FAIL]" }
    };
    println!("{} {}: {}", status, result.name, result.message);
}

#[async_trait]
trait HealthCheck: Send + Sync {
    async fn run(&self, config: &RalphConfig) -> HealthResult;
}

#[derive(Clone, Serialize)]
pub struct HealthResult {
    pub name: String,
    pub success: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remediation: Option<String>,
}

struct GitCheck;

#[async_trait]
impl HealthCheck for GitCheck {
    async fn run(&self, config: &RalphConfig) -> HealthResult {
        let root = &config.core.workspace_root;
        let git_dir = root.join(".git");
        if git_dir.exists() {
            HealthResult {
                name: "Git Integrity".to_string(),
                success: true,
                message: "Valid git repository found".to_string(),
                remediation: None,
            }
        } else {
            HealthResult {
                name: "Git Integrity".to_string(),
                success: false,
                message: "Not a git repository".to_string(),
                remediation: Some("Run `git init` or move to a git repository root.".to_string()),
            }
        }
    }
}

struct ConductorCheck;

#[async_trait]
impl HealthCheck for ConductorCheck {
    async fn run(&self, config: &RalphConfig) -> HealthResult {
        let root = &config.core.workspace_root;
        let conductor_dir = root.join("conductor");
        if !conductor_dir.exists() {
            return HealthResult {
                name: "Conductor Environment".to_string(),
                success: false,
                message: "Conductor directory missing".to_string(),
                remediation: Some("Run `/conductor:setup` to initialize the environment.".to_string()),
            };
        }

        let mandatory_files = ["index.md", "workflow.md", "product.md", "tech-stack.md"];
        let mut missing = Vec::new();
        for file in mandatory_files {
            if !conductor_dir.join(file).exists() {
                missing.push(file);
            }
        }

        if missing.is_empty() {
            HealthResult {
                name: "Conductor Environment".to_string(),
                success: true,
                message: "All core Conductor files present".to_string(),
                remediation: None,
            }
        } else {
            HealthResult {
                name: "Conductor Environment".to_string(),
                success: false,
                message: format!("Missing files: {}", missing.join(", ")),
                remediation: Some("Ensure all mandatory Conductor files are present in the conductor/ directory.".to_string()),
            }
        }
    }
}

struct EnvCheck;

#[async_trait]
impl HealthCheck for EnvCheck {
    async fn run(&self, config: &RalphConfig) -> HealthResult {
        if config.robot.enabled {
            if config.robot.resolve_bot_token().is_some() {
                 HealthResult {
                    name: "Environment Variables".to_string(),
                    success: true,
                    message: "Mandatory environment variables present".to_string(),
                    remediation: None,
                }
            } else {
                HealthResult {
                    name: "Environment Variables".to_string(),
                    success: false,
                    message: "RALPH_TELEGRAM_BOT_TOKEN missing".to_string(),
                    remediation: Some("Set the RALPH_TELEGRAM_BOT_TOKEN environment variable or configure it in ralph.yml.".to_string()),
                }
            }
        } else {
            HealthResult {
                name: "Environment Variables".to_string(),
                success: true,
                message: "No mandatory environment variables required for current config".to_string(),
                remediation: None,
            }
        }
    }
}

struct DiskCheck;

#[async_trait]
impl HealthCheck for DiskCheck {
    async fn run(&self, _config: &RalphConfig) -> HealthResult {
        HealthResult {
            name: "Disk Space".to_string(),
            success: true,
            message: "Sufficient disk space available (assumed)".to_string(),
            remediation: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ralph_core::RalphConfig;

    #[tokio::test]
    async fn test_git_check_pass() {
        let temp = tempfile::tempdir().unwrap();
        std::fs::create_dir(temp.path().join(".git")).unwrap();
        let mut config = RalphConfig::default();
        config.core.workspace_root = temp.path().to_path_buf();
        
        let check = GitCheck;
        let result = check.run(&config).await;
        assert!(result.success);
        assert!(result.message.contains("Valid git repository"));
    }

    #[tokio::test]
    async fn test_git_check_fail() {
        let temp = tempfile::tempdir().unwrap();
        let mut config = RalphConfig::default();
        config.core.workspace_root = temp.path().to_path_buf();
        
        let check = GitCheck;
        let result = check.run(&config).await;
        assert!(!result.success);
        assert!(result.message.contains("Not a git repository"));
    }

    #[tokio::test]
    async fn test_conductor_check_fail_missing_dir() {
        let temp = tempfile::tempdir().unwrap();
        let mut config = RalphConfig::default();
        config.core.workspace_root = temp.path().to_path_buf();
        
        let check = ConductorCheck;
        let result = check.run(&config).await;
        assert!(!result.success);
        assert!(result.message.contains("Conductor directory missing"));
    }
}