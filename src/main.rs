use anyhow::Result;
use clap::{Parser, Subcommand};
use cmux_magnet::{apply_preset, LayoutPreset};

#[derive(Parser, Debug)]
#[command(name = "cmux-magnet", version, about = "Magnet-style equal layouts for cmux")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    Apply {
        #[arg(value_enum)]
        preset: LayoutPreset,
    },
    Presets,
    Doctor,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Apply { preset } => {
            apply_preset(preset)?;
            println!("applied {}", preset.as_str());
        }
        Cmd::Presets => {
            for p in [LayoutPreset::Rows3, LayoutPreset::Cols3, LayoutPreset::Halves, LayoutPreset::Grid2x2] {
                println!("{}", p.as_str());
            }
        }
        Cmd::Doctor => match cmux_magnet::cmux::run_cmux(&["--version"]) {
            Ok(v) => println!("cmux ok: {v}"),
            Err(e) => {
                eprintln!("cmux missing/broken: {e}");
                std::process::exit(1);
            }
        },
    }
    Ok(())
}
