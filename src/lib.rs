//! cmux-magnet: Magnet-style equal pane layouts via cmux CLI/RPC.
//! Live blue snap-zone overlays need native cmux (manaflow-ai/cmux#12230).

pub mod layouts;
pub mod cmux;

pub use layouts::{LayoutPreset, apply_preset};
