use anyhow::{bail, Context, Result};
use clap::ValueEnum;
use serde::Serialize;

use crate::cmux;

#[derive(Debug, Clone, Copy, ValueEnum, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum LayoutPreset {
    Rows3,
    Cols3,
    Halves,
    Grid2x2,
}

impl LayoutPreset {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Rows3 => "rows-3",
            Self::Cols3 => "cols-3",
            Self::Halves => "halves",
            Self::Grid2x2 => "grid-2x2",
        }
    }
}

pub fn apply_preset(preset: LayoutPreset) -> Result<()> {
    match preset {
        LayoutPreset::Halves => {
            ensure_min_panes_direction(2, "right")?;
            cmux::equalize_splits().context("equalize after halves")?;
        }
        LayoutPreset::Rows3 => {
            ensure_min_panes_direction(3, "down")?;
            cmux::equalize_splits().context("equalize after rows-3")?;
        }
        LayoutPreset::Cols3 => {
            ensure_min_panes_direction(3, "right")?;
            cmux::equalize_splits().context("equalize after cols-3")?;
        }
        LayoutPreset::Grid2x2 => {
            ensure_min_panes_direction(2, "right")?;
            let _ = cmux::new_split("down");
            let _ = cmux::run_cmux(&["workspace-action", "--action", "focusNextPane"]);
            let _ = cmux::new_split("down");
            cmux::equalize_splits().context("equalize after grid-2x2")?;
        }
    }
    Ok(())
}

fn ensure_min_panes_direction(n: usize, direction: &str) -> Result<()> {
    let count = cmux::pane_count().unwrap_or(1);
    if count >= n {
        return Ok(());
    }
    for _ in count..n {
        cmux::new_split(direction)
            .with_context(|| format!("new-split {direction} while targeting {n} panes"))?;
    }
    let after = cmux::pane_count().unwrap_or(0);
    if after < n {
        bail!("only {after} panes after splits; wanted {n}. Focus a pane and re-run.");
    }
    Ok(())
}
