# WriterCraft — AGENTS.md

**Parent hub:** [`mcp-central-docs/standards/RUST_ARTCRAFT_STANDARD.md`](../mcp-central-docs/standards/RUST_ARTCRAFT_STANDARD.md)  
**Containment:** [`mcp-central-docs/standards/rust-artcraft/CONTAINMENT_AND_CRATES.md`](../mcp-central-docs/standards/rust-artcraft/CONTAINMENT_AND_CRATES.md)  
**Style & Dialectic:** [`docs/WRITER_AND_AI_DIALECTIC.md`](docs/WRITER_AND_AI_DIALECTIC.md)  
**Publishing Standards:** [`docs/PUBLISHING_AVENUES.md`](docs/PUBLISHING_AVENUES.md)

WriterCraft is an **ArtCraft-tier multi-persona authoring environment and AI writing companion**. It unifies narrative corkboards, Calibre library integration, Fountain screenplays, academic Typst/BibTeX, and the fleet book corpus (`on-ai-takeover`, `on-hyperwar`, `on-hyperart`, `on-metacraft`, `on-hypermind`).

## Crate Layout

| Crate | Tier | Contents |
|---|---|---|
| `crates/writercraft-narrative` | L1 | Scene models, emotional valence math, TV Tropes taxonomies, beat sheet generator |
| `crates/writercraft-core` | L1 | Calibre zero-lock connector, style guide linter, companion drafter, fleet book auditor, Fountain parser, scholar tools |
| `crates/writercraft-mcp` | L2 | FastMCP 2024-11-05 stdio JSON-RPC server and port 14121 daemon |
| `crates/writercraft-app` | L3 | `writercraft` (eframe/egui native GUI with 5 workspaces) & `writercraft-cli` |

## Non-Negotiable Rules

1. **Zero Unasked Syndication:** AI companion may draft Blogposts and Hacker News notes, but they are staged **strictly locally in `outbox/`**. Never transmit, post, curl, or email without explicit human confirmation.
2. **Zero-Lock Calibre Reads:** Always open Calibre's `metadata.db` with `SQLITE_OPEN_READ_ONLY | SQLITE_OPEN_NO_MUTEX`. Never write to Calibre's database directly or bypass read-only isolation.
3. **No-AI-Tells Enforcement:** All companion drafts must be audited through `StyleLinter` (`crates/writercraft-core/src/style_guide.rs`). No "loadbearing", no "delve", no "tapestry", no reflexive triads, no moralizing conclusion paragraphs.
4. **Living Claim Anchors:** Every unverified empirical assertion in drafts must be marked with `[construction: <task>]`. Never invent a date, person, or fact.
5. **Stdio Invariant:** In `--stdio` mode, `stdout` is 100% line-delimited JSON-RPC. Logs go to `stderr`.
6. **Fleet Multi-Agent Rules:**
   - Max 10 files per commit / invocation.
   - 3+ file edits need a timestamped `.bak` per file written first.
   - Run `cargo test --manifest-path writercraft/Cargo.toml` before claiming completion.

## Verification

```powershell
cargo test --manifest-path writercraft/Cargo.toml
./target/release/writercraft.exe --smoke
./target/release/writercraft-cli.exe lint on-metacraft/chapters/01-the-breach.md
```
