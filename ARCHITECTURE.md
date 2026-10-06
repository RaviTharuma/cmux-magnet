# Architecture

```
cmux-magnet apply <preset>  (src/main.rs, layouts.rs)
      │  cmux-client: new-split + equalize (src/cmux.rs)
      ▼
cmux workspace panes  ◀── preset picker: cmux-magnet-sidebar (src/sidebar_main.rs) + swift/CmuxMagnetSidebar
```

Presets: `rows-3`, `cols-3`, `halves`, `grid-2x2` (best-effort). Live drag snap zones need native cmux support (ExtensionKit is sidebar-only) — tracked in manaflow-ai/cmux#12230.
