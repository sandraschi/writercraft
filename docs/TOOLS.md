# WriterCraft — Tools Reference (CLI & MCP)

WriterCraft provides identical capabilities across two surfaces:
1. `writercraft-cli <subcommand>` for humans and shell scripts.
2. `writercraft-mcp` (stdio & port 14121) for AI agents over JSON-RPC.

---

## 1. CLI Commands Reference

### Calibre Operations
```powershell
# List all auto-discovered local Calibre libraries
writercraft-cli calibre-libs

# Search books across Calibre library (zero database locks)
writercraft-cli calibre-search "cybernetics" --limit 5
```

### Fleet Book Corpus & Chapter Auditing
```powershell
# Inspect high-level metrics for on-metacraft (or all books)
writercraft-cli book on-metacraft

# Deep multi-dimensional chapter analysis (reading ease, claims, AI tells, tension)
writercraft-cli deep on-metacraft 01-the-breach.md

# Lint manuscript against Claude-ese tells, triads, and contrast frames
writercraft-cli lint on-metacraft/chapters/01-the-breach.md

# Synthesize 3-beat dialectic continuation scaffold
writercraft-cli continue on-metacraft 01-the-breach.md
```

### AI Writing Companion & Staged Syndication
```powershell
# AI drafts next chapter given constraints, hints, and zero-slop rules
writercraft-cli draft-next on-metacraft 01-the-breach.md --hint "focus on high voltage and silicon test coupons" --tone "Coldly Empirical"

# Generate companion Blogpost and Hacker News notes staged locally in ./outbox (never sent unasked)
writercraft-cli syndicate on-metacraft/chapters/01-the-breach.md --out-dir outbox
```

### Narrative, Screenplay & Scholar
```powershell
# List active scene cards
writercraft-cli scenes

# Calculate emotional valence arc and climax scene
writercraft-cli arc

# Generate high-velocity genre beat sheet (romance, thriller, scifi)
writercraft-cli beats --genre romance --title "Tethered Hearts" --protagonist "Lady Genevieve"

# Query curated TV Tropes catalog
writercraft-cli tropes

# Parse Fountain screenplay format (.fountain)
writercraft-cli screenplay script.fountain

# Parse BibTeX bibliography file
writercraft-cli bibtex references.bib

# Audit single markdown file for readability and claims
writercraft-cli audit chapter.md

# Generate satirical Frankenstein Smorgasbord beat sheet
writercraft-cli beats --genre frankenstein --title "Protocol Omega" --protagonist "Dr. Elena Vance"
```

### Craft Delectations & Essays
```powershell
# List all 5 in-app craft essays
writercraft-cli essays

# Read full text of an essay (dialectic, canon, frontier/scifi, tropes, publishing)
writercraft-cli essay scifi
writercraft-cli essay canon
```

---

## 2. FastMCP JSON-RPC Tools Reference

All tools are callable over stdio (`writercraft-mcp --stdio`) or via port `14121`:

| Tool | Parameters | Description |
|---|---|---|
| `writercraft_scenes` | None | Returns active corkboard scene cards. |
| `writercraft_arc` | None | Returns total scenes, mean valence, peak tension scene ID, and tropes. |
| `writercraft_generate_beats` | `genre`, `title`, `protagonist` | Generates structured 15-beat or multi-act outline with emotional valences. |
| `writercraft_tropes` | None | Queries TV Tropes and story mechanics index. |
| `writercraft_parse_fountain` | `text` | Parses Fountain screenplay text, returning scenes, character dialogue word share, and run-time estimates. |
| `writercraft_readability_audit` | `text` | Computes Flesch Reading Ease and Kincaid Grade Level. |
| `writercraft_style_lint` | `text` | Scans for 35+ Claude-ese crutches, reflexive triads, and contrast frames. |
| `writercraft_claim_audit` | `filename`, `text` | Extracts all `[construction: ...]` tags with line numbers. |
| `writercraft_bibtex_parse` | `bib_content` | Parses BibTeX bibliography into structured citation keys. |
| `writercraft_fleet_book_inspect` | `slug_or_path` (optional) | Inspects one or all 5 fleet books (`on-ai-takeover`, `on-hyperwar`, `on-hyperart`, `on-metacraft`, `on-hypermind`). |
| `writercraft_chapter_deep_analysis` | `book_slug`, `chapter_file` | Performs multi-dimensional deep analysis on chapter content. |
| `writercraft_continuation_synthesize`| `book_slug`, `chapter_file` | Synthesizes 3-beat dialectic continuation scaffold and zero-slop drafting prompt. |
| `writercraft_companion_draft` | `book_slug`, `preceding_title`, `ending_excerpt`, `hints`, `thesis`, `tone` | AI companion drafts complete next chapter with zero-slop constraints. |
| `writercraft_companion_syndicate` | `title`, `text`, `out_dir` | Stages Substack blogpost and Hacker News notes locally in outbox/. |
| `writercraft_calibre_libraries` | None | Auto-discovers all local Calibre libraries registered in system config. |
| `writercraft_calibre_search` | `query`, `library_path` (opt), `limit` (opt) | Searches books in Calibre library without locking SQLite database. |
| `writercraft_list_craft_essays` | None | Lists all in-app literary essays and rhetorical treatises with metadata and reading times. |
| `writercraft_read_craft_essay` | `slug` | Reads the full text of a craft essay (dialectic, canon, tropes, publishing). |
| `writercraft_frankenstein_beats` | `title`, `protagonist` | Generates the satirical Frankenstein Smorgasbord beat sheet combining Romance, Sci-Fi, Action, and Prestige Gloom. |
