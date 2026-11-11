# WriterCraft — HANDOVER.md

**Date:** 2026-11-11  
**Status:** v0.3.1 (Tier 4 / Sovereign Epistemic Production Grade) — Multi-Persona Studio, Calibre Connector, AI Writing Companion, Epistemic Matrix Engine, Idiolect Profiler, Tension Waveform Painter, and Fleet Epistemic Harmonizer.  
**Verification:** 16/16 tests passing (`cargo test --manifest-path writercraft/Cargo.toml`), `--smoke` passes with code 0 (17 capabilities, 5 fleet books detected, 13 Calibre libraries detected).

---

## What Was Done Today

1. **Prominent Calibre Library Connector (`crates/writercraft-core/src/calibre.rs`)**:
   - Auto-discovers Calibre libraries from `%APPDATA%/calibre/gui.json` and `global.py.json`.
   - Connected primary library: `L:\Multimedia Files\Written Word\Calibre-Bibliothek` (10,346 books).
   - Zero-lock SQLite read-only architecture using `SQLITE_OPEN_READ_ONLY | SQLITE_OPEN_NO_MUTEX`.
   - Live search by title, author, series, and tags with blurb extraction.
   - One-click corkboard adaptation card generation and BibTeX citation formatting.

2. **Interactive Master-Detail Fleet Books Studio (`crates/writercraft-core/src/fleet_book.rs` & `continuation.rs`)**:
   - Master list for the 5 books (`on-ai-takeover`, `on-hyperwar`, `on-hyperart`, `on-metacraft`, `on-hypermind`).
   - Chapter breakdown with word counts, Flesch-Kincaid grade levels, and open `[construction]` badges.
   - Chapter deep analysis with line-level claim ledgers and emotional tension scores.
   - 3-Beat continuation synthesis (*Thesis Breach* $\to$ *Physical Firewall Collision* $\to$ *Irreversible Baseline*).

3. **Style Guide & Anti-Claude-ese Deterministic Linter (`crates/writercraft-core/src/style_guide.rs`)**:
   - Scans 35+ signature AI crutches (*loadbearing*, *delve*, *tapestry*, *testament to*, *nuanced*, *dance of*).
   - Detects reflexive triads ($A, B, \text{ and } C$) and contrast frames (*not just X, but Y*).
   - Sentence length variance scoring (flags monotonic machine cadence).
   - Codified full treatise in `docs/WRITER_AND_AI_DIALECTIC.md`.

4. **AI Writing Companion & Staged Syndication (`crates/writercraft-core/src/companion.rs`)**:
   - Constrained next-chapter drafting based on author hints, target rung, tone, and zero-slop rules.
   - Companion Substack/blogpost and Hacker News Show HN note generator.
   - **Strict Guardrail**: Stored strictly locally in `outbox/`; **never** dispatched unasked.

5. **GUI & CLI Overhaul (`crates/writercraft-app`)**:
   - 6 dedicated workspaces: Fleet Studio, Calibre Connector, Corkboard & Arcs, Screenplay (Fountain), Scholar & BibTeX, and Craft Delectations.
   - Subcommands added: `calibre-libs`, `calibre-search`, `book`, `deep`, `continue`, `lint`, `draft-next`, `syndicate`, `essays`, `essay`, `epistemic`, `idiolect`, `fleet-harmonize`.

6. **In-App Craft Delectations & Trope Library (`crates/writercraft-core/src/delectation.rs`)**:
   - Compiles all 5 rich literary treatises directly into the desktop studio with generous prose typography:
     - `dialectic`: *The Writer & His AI Companion*
     - `canon`: *What Makes Classics Classics?*
     - `frontier`: *The Speculative Frontier & The RSI Novel*
     - `tropes`: *The Cliché Motherlode & Trope Smorgasbord*
     - `publishing`: *Publishing Avenues & The Cautionary Tale*
   - Interactive essay integration: one-click loading of the 7-beat Frankenstein Smorgasbord Scaffold into the Corkboard cards, instant style auditing, and zero-lock Calibre classics searching.

7. **Tier 4 Sovereign Epistemic Knowledge Matrix, Voice Profiler & Harmonizer**:
   - **Epistemic Knowledge Matrix (`crates/writercraft-core/src/epistemic_matrix.rs`)**: Models characters' knowledge states (`Ignorant`, `Suspected`, `Known`, `Misinformed`) across narrative scenes. Validates action prerequisites and detects `PrematureKnowledgeLeak` and `ContradictoryBeliefAction`. Reconciles narrative reading sequence (Syuzhet) with in-universe chronological order (Fabula) with automatic flashback detection.
   - **Character Idiolect & Dialogue Voice Profiler (`crates/writercraft-core/src/idiolect.rs`)**: Analyzes Fountain dialogue cues and prose manuscripts, computing speaking turns, word count allocation, Type-Token Ratio (TTR), contraction frequency (formal vs informal), sentence length variance, and overall voice divergence $[0.0, 1.0]$.
   - **POV Head-Hopping Detector**: Deterministically scans third-person limited prose manuscripts for non-POV characters performing internal cognitive verbs (*thought*, *felt*, *knew*, *wondered*, *realized*, *secretly wanted*, *believed in their heart*).
   - **Interactive Tension Waveform Painter (`crates/writercraft-app/src/main.rs`)**: Real-time 2D egui waveform curve displaying scene emotional valences $[-1.0, +1.0]$, tension spikes, Climax marker, and Dark Night of the Soul marker directly in Corkboard & Arcs.
   - **Fleet Epistemic Harmonizer (`crates/writercraft-core/src/fleet_harmonizer.rs`)**: Audits cross-corpus alignment across the 5 fleet books (`on-metacraft`, `on-ai-takeover`, `on-hyperwar`, `on-hyperart`, `on-hypermind`), auditing the 6-Level Autonomy Ladder (Levels 0-5), clustering 100+ open `[construction]` claims into thematic clusters, and computing terminology consistency scores.
   - **FastMCP Protocol Expansion (`crates/writercraft-mcp/src/main.rs`)**: 22 total tools including `writercraft_epistemic_validate`, `writercraft_idiolect_audit`, and `writercraft_fleet_harmonize`.

---

## Verification Commands

```powershell
# Run all 16 tests
cargo test --manifest-path writercraft/Cargo.toml

# Headless smoke test (17 capabilities)
./target/release/writercraft.exe --smoke

# Epistemic Knowledge Matrix audit
./target/release/writercraft-cli.exe epistemic

# Dialogue Idiolect & Head-Hopping audit
./target/release/writercraft-cli.exe idiolect path/to/script.fountain --pov "Elena"

# Fleet Epistemic Harmonization
./target/release/writercraft-cli.exe fleet-harmonize
```

---

## Deliberately Not Done (Boundaries & Non-Goals)

1. **No External Network Dispatching**: WriterCraft never auto-posts to Substack, Medium, Reddit, or Hacker News. Staging to `./outbox` is the strict stopping boundary.
2. **No Calibre Mutation**: Calibre's `metadata.db` is strictly opened read-only. For editing/converting Calibre books, the dedicated sibling craft [`bookcraft`](../bookcraft) handles mutations through transactional SQL with backups.
3. **No TeX Live Dependency**: Academic papers export to clean Typst markup and LaTeX source; WriterCraft does not bundle a multi-gigabyte TeX Live distribution.
