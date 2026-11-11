//! WriterCraft MCP Server
//!
//! Exposes corkboard scene cards, TV Tropes cataloging,
//! Fountain screenplay analysis, Flesch-Kincaid readability,
//! Calibre library connector, deep chapter analysis, and fleet book continuation
//! as FastMCP JSON-RPC tools on port 14121 and stdio.

use serde_json::{json, Value};
use std::io::{self, BufRead};
use std::path::Path;
use writercraft_core::WriterEngine;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port: u16 = 14121;
    let mut engine = WriterEngine::new();

    // Check if arguments contain --stdio or if stdin is piped
    let args: Vec<String> = std::env::args().collect();
    let is_stdio = args.iter().any(|a| a == "--stdio" || a == "mcp");

    if !is_stdio && args.len() <= 1 {
        // Human/diagnostic start
        let arc = engine.analyze_narrative_arc()?;
        let calibre_libs = engine.discover_calibre_libraries();
        let status = json!({
            "status": "ready",
            "mcp_server": "writercraft",
            "port": port,
            "capabilities": [
                "CorkboardSceneManagement",
                "TropeCatalogIntegration",
                "GenreBeatSheetGeneration",
                "FountainScreenplayParsing",
                "FleschKincaidReadabilityAudit",
                "AiTellDetection",
                "ScholarBibTexParsing",
                "ConstructionClaimLedger",
                "FleetBookInspection",
                "FleetChapterDeepAnalysis",
                "FleetBookContinuationSynthesis",
                "CalibreLibraryConnector"
            ],
            "scene_count": engine.active_scenes.len(),
            "average_valence": arc.average_valence,
            "climax_scene_id": arc.max_tension_scene_id,
            "calibre_libraries_discovered": calibre_libs.len(),
            "fleet_books_connected": [
                "on-ai-takeover",
                "on-hyperwar",
                "on-hyperart",
                "on-metacraft",
                "on-hypermind"
            ]
        });
        println!("[writercraft-mcp] Initializing FastMCP engine on port {}...", port);
        println!("[writercraft-mcp] Status:\n{}", serde_json::to_string_pretty(&status)?);
        return Ok(());
    }

    // stdio JSON-RPC loop for agent protocol
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        if let Ok(req) = serde_json::from_str::<Value>(trimmed) {
            let id = req.get("id").cloned().unwrap_or(Value::Null);
            let method = req.get("method").and_then(|m| m.as_str()).unwrap_or("");

            let response = match method {
                "initialize" => json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "protocolVersion": "2024-11-05",
                        "serverInfo": { "name": "writercraft-mcp", "version": "0.3.1" },
                        "capabilities": { "tools": {} }
                    }
                }),
                "tools/list" => json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "tools": [
                            {
                                "name": "writercraft_scenes",
                                "description": "List all scene cards in the active manuscript corkboard",
                                "inputSchema": { "type": "object", "properties": {} }
                            },
                            {
                                "name": "writercraft_arc",
                                "description": "Analyze narrative tension, emotional valence curve, and climax scene",
                                "inputSchema": { "type": "object", "properties": {} }
                            },
                            {
                                "name": "writercraft_generate_beats",
                                "description": "Generate a high-velocity beat sheet by genre (e.g. romance, thriller, scifi)",
                                "inputSchema": {
                                    "type": "object",
                                    "properties": {
                                        "genre": { "type": "string" },
                                        "title": { "type": "string" },
                                        "protagonist": { "type": "string" }
                                    },
                                    "required": ["genre", "title", "protagonist"]
                                }
                            },
                            {
                                "name": "writercraft_tropes",
                                "description": "Query the curated TV Tropes and story mechanics index",
                                "inputSchema": { "type": "object", "properties": {} }
                            },
                            {
                                "name": "writercraft_parse_fountain",
                                "description": "Parse Fountain screenplay format into scene cards, character speaking stats, and run-time estimates",
                                "inputSchema": {
                                    "type": "object",
                                    "properties": { "text": { "type": "string" } },
                                    "required": ["text"]
                                }
                            },
                            {
                                "name": "writercraft_readability_audit",
                                "description": "Score Flesch-Kincaid readability, sentence complexity, and detect AI tells/cliches",
                                "inputSchema": {
                                    "type": "object",
                                    "properties": { "text": { "type": "string" } },
                                    "required": ["text"]
                                }
                            },
                            {
                                "name": "writercraft_claim_audit",
                                "description": "Extract [construction] unverified claim flags from a manuscript",
                                "inputSchema": {
                                    "type": "object",
                                    "properties": {
                                        "filename": { "type": "string" },
                                        "text": { "type": "string" }
                                    },
                                    "required": ["filename", "text"]
                                }
                            },
                            {
                                "name": "writercraft_bibtex_parse",
                                "description": "Parse BibTeX citations and generate a structured citation index",
                                "inputSchema": {
                                    "type": "object",
                                    "properties": { "bib_content": { "type": "string" } },
                                    "required": ["bib_content"]
                                }
                            },
                            {
                                "name": "writercraft_fleet_book_inspect",
                                "description": "Inspect a fleet book (e.g. on-metacraft, on-ai-takeover) or scan all 5 books",
                                "inputSchema": {
                                    "type": "object",
                                    "properties": { "slug_or_path": { "type": "string" } }
                                }
                            },
                            {
                                "name": "writercraft_chapter_deep_analysis",
                                "description": "Deep multi-dimensional analysis of a chapter (readability, claims, AI tells, tension)",
                                "inputSchema": {
                                    "type": "object",
                                    "properties": {
                                        "book_slug": { "type": "string" },
                                        "chapter_file": { "type": "string" }
                                    },
                                    "required": ["book_slug", "chapter_file"]
                                }
                            },
                            {
                                "name": "writercraft_continuation_synthesize",
                                "description": "Synthesize continuation beats and prompt for a fleet book chapter",
                                "inputSchema": {
                                    "type": "object",
                                    "properties": {
                                        "book_slug": { "type": "string" },
                                        "chapter_file": { "type": "string" }
                                    },
                                    "required": ["book_slug", "chapter_file"]
                                }
                            },
                            {
                                "name": "writercraft_calibre_libraries",
                                "description": "Discover all local Calibre libraries registered on the system",
                                "inputSchema": { "type": "object", "properties": {} }
                            },
                            {
                                "name": "writercraft_calibre_search",
                                "description": "Search books in a Calibre library by query string matching title, author, series, or tags",
                                "inputSchema": {
                                    "type": "object",
                                    "properties": {
                                        "query": { "type": "string" },
                                        "library_path": { "type": "string" },
                                        "limit": { "type": "integer" }
                                    },
                                    "required": ["query"]
                                }
                            },
                            {
                                "name": "writercraft_style_lint",
                                "description": "Deterministic linter against Claude-ese, loadbearing/tapestry tells, triads, and contrast frames",
                                "inputSchema": {
                                    "type": "object",
                                    "properties": { "text": { "type": "string" } },
                                    "required": ["text"]
                                }
                            },
                            {
                                "name": "writercraft_companion_draft",
                                "description": "AI writing companion drafts next chapter given constraints, hints, and zero-slop rules",
                                "inputSchema": {
                                    "type": "object",
                                    "properties": {
                                        "book_slug": { "type": "string" },
                                        "preceding_title": { "type": "string" },
                                        "ending_excerpt": { "type": "string" },
                                        "hints": { "type": "string" },
                                        "thesis": { "type": "string" },
                                        "tone": { "type": "string" }
                                    },
                                    "required": ["book_slug", "preceding_title", "ending_excerpt"]
                                }
                            },
                            {
                                "name": "writercraft_companion_syndicate",
                                "description": "Draft companion Blogpost and Hacker News notes staged locally in outbox/ (never sent unasked)",
                                "inputSchema": {
                                    "type": "object",
                                    "properties": {
                                        "title": { "type": "string" },
                                        "text": { "type": "string" },
                                        "out_dir": { "type": "string" }
                                    },
                                    "required": ["title", "text"]
                                }
                            },
                            {
                                "name": "writercraft_list_craft_essays",
                                "description": "List all literary essays, rhetorical treatises, trope catalogs, and publishing guides",
                                "inputSchema": { "type": "object", "properties": {} }
                            },
                            {
                                "name": "writercraft_read_craft_essay",
                                "description": "Read full text of a craft essay by slug (dialectic, canon, tropes, publishing)",
                                "inputSchema": {
                                    "type": "object",
                                    "properties": {
                                        "slug": { "type": "string" }
                                    },
                                    "required": ["slug"]
                                }
                            },
                            {
                                "name": "writercraft_frankenstein_beats",
                                "description": "Generate the satirical Frankenstein Smorgasbord beat sheet combining Romance, Sci-Fi, Thrillers, and Prestige Gloom",
                                "inputSchema": {
                                    "type": "object",
                                    "properties": {
                                        "title": { "type": "string" },
                                        "protagonist": { "type": "string" }
                                    }
                                }
                            },
                            {
                                "name": "writercraft_epistemic_validate",
                                "description": "Validate epistemic secret matrix, plot holes, and Fabula vs Syuzhet timeline",
                                "inputSchema": { "type": "object", "properties": {} }
                            },
                            {
                                "name": "writercraft_idiolect_audit",
                                "description": "Profile character voice distinctiveness, cadence variance, lexical richness (TTR), and POV head-hopping",
                                "inputSchema": {
                                    "type": "object",
                                    "properties": {
                                        "text": { "type": "string" },
                                        "declared_pov": { "type": "string" }
                                    },
                                    "required": ["text"]
                                }
                            },
                            {
                                "name": "writercraft_fleet_harmonize",
                                "description": "Harmonize terminology, autonomy ladders, and open [construction] claims across the 5 fleet books",
                                "inputSchema": { "type": "object", "properties": {} }
                            }
                        ]
                    }
                }),
                "tools/call" => {
                    let tool_name = req.get("params")
                        .and_then(|p| p.get("name"))
                        .and_then(|n| n.as_str())
                        .unwrap_or("");
                    let arguments = req.get("params")
                        .and_then(|p| p.get("arguments"))
                        .cloned()
                        .unwrap_or(json!({}));

                    let repos_dir = if Path::new("on-metacraft").exists() {
                        Path::new(".")
                    } else {
                        Path::new("..")
                    };

                    let call_result: Value = match tool_name {
                        "writercraft_scenes" => json!({ "scenes": engine.active_scenes }),
                        "writercraft_arc" => match engine.analyze_narrative_arc() {
                            Ok(arc) => json!(arc),
                            Err(e) => json!({ "error": e.to_string() }),
                        },
                        "writercraft_generate_beats" => {
                            let g = arguments.get("genre").and_then(|v| v.as_str()).unwrap_or("scifi");
                            let t = arguments.get("title").and_then(|v| v.as_str()).unwrap_or("Untitled");
                            let p = arguments.get("protagonist").and_then(|v| v.as_str()).unwrap_or("Protagonist");
                            let sheet = engine.generate_beats(g, t, p);
                            json!(sheet)
                        }
                        "writercraft_tropes" => json!({ "tropes": engine.get_tropes() }),
                        "writercraft_parse_fountain" => {
                            let text = arguments.get("text").and_then(|v| v.as_str()).unwrap_or("");
                            json!(engine.parse_fountain(text))
                        }
                        "writercraft_readability_audit" => {
                            let text = arguments.get("text").and_then(|v| v.as_str()).unwrap_or("");
                            json!(engine.audit_readability(text))
                        }
                        "writercraft_claim_audit" => {
                            let fname = arguments.get("filename").and_then(|v| v.as_str()).unwrap_or("manuscript.md");
                            let text = arguments.get("text").and_then(|v| v.as_str()).unwrap_or("");
                            json!({ "claims": engine.audit_claims(fname, text) })
                        }
                        "writercraft_bibtex_parse" => {
                            let bib = arguments.get("bib_content").and_then(|v| v.as_str()).unwrap_or("");
                            json!({ "entries": engine.parse_bibtex(bib) })
                        }
                        "writercraft_fleet_book_inspect" => {
                            let target = arguments.get("slug_or_path").and_then(|v| v.as_str()).unwrap_or("");
                            if target.is_empty() || target == "all" {
                                json!({ "fleet_books": engine.scan_all_fleet_books(repos_dir) })
                            } else {
                                let p = if Path::new(target).exists() {
                                    Path::new(target).to_path_buf()
                                } else {
                                    repos_dir.join(target)
                                };
                                match engine.inspect_fleet_book(&p) {
                                    Ok(res) => json!(res),
                                    Err(e) => json!({ "error": e }),
                                }
                            }
                        }
                        "writercraft_chapter_deep_analysis" => {
                            let book_slug = arguments.get("book_slug").and_then(|v| v.as_str()).unwrap_or("on-metacraft");
                            let chapter_file = arguments.get("chapter_file").and_then(|v| v.as_str()).unwrap_or("01-the-breach.md");
                            let book_dir = repos_dir.join(book_slug);
                            match engine.read_chapter_content(&book_dir, chapter_file) {
                                Ok(content) => {
                                    let analysis = engine.analyze_chapter_deep(chapter_file, &content);
                                    json!(analysis)
                                }
                                Err(e) => json!({ "error": e }),
                            }
                        }
                        "writercraft_continuation_synthesize" => {
                            let book_slug = arguments.get("book_slug").and_then(|v| v.as_str()).unwrap_or("on-metacraft");
                            let chapter_file = arguments.get("chapter_file").and_then(|v| v.as_str()).unwrap_or("01-the-breach.md");
                            let book_dir = repos_dir.join(book_slug);
                            match engine.read_chapter_content(&book_dir, chapter_file) {
                                Ok(content) => {
                                    let analysis = engine.analyze_chapter_deep(chapter_file, &content);
                                    let scaffold = engine.synthesize_continuation(book_slug, &analysis, None);
                                    json!(scaffold)
                                }
                                Err(e) => json!({ "error": e }),
                            }
                        }
                        "writercraft_calibre_libraries" => {
                            json!({ "libraries": engine.discover_calibre_libraries() })
                        }
                        "writercraft_calibre_search" => {
                            let query = arguments.get("query").and_then(|v| v.as_str()).unwrap_or("");
                            let limit = arguments.get("limit").and_then(|v| v.as_u64()).unwrap_or(20) as usize;
                            let lib_path_arg = arguments.get("library_path").and_then(|v| v.as_str());

                            let lib_path = match lib_path_arg {
                                Some(lp) => Path::new(lp).to_path_buf(),
                                None => {
                                    let libs = engine.discover_calibre_libraries();
                                    if let Some(first) = libs.first() {
                                        Path::new(&first.path).to_path_buf()
                                    } else {
                                        Path::new("L:/Multimedia Files/Written Word/Calibre-Bibliothek").to_path_buf()
                                    }
                                }
                            };

                            match engine.search_calibre(&lib_path, query, limit) {
                                Ok(books) => json!({
                                    "library": lib_path.to_string_lossy(),
                                    "query": query,
                                    "total_found": books.len(),
                                    "books": books
                                }),
                                Err(e) => json!({ "error": e }),
                            }
                        }
                        "writercraft_style_lint" => {
                            let text = arguments.get("text").and_then(|v| v.as_str()).unwrap_or("");
                            json!(engine.audit_style_guide(text))
                        }
                        "writercraft_companion_draft" => {
                            let req = writercraft_core::companion::ChapterDraftRequest {
                                book_slug: arguments.get("book_slug").and_then(|v| v.as_str()).unwrap_or("on-metacraft").to_string(),
                                preceding_chapter_file: "01-the-breach.md".to_string(),
                                preceding_chapter_title: arguments.get("preceding_title").and_then(|v| v.as_str()).unwrap_or("Preceding Chapter").to_string(),
                                ending_excerpt: arguments.get("ending_excerpt").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                                writer_hints: arguments.get("hints").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                                target_rung_or_thesis: arguments.get("thesis").and_then(|v| v.as_str()).unwrap_or("The Physical Firewall").to_string(),
                                tone: arguments.get("tone").and_then(|v| v.as_str()).unwrap_or("Coldly Empirical").to_string(),
                                target_word_count: 1200,
                                enforce_zero_slop: true,
                            };
                            json!(engine.companion_draft_chapter(&req))
                        }
                        "writercraft_companion_syndicate" => {
                            let title = arguments.get("title").and_then(|v| v.as_str()).unwrap_or("Untitled");
                            let text = arguments.get("text").and_then(|v| v.as_str()).unwrap_or("");
                            let out_dir_str = arguments.get("out_dir").and_then(|v| v.as_str()).unwrap_or("outbox");
                            let out_path = Path::new(out_dir_str);
                            match engine.companion_syndicate(title, text, Some(out_path)) {
                                Ok(res) => json!(res),
                                Err(e) => json!({ "error": e }),
                            }
                        }
                        "writercraft_list_craft_essays" => {
                            json!({ "essays": engine.list_craft_essays() })
                        }
                        "writercraft_read_craft_essay" => {
                            let slug = arguments.get("slug").and_then(|v| v.as_str()).unwrap_or("dialectic");
                            match engine.get_craft_essay(slug) {
                                Some(essay) => json!({
                                    "slug": essay.slug,
                                    "title": essay.title,
                                    "subtitle": essay.subtitle,
                                    "category": essay.category,
                                    "reading_time_mins": essay.reading_time_mins,
                                    "word_count": essay.word_count,
                                    "abstract": essay.abstract_text,
                                    "content_markdown": essay.content_markdown
                                }),
                                None => json!({ "error": format!("Unknown essay slug: {}", slug) }),
                            }
                        }
                        "writercraft_frankenstein_beats" => {
                            let title = arguments.get("title").and_then(|v| v.as_str()).unwrap_or("The Sovereign Protocol");
                            let protagonist = arguments.get("protagonist").and_then(|v| v.as_str()).unwrap_or("Dr. Elena Vance");
                            json!(engine.generate_frankenstein_beats(title, protagonist))
                        }
                        "writercraft_epistemic_validate" => {
                            let graph = engine.mock_epistemic_graph();
                            let anomalies = engine.validate_epistemic_graph(&graph);
                            let timeline = graph.build_timeline();
                            json!({
                                "facts": graph.facts,
                                "timeline_beats": timeline,
                                "anomalies_detected": anomalies.len(),
                                "anomalies": anomalies
                            })
                        }
                        "writercraft_idiolect_audit" => {
                            let text = arguments.get("text").and_then(|v| v.as_str()).unwrap_or("");
                            let pov = arguments.get("declared_pov").and_then(|v| v.as_str()).unwrap_or("Protagonist");
                            let mut turns: Vec<(String, String)> = Vec::new();
                            let mut cur_char: Option<String> = None;
                            let mut cur_speech = String::new();

                            for line in text.lines() {
                                let trimmed = line.trim();
                                if trimmed.is_empty() {
                                    if let Some(ch) = cur_char.take() {
                                        if !cur_speech.trim().is_empty() {
                                            turns.push((ch, cur_speech.trim().to_string()));
                                            cur_speech.clear();
                                        }
                                    }
                                    continue;
                                }
                                if trimmed.chars().all(|c| c.is_uppercase() || c.is_whitespace() || c == '(' || c == ')' || c == '.') && !trimmed.starts_with("EXT.") && !trimmed.starts_with("INT.") {
                                    if let Some(ch) = cur_char.take() {
                                        if !cur_speech.trim().is_empty() {
                                            turns.push((ch, cur_speech.trim().to_string()));
                                            cur_speech.clear();
                                        }
                                    }
                                    let clean = trimmed.split('(').next().unwrap_or(trimmed).trim().to_string();
                                    cur_char = Some(clean);
                                } else if cur_char.is_some() && !trimmed.starts_with('(') {
                                    cur_speech.push(' ');
                                    cur_speech.push_str(trimmed);
                                }
                            }
                            if let Some(ch) = cur_char {
                                if !cur_speech.trim().is_empty() {
                                    turns.push((ch, cur_speech.trim().to_string()));
                                }
                            }

                            let mut report = engine.audit_dialogue_turns(&turns);
                            let other_chars: Vec<&str> = report.character_profiles.iter().map(|p| p.character.as_str()).collect();
                            report.head_hopping_alerts = engine.scan_pov_head_hopping(text, pov, &other_chars);
                            json!(report)
                        }
                        "writercraft_fleet_harmonize" => {
                            json!(engine.harmonize_fleet_corpus(repos_dir))
                        }
                        _ => json!({ "error": format!("Unknown tool: {}", tool_name) }),
                    };

                    json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "result": {
                            "content": [{
                                "type": "text",
                                "text": serde_json::to_string_pretty(&call_result).unwrap_or_default()
                            }]
                        }
                    })
                }
                _ => json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "error": { "code": -32601, "message": "Method not found" }
                }),
            };

            use std::io::Write;
            let resp_str = serde_json::to_string(&response).unwrap_or_default();
            let _ = writeln!(stdout, "{}", resp_str);
            let _ = stdout.flush();
        }
    }

    Ok(())
}
