# WriterCraft — HANDOVER.md

**Date:** 2026-11-10  
**Status:** v0.3.1 — Complete Multi-Persona Studio, Calibre Connector, AI Writing Companion, Staged Syndication Engine, and Craft Delectations Library.  
**Verification:** 13/13 tests passing (`cargo test --manifest-path writercraft/Cargo.toml`), `--smoke` passes with code 0 (14 capabilities).

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
   - Subcommands added: `calibre-libs`, `calibre-search`, `book`, `deep`, `continue`, `lint`, `draft-next`, `syndicate`, `essays`, `essay`.

6. **In-App Craft Delectations & Trope Library (`crates/writercraft-core/src/delectation.rs`)**:
   - Compiles all 5 rich literary treatises directly into the desktop studio with generous prose typography, headings, blockquotes, and tables:
     - `dialectic`: *The Writer & His AI Companion* (Asymmetry, 35+ Claude-ese crutches, classical rhetoric subversion, 20-antipattern human correction matrix).
     - `canon`: *What Makes Classics Classics?* (Dickens, Dostoevsky, and the Brontës: Bakhtin's polyphony, grotesque particularity, unpayable moral debt, and civilizational seismology).
     - `frontier`: *The Speculative Frontier & The RSI Novel* (Hard Sci-Fi classics: Gibson, Stephenson, Egan; the sci-fi hiatus; how Iain M. Banks bypassed the superior-mind paradox in The Culture; the anatomy of the great Recursive Self-Improvement novel).
     - `tropes`: *The Cliché Motherlode & Trope Smorgasbord* (Romance, Sci-Fi, Action/Thrillers, Prestige Gloom, and the Frankenstein Smorgasbord Scaffold).
     - `publishing`: *Publishing Avenues & The Cautionary Tale* (The Coral Hart 370-book-a-year churn vs durable backlist architecture).
   - Interactive essay integration: one-click loading of the 7-beat Frankenstein Smorgasbord Scaffold into the Corkboard cards, instant style auditing, and zero-lock Calibre classics searching.
   - FastMCP tools expanded to 19 total tools (`writercraft_list_craft_essays`, `writercraft_read_craft_essay`, `writercraft_frankenstein_beats`).

---

## Verification Commands

```powershell
# Run all tests
cargo test --manifest-path writercraft/Cargo.toml

# Smoke test
./target/release/writercraft.exe --smoke

# Lint chapter
./target/release/writercraft-cli.exe lint on-metacraft/chapters/01-the-breach.md

# AI companion chapter draft
./target/release/writercraft-cli.exe draft-next on-metacraft 01-the-breach.md --hint "focus on high voltage"

# Staged syndication
./target/release/writercraft-cli.exe syndicate on-metacraft/chapters/01-the-breach.md --out-dir outbox
```

---

## Deliberately Not Done (Boundaries & Non-Goals)

1. **No External Network Dispatching**: WriterCraft never auto-posts to Substack, Medium, Reddit, or Hacker News. Staging to `./outbox` is the strict stopping boundary.
2. **No Calibre Mutation**: Calibre's `metadata.db` is strictly opened read-only. For editing/converting Calibre books, the dedicated sibling craft [`bookcraft`](../bookcraft) handles mutations through transactional SQL with backups.
3. **No TeX Live Dependency**: Academic papers export to clean Typst markup and LaTeX source; WriterCraft does not bundle a multi-gigabyte TeX Live distribution.
