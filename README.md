# WriterCraft & ScholarCraft

**WriterCraft** is an ArtCraft-tier local-first multi-persona authoring environment, AI writing companion, Calibre library connector, narrative corkboard, TV Tropes indexer, Fountain screenplay studio, and academic publishing suite running entirely offline without cloud extraction.

---

## The 6 Sovereign Workspaces

### 1. 📚 Fleet Books Studio (Interactive Master-Detail)
- **Fleet Corpus (5 Books)**: Interactive master-detail inspector for [`on-ai-takeover`](../on-ai-takeover), [`on-hyperwar`](../on-hyperwar), [`on-hyperart`](../on-hyperart), [`on-metacraft`](../on-metacraft), and [`on-hypermind`](../on-hypermind).
- **Chapter Breakdown**: Word count, Flesch-Kincaid grade level, and open `[construction]` flags badge per chapter.
- **Deep Chapter Analysis**: Flesch Reading Ease, reading band, line-level claim ledgers, and emotional tension rating $[-1.0, +1.0]$.
- **Anti-Claude-ese & Rhetoric Audit**: Deterministic detection of 35+ signature AI tells (*loadbearing*, *delve*, *tapestry*, *testament to*, *nuanced*), reflexive triads ($A, B, \text{ and } C$), contrast frames (*not just X, but Y*), and cadence variation scoring.
- **Continuation Engine**: 3-beat dialectic scaffolding (*Thesis Breach* $\to$ *Physical Firewall Collision* $\to$ *Irreversible Baseline*) targeting core book ladders.
- **AI Writing Companion ("AI Writes Next Chapter")**: Generates constrained next-chapter drafts based on author hints, target rung, and zero-slop constraints.
- **Staged Syndication**: Generates Substack/blogpost essays and Hacker News "Show HN" technical notes staged locally in `outbox/`.
  - **Strict Guardrail**: Stored strictly locally on disk; **never** dispatched, posted, or transmitted unasked.

### 2. 🗃️ Calibre Library Connector (Prominent)
- **Automatic Library Discovery**: Scans `%APPDATA%/calibre/gui.json` and `%APPDATA%/calibre/global.py.json`, auto-detecting 13 local libraries (including the 10,340+ book master library at `L:\Multimedia Files\Written Word\Calibre-Bibliothek`).
- **Zero-Lock Read-Only Engine**: SQLite queries against `metadata.db` using `SQLITE_OPEN_READ_ONLY | SQLITE_OPEN_NO_MUTEX` to prevent database locks while Calibre is open.
- **Fast Search**: Instant search by title, author, series, or tags across 10,000+ volumes.
- **Project Ingestion**:
  - *"Load into Corkboard as Reference / Adaptation"*: Converts book blurbs into corkboard scene cards.
  - *"Add to BibTeX Bibliography"*: Generates structured `@book{...}` academic citations.

### 3. 📌 Fiction & Corkboard Studio
- **Corkboard Scene Cards**: Formalized dramatically via POV character, dramatic goal, conflict obstacle, disaster outcome, and emotional valence $[-1.0, +1.0]$.
- **Interactive Narrative Arc & Tension Waveform Painter (egui)**: Visual 2D curve displaying emotional valence $[-1.0, +1.0]$, tension spikes, Climax marker, and Dark Night of the Soul marker directly in Corkboard & Arcs.
- **Epistemic Knowledge & Secret Matrix ("Who Knows What When")**: Tracks character knowledge states (`Ignorant`, `Suspected`, `Known`, `Misinformed`), secret leaks, dramatic irony, timeline reconciliation (Fabula vs. Syuzhet with flashback tagging), and detects epistemic plot holes (`PrematureKnowledgeLeak`, `ContradictoryBeliefAction`).
- **High-Velocity Beat Sheet Generator**: Scaffolds 15-beat / multi-act outlines for Romance (KDP trope formulas: Enemies-to-Lovers, Fake Dating, Forced Proximity), Thrillers (The Ticking Clock, Red Herring), and Sci-Fi/Singularity.
- **Curated TV Tropes Catalog**: Curated narrative mechanics index with typical emotional valence shifts.

