use cmux_magnet::LayoutPreset;

fn main() {
    println!("cmux-magnet sidebar host (0.1.0)");
    println!("Use: cmux-magnet apply rows-3|cols-3|halves|grid-2x2");
    for p in [LayoutPreset::Rows3, LayoutPreset::Cols3, LayoutPreset::Halves, LayoutPreset::Grid2x2] {
        println!("- {}", p.as_str());
    }
}
