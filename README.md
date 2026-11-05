# WriterCraft & ScholarCraft

**WriterCraft** is an ArtCraft-tier local-first narrative architecture, TV Tropes indexation, corkboard scene manager, and academic publishing studio running entirely offline without cloud extraction.

Features:
- **Corkboard Scene Cards**: Formalized dramatically via POV character, dramatic goal, conflict obstacle, disaster outcome, and emotional valence $[-1.0, +1.0]$.
- **Narrative Arc & Tension Analysis**: Algorithmic tension curve calculation, climax scene identification, and trope saturation metrics.
- **TV Tropes Integration**: Curated catalog of narrative mechanics ("Call to Adventure", "Ghost in the Machine", "Cassandra Truth").
- **FastMCP Protocol**: Port `14121` stdio JSON-RPC interface for multi-agent narrative brainstorming swarms.

---

## Workspace Architecture

1. `crates/writercraft-narrative`: Scene models, emotional valence mathematics, and TV Tropes taxonomies.
2. `crates/writercraft-core`: Manuscript tension analyzer, chapter ordering, and DatabaseCraft persistence.
3. `crates/writercraft-mcp`: FastMCP JSON-RPC server on port `14121`.
4. `crates/writercraft-app`: Native GUI (`eframe`/`egui`) and CLI tool (`writercraft-cli`).

---

## Verification & Usage

```powershell
# Run test suite
cargo test

# Headless smoke test
./target/debug/writercraft.exe --smoke

# CLI Operations
./target/debug/writercraft-cli.exe scenes
./target/debug/writercraft-cli.exe arc
```

## Lineage — the Storytold craft-app pattern

Storytold ArtCraft ([storytold/artcraft](https://github.com/storytold/artcraft) — "the IDE for artists") was the starting point. The 16 craft apps in the Storytold stable each ship one binary that is both the human CLI and the agent MCP server (stdio), headless-first, with `run.json` + report artifacts. That modus operandi and harness is what this repo implements; the code here is independent, the pattern is shared.
