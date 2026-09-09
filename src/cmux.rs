use anyhow::{Context, Result};
use std::process::Command;

pub fn run_cmux(args: &[&str]) -> Result<String> {
    let out = Command::new("cmux")
        .args(args)
        .output()
        .context("spawn cmux — is cmux on PATH?")?;
    if !out.status.success() {
        let stderr = String::from_utf8_lossy(&out.stderr);
        anyhow::bail!("cmux {} failed: {stderr}", args.join(" "));
    }
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

pub fn equalize_splits() -> Result<()> {
    if run_cmux(&["rpc", "workspace.equalize_splits"]).is_ok() {
        return Ok(());
    }
    if run_cmux(&["workspace-action", "--action", "equalizeSplits"]).is_ok() {
        return Ok(());
    }
    run_cmux(&["equalize-splits"]).map(|_| ())
}

pub fn new_split(direction: &str) -> Result<()> {
    run_cmux(&["new-split", direction]).map(|_| ())
}

pub fn pane_count() -> Result<usize> {
    if let Ok(s) = run_cmux(&["rpc", "workspace.pane_count"]) {
        if let Ok(n) = s.parse::<usize>() {
            return Ok(n);
        }
    }
    if let Ok(s) = run_cmux(&["list-panes"]) {
        let n = s.lines().filter(|l| !l.trim().is_empty()).count();
        if n > 0 {
            return Ok(n);
        }
    }
    Ok(1)
}