### 4. 🎬 Screenplay Studio (Fountain) & Idiolect Profiler
- **Fountain Screenplay Parser**: Industry-standard `.fountain` parser tracking scene headings, character dialogue cues, speaking turns, word count allocation, and page-a-minute run-time estimations.
- **Character Idiolect & Voice Profiler**: Measures cadence variance, Type-Token Ratio (TTR), contraction ratios, distinctiveness scores, and overall voice divergence $[0.0, 1.0]$.
- **POV Head-Hopping Detector**: Scans third-person limited prose manuscripts for non-POV characters performing internal cognitive verbs (*thought*, *felt*, *knew*, *wondered*, *realized*, *secretly wanted*, *believed in their heart*).

### 5. 🎓 Scholar & BibTeX Studio
- **BibTeX Citation Indexer**: Parses standard BibTeX files, indexing cite keys, titles, authors, and publication venues.
- **Claim Ledger Audit**: Scans manuscripts for `[construction: ...]` tags, reporting exact line numbers and unverified claims.
- **Typst Academic Scaffolding**: Exports markdown chapters to compile-ready Typst (`.typ`) markup.
- **Fleet Epistemic Harmonizer**: Audits cross-corpus alignment across the 5 fleet books (`on-metacraft`, `on-ai-takeover`, `on-hyperwar`, `on-hyperart`, `on-hypermind`), auditing the 6-Level Autonomy Ladder (Levels 0-5), clustering 100+ open `[construction]` claims into thematic clusters, and computing terminology consistency scores.

