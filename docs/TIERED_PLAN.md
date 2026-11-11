# WriterCraft — Tiered Execution Plan

**Parent Standard:** [`RUST_ARTCRAFT_STANDARD.md`](../../mcp-central-docs/standards/RUST_ARTCRAFT_STANDARD.md)  
**Menu Bar Standard:** [`CRAFT_MENU_BAR_STANDARD.md`](../../mcp-central-docs/standards/rust-artcraft/CRAFT_MENU_BAR_STANDARD.md)  
**Architecture Model:** 4-Tier ArtCraft Incremental Delivery  
**Status:** Active  

---

## Architecture Flow

```
Tier 1: Core Engine ──> Tier 2: MCP / CLI ──> Tier 3: Desktop UI ──> Tier 4: Menu Bar, Help & Packaging
(writercraft-core)     (writercraft-mcp)     (writercraft-app)     (Hardened Workstation)
```

---

## Tier Specifications

### Tier 1 — Core Manuscript & Narrative Engine (`writercraft-core`)
- [x] L1 `Command::dispatch` universal authoring execution bus.
- [x] Manuscript document tree structure (Acts, Chapters, Scenes).
- [x] Scene graph tracking characters, locations, and narrative arcs.
- [x] Lexical richness metrics and readability indices.
- [x] `#![forbid(unsafe_code)]` and zero-panic clippy lints.

### Tier 2 — Headless Driver & FastMCP (`writercraft-mcp`)
- [x] Unified CLI `writercraft-cli` with `--json` output.
- [x] FastMCP stdio server communicating over JSON-RPC 2.0.
- [x] Manuscript compilation pipeline (Markdown, Typst PDF, EPUB).
- [x] Headless `--smoke` verification without window initialization.

### Tier 3 — Desktop Workstation UI (`writercraft-app`)
- [x] Immediate-mode `egui` 0.37 / `wgpu` workstation.
- [x] Distraction-free typewriter editor with warm paper typography.
- [x] Narrative outline binder and live word-count pacing HUD.

### Tier 4 — Craft Menu Bar Standard, Help Suite & Hardened Verification
- [ ] Universal 8-menu bar (`File`, `Edit`, `View`, `Insert`, `Modify`, `Commands`, `Window`, `Help`) with `switch_on_hover`.
- [ ] Mandatory 5-dialog Help suite (`Command Palette...`, `Keyboard Shortcuts...`, `Project Files & Docs...`, `Licenses & Notices...`, `About WriterCraft`).
- [ ] Fleet UI/UX Pattern 2 adherence: viewport-centric navigation, top button row mapped to View toggles, panel visibility managed via `View` / `Window`.
- [ ] Tofu Guard verification (100% font glyph coverage, zero tofu placeholder boxes).
- [ ] Headless `--smoke` parity validation.
- [ ] Standalone release binary packaging and NSIS installer.
