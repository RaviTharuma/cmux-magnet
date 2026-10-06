# Design

Magnet-style layout presets with a sidebar **preset picker**.
- **Native first**: inherit the cmux sidebar chrome and the Ghostty/terminal theme (colours, fonts, spacing); no custom palette.
- Status is shown as compact pills; colour carries state only together with a text/glyph (never colour alone).
- Must stay legible in both light and dark cmux themes; test at narrow sidebar widths.
- Preset icons are schematic pane diagrams (rows/cols/halves/grid); blue snap-zone overlays are the target UX once cmux exposes drag hooks.