### 6. 📖 Craft Delectations & Trope Library (Generous Prose Reader)
- **In-App Literary Essays**: Direct in-studio reader for deep treatises compiled directly into the application binary:
  - `dialectic`: *The Writer & His AI Companion* (Asymmetry, 35+ Claude-ese crutches, classical rhetoric subversion, 20-antipattern human correction matrix).
  - `canon`: *What Makes Classics Classics?* (Dickens, Dostoevsky, and the Brontës: Bakhtin's polyphony, grotesque particularity, unpayable moral debt, and civilizational seismology).
  - `frontier`: *The Speculative Frontier & The RSI Novel* (Hard Sci-Fi classics: Gibson, Stephenson, Egan; the sci-fi hiatus; how Iain M. Banks bypassed the superior-mind paradox in The Culture; the anatomy of the great Recursive Self-Improvement novel).
  - `tropes`: *The Cliché Motherlode & Trope Smorgasbord* (Romance, Sci-Fi, Action/Thrillers, Prestige Gloom, and the satirical Frankenstein Smorgasbord Scaffold).
  - `publishing`: *Publishing Avenues & The Cautionary Tale* (The Coral Hart 370-book-a-year churn vs durable backlist architecture).
- **Interactive Action Hooks**:
  - *"⚡ Load Frankenstein Smorgasbord Beat Sheet into Corkboard"*: Instantly injects the 7-beat satirical hybrid into active corkboard cards.
  - *"🔍 Run Anti-Claude-ese Style Audit"*: Jumps straight to Fleet Studio's style linter.
  - *"🗃️ Search Classics in Calibre"*: Launches instant zero-lock Calibre queries for Dickens, Dostoevsky, and 19th-century literature.

---

## Workspace Architecture

```
writercraft/
├── crates/
│   ├── writercraft-narrative/   # Scene models, emotional valence math, TV Tropes, beat sheets
│   ├── writercraft-core/        # Calibre connector, style guide linter, companion, fleet books, fountain, scholar
│   ├── writercraft-mcp/         # FastMCP stdio JSON-RPC server on port 14121
│   └── writercraft-app/         # Native egui desktop GUI & clap CLI (writercraft-cli)
├── docs/
│   ├── WRITER_AND_AI_DIALECTIC.md # Treatise on AI rhetoric, Claude-ese tells, and 20 anti-patterns
│   ├── PUBLISHING_AVENUES.md    # Publishing funnels, KDP vs Wide, and the Coral Hart case study
│   ├── PRD.md                   # Product requirements & boundary definitions
│   └── TOOLS.md                 # CLI verbs & FastMCP tool reference
├── AGENTS.md                    # Mandatory fleet navigation & multi-agent rules
├── HANDOVER.md                  # Current state, verified actions, next steps
└── Cargo.toml                   # Workspace manifest
```

---

## Verification & Usage

```powershell
# Run comprehensive test suite (16 tests)
cargo test --manifest-path writercraft/Cargo.toml

# Headless smoke test
./target/release/writercraft.exe --smoke

# Calibre Operations
./target/release/writercraft-cli.exe calibre-libs
./target/release/writercraft-cli.exe calibre-search "cyber" --limit 5

# Fleet Book Deep Analysis, Linter & Continuation
./target/release/writercraft-cli.exe book on-metacraft
./target/release/writercraft-cli.exe deep on-metacraft 01-the-breach.md
./target/release/writercraft-cli.exe lint on-metacraft/chapters/01-the-breach.md
./target/release/writercraft-cli.exe continue on-metacraft 01-the-breach.md

# AI Writing Companion & Staged Syndication
./target/release/writercraft-cli.exe draft-next on-metacraft 01-the-breach.md --hint "focus on high voltage"
./target/release/writercraft-cli.exe syndicate on-metacraft/chapters/01-the-breach.md --out-dir outbox

# Narrative, Screenplay & Scholar
./target/release/writercraft-cli.exe scenes
./target/release/writercraft-cli.exe arc
./target/release/writercraft-cli.exe beats --genre romance --title "Tethered Hearts" --protagonist "Lady Genevieve"
./target/release/writercraft-cli.exe screenplay path/to/script.fountain
./target/release/writercraft-cli.exe bibtex path/to/references.bib

# Tier 4: Epistemic Matrix, Idiolect Voice Profiler & Fleet Harmonizer
./target/release/writercraft-cli.exe epistemic
./target/release/writercraft-cli.exe idiolect path/to/script.fountain --pov "Elena"
./target/release/writercraft-cli.exe fleet-harmonize
```

---

## FastMCP Server Tools (Port 14121 & stdio)

| Tool Name | Purpose |
|---|---|
| `writercraft_scenes` | List active corkboard scene cards |
| `writercraft_arc` | Compute narrative tension and emotional valence curves |
| `writercraft_generate_beats` | Generate genre beat sheets (Romance, Thriller, Sci-Fi) |
| `writercraft_tropes` | Query curated TV Tropes catalog |
| `writercraft_parse_fountain` | Parse Fountain screenplay syntax into scene cards & speaking stats |
| `writercraft_readability_audit` | Score Flesch-Kincaid readability and detect AI cliches |
| `writercraft_style_lint` | Deterministic linter against Claude-ese, triads, and contrast frames |
| `writercraft_claim_audit` | Extract unverified `[construction: ...]` tags |
| `writercraft_bibtex_parse` | Parse BibTeX bibliography files |
| `writercraft_fleet_book_inspect` | Inspect one or all 5 fleet books |
| `writercraft_chapter_deep_analysis` | Multi-dimensional chapter analysis |
| `writercraft_continuation_synthesize` | Synthesize 3-beat continuation and prompt |
| `writercraft_companion_draft` | AI companion drafts next chapter given constraints & hints |
| `writercraft_companion_syndicate` | Draft companion Blogpost & HN note staged locally in `outbox/` |
| `writercraft_calibre_libraries` | Auto-discover registered local Calibre libraries |
| `writercraft_calibre_search` | Search books in Calibre library with zero database locks |
| `writercraft_list_craft_essays` | List all 5 in-app literary essays and rhetorical treatises with metadata |
| `writercraft_read_craft_essay` | Read full markdown text of a craft essay (dialectic, canon, frontier, tropes, publishing) |
| `writercraft_frankenstein_beats` | Generate satirical Frankenstein Smorgasbord beat sheet combining 4 genres |
| `writercraft_epistemic_validate` | Validate epistemic knowledge matrix, detect secret leaks and plot holes |
| `writercraft_idiolect_audit` | Profile character dialogue cadence, contractions, TTR, and head-hopping |
| `writercraft_fleet_harmonize` | Audit 6-level autonomy ladder and cluster open claims across fleet books |
