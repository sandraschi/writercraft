# WriterCraft — Product Requirements Document (PRD)

**Document Identifier:** `CRAFT-PRD-2026-WRITE-001`  
**Version:** 1.0.0  
**Status:** Active  
**Domain:** Distraction-Free Long-Form Authoring, Narrative Graphs, Canon Analytics  
**Standard Port:** 14124  
**Parent Standard:** [`RUST_ARTCRAFT_STANDARD.md`](../../mcp-central-docs/standards/RUST_ARTCRAFT_STANDARD.md)  
**Menu Bar Standard:** [`CRAFT_MENU_BAR_STANDARD.md`](../../mcp-central-docs/standards/rust-artcraft/CRAFT_MENU_BAR_STANDARD.md)  

---

## 1. Executive Summary & Problem Statement

Modern creative writing tools (Scrivener, Ulysses, Final Draft) are locked into proprietary file formats, platform-specific ecosystems, or fragile cloud sync backends.

**The WriterCraft Solution:**
WriterCraft is a local-first, distraction-free long-form authoring suite and narrative structure workstation written in pure safe Rust:
- Plain markdown / CommonMark document hierarchy with frontmatter metadata.
- Scene graph and narrative arc timeline with character POV tracking.
- Classical and speculative canon corpus analysis with lexical richness metrics.
- Export to print-ready PDF, EPUB, and submission manuscripts (Shunn format).
- Full MCP stdio server driver for agentic research, outlining, and consistency checking.

---

## 2. Core Functional Requirements

1. **Long-Form Manuscript Engine (`writercraft-core`):**
   - Manuscript document tree (Acts, Chapters, Scenes).
   - Scene graph tracking characters, locations, timelines, and POV tags.
   - Word count targets, daily writing sprints, and reading time analytics.
2. **Export & Publishing Pipelines:**
   - Multi-format compilation (PDF via Typst, EPUB via BookCraft, Manuscript formatting).
3. **Universal Command Bus (`Command::dispatch`):**
   - 100% of CLI, MCP, and UI actions dispatch through `writercraft-core::dispatch`.

---

## 3. UI/UX Architecture & Menu Bar Standard

In accordance with [`CRAFT_MENU_BAR_STANDARD.md`](../../mcp-central-docs/standards/rust-artcraft/CRAFT_MENU_BAR_STANDARD.md):
- **Pattern 2 (Viewport-Centric Distraction-Free Authoring Workstation):** Persistent text editor canvas with typography scaling, typewriter mode, and collapsible narrative outline binder. Top quick button row acts as accelerators into the `View` menu. To add a new manuscript tool or canon analyzer:
  1. Register a visibility toggle in the `View` or `Window` menu.
  2. Add an entry in the sidebar Binder / Characters toolbox.
  3. Wire the action directly into the universal `Command::dispatch` bus.
- **Top Menu Bar Hierarchy:**
  - `File`: New Manuscript (`Ctrl+N`), Open Project (`Ctrl+O`), Open Recent ▸, Save Manuscript (`Ctrl+S`), Save As... (`Ctrl+Shift+S`), Compile / Export (`Ctrl+E`), Close (`Ctrl+W`), Quit (`Ctrl+Q`).
  - `Edit`: Undo (`Ctrl+Z`), Redo (`Ctrl+Y`), Cut, Copy, Paste, Select All, Find / Replace (`Ctrl+F`), Preferences (`Ctrl+,`).
  - `View`: Editor Stage, Binder Tree, Narrative Arc Timeline, Character Inspector, Word Count HUD, Theme (`Warm Paper`/`Obsidian`).
  - `Insert`: Add Chapter, Add Scene, Insert Character POV Marker, Add Research Note.
  - `Modify`: Reorder Scenes, Split Scene, Merge Chapters, Update Word Target.
  - `Commands`: Compile Manuscript, Run Canon Consistency Check, Calculate Readability Score, Start Sprint Timer.
  - `Window`: Reset Layout, Editor Viewport, Binder Drawer, Inspector Dock.
  - `Help` (Mandatory 5-Dialog Suite): `Command Palette...` (`Ctrl+K`), `Keyboard Shortcuts...` (`F1`), `Project Files & Docs...`, `Licenses & Notices...`, `About WriterCraft`.
- **Behavioral Mechanics:** Smooth `switch_on_hover` between top-level menus, zero-unsafe immediate-mode rendering, and headless `--smoke` parity.
