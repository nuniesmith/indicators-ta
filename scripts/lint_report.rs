#!/usr/bin/env rust-script
//! lint_report — Run ruff and mypy on a source directory and write a Markdown report.
//!
//! Usage: lint_report [src_dir] [output_file]
//!   src_dir      directory to lint  (default: src)
//!   output_file  report path        (default: lint_report.md)

use std::env;
use std::fmt;
use std::fs;
use std::io;
use std::path::Path;
use std::process::{Command, Stdio};

// ── types ─────────────────────────────────────────────────────────────────────

#[derive(Debug)]
struct ToolResult {
    output: String,
    exit_ok: bool,
    issue_count: usize,
}

#[derive(Debug)]
enum AppError {
    MissingTool(String),
    MissingSourceDir(String),
    Io(io::Error),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::MissingTool(name) => write!(
                f,
                "ERROR: '{}' not found. Install it with: pip install {}",
                name, name
            ),
            AppError::MissingSourceDir(dir) => {
                write!(f, "ERROR: Source directory '{}' not found.", dir)
            }
            AppError::Io(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl From<io::Error> for AppError {
    fn from(e: io::Error) -> Self {
        AppError::Io(e)
    }
}

// ── helpers ───────────────────────────────────────────────────────────────────

/// Returns true if `tool` resolves on PATH.
fn command_exists(tool: &str) -> bool {
    // `which` / `where` work on Unix/Windows respectively; using `command -v`
    // equivalent via attempting a no-op execution check via PATH lookup.
    which(tool).is_some()
}

fn which(tool: &str) -> Option<()> {
    // Attempt to spawn with --version or similar; simpler: just check PATH.
    let path_var = env::var("PATH").unwrap_or_default();
    let separator = if cfg!(windows) { ';' } else { ':' };
    for dir in path_var.split(separator) {
        let candidate = Path::new(dir).join(tool);
        // Also check with .exe on Windows
        if candidate.is_file() {
            return Some(());
        }
        #[cfg(windows)]
        {
            let exe = Path::new(dir).join(format!("{}.exe", tool));
            if exe.is_file() {
                return Some(());
            }
        }
    }
    None
}

fn check_tool(name: &str) -> Result<(), AppError> {
    if !command_exists(name) {
        return Err(AppError::MissingTool(name.to_string()));
    }
    Ok(())
}

/// Run a command against `src_dir`, capturing combined stdout + stderr.
fn run_tool(tool: &str, src_dir: &str) -> ToolResult {
    eprintln!("Running {} on {} …", tool, src_dir);

    let result = Command::new(tool)
        .args(if tool == "mypy" {
            vec![src_dir]
        } else {
            // ruff needs the `check` subcommand
            vec!["check", src_dir]
        })
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output();

    match result {
        Err(e) => ToolResult {
            output: format!("Failed to spawn {}: {}", tool, e),
            exit_ok: false,
            issue_count: 0,
        },
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
            let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
            let combined = format!("{}{}", stdout, stderr);
            let exit_ok = out.status.success();
            let issue_count = count_issues(&combined);
            ToolResult {
                output: combined,
                exit_ok,
                issue_count,
            }
        }
    }
}

/// Count lines that look like file-level diagnostics (contain `path:line`).
fn count_issues(output: &str) -> usize {
    output
        .lines()
        .filter(|line| {
            // Match lines like  src/foo.py:12:3: E501 ...
            line.contains(':')
                && line
                    .splitn(3, ':')
                    .nth(1)
                    .map(|s| s.trim().parse::<u64>().is_ok())
                    .unwrap_or(false)
        })
        .count()
}

fn overall_status(ruff: &ToolResult, mypy: &ToolResult) -> &'static str {
    if ruff.exit_ok && mypy.exit_ok {
        "✅ Passed"
    } else {
        "❌ Issues found"
    }
}

fn tool_badge(result: &ToolResult) -> String {
    if result.exit_ok {
        "✅ Clean".to_string()
    } else {
        format!("❌ {} issue(s)", result.issue_count)
    }
}

fn tool_output_block(result: &ToolResult) -> &str {
    let trimmed = result.output.trim();
    if trimmed.is_empty() {
        "No issues found."
    } else {
        trimmed
    }
}

// ── report writer ─────────────────────────────────────────────────────────────

fn write_report(
    path: &str,
    src_dir: &str,
    timestamp: &str,
    ruff: &ToolResult,
    mypy: &ToolResult,
) -> Result<(), AppError> {
    let report = format!(
        r#"# Lint Report

| | |
|---|---|
| **Generated** | {timestamp} |
| **Source directory** | `{src_dir}` |
| **Overall status** | {status} |

---

## Summary

| Tool | Status | Issues |
|------|--------|--------|
| [ruff](https://docs.astral.sh/ruff/) | {ruff_badge} | {ruff_count} |
| [mypy](https://mypy.readthedocs.io/) | {mypy_badge} | {mypy_count} |

---

## ruff

> Fast Python linter — style, imports, and common bugs.

```
{ruff_output}
```

---

## mypy

> Static type checker.

```
{mypy_output}
```

---

*Report generated by `lint_report`*
"#,
        timestamp = timestamp,
        src_dir = src_dir,
        status = overall_status(ruff, mypy),
        ruff_badge = tool_badge(ruff),
        ruff_count = ruff.issue_count,
        mypy_badge = tool_badge(mypy),
        mypy_count = mypy.issue_count,
        ruff_output = tool_output_block(ruff),
        mypy_output = tool_output_block(mypy),
    );

    fs::write(path, report)?;
    Ok(())
}

// ── main ──────────────────────────────────────────────────────────────────────

fn run() -> Result<bool, AppError> {
    let args: Vec<String> = env::args().collect();
    let src_dir = args.get(1).map(String::as_str).unwrap_or("src");
    let output = args.get(2).map(String::as_str).unwrap_or("lint_report.md");

    // Pre-flight checks
    if !Path::new(src_dir).is_dir() {
        return Err(AppError::MissingSourceDir(src_dir.to_string()));
    }
    check_tool("ruff")?;
    check_tool("mypy")?;

    // Timestamp
    let timestamp = {
        // Use `date` for simplicity (avoids pulling in chrono for a CLI tool).
        // Falls back to an empty string if unavailable.
        Command::new("date")
            .arg("+%Y-%m-%d %H:%M:%S")
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_else(|_| "unknown".to_string())
    };

    // Run linters
    let ruff = run_tool("ruff", src_dir);
    let mypy = run_tool("mypy", src_dir);

    // Write report
    write_report(output, src_dir, &timestamp, &ruff, &mypy)?;
    eprintln!("Report written to: {}", output);

    Ok(ruff.exit_ok && mypy.exit_ok)
}

fn main() {
    match run() {
        Ok(passed) => {
            if !passed {
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}
