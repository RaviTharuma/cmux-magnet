# cmux-magnet

**Magnet-style equal pane layouts** for [cmux](https://github.com/manaflow-ai/cmux) — **Rust + Swift only**.

https://github.com/RaviTharuma/cmux-magnet

Tracks [manaflow-ai/cmux#12230](https://github.com/manaflow-ai/cmux/issues/12230) (related [#731](https://github.com/manaflow-ai/cmux/issues/731)).

## Status

| Feature | Status |
| --- | --- |
| `cmux-magnet apply rows-3` | Best-effort via `cmux new-split` + equalize |
| `cols-3` / `halves` / `grid-2x2` | Same |
| Swift sidebar preset picker | Scaffold in `swift/CmuxMagnetSidebar` |
| Live Magnet blue snap zones while dragging | **Needs native cmux** — ExtensionKit is sidebar-only |

## Install

```bash
git clone https://github.com/RaviTharuma/cmux-magnet.git
cd cmux-magnet
cargo build --release
```

Use `cmux-plugin.toml` like `cmux-moshi` / `cmux-herdr`.

## Usage

```bash
cmux-magnet doctor
cmux-magnet presets
cmux-magnet apply rows-3
```

## License

MIT
