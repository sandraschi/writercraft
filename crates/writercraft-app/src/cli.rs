use clap::{Parser, Subcommand};
use std::fs;
use std::path::{Path, PathBuf};
use writercraft_core::WriterEngine;

#[derive(Parser)]
#[command(name = "writercraft-cli", about = "WriterCraft Multi-Persona Authoring, Calibre & Narrative CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all scene cards in the active manuscript
    Scenes,
    /// Analyze narrative tension and emotional valence arc
    Arc,
    /// Generate a structured beat sheet by genre (romance, thriller, scifi, etc.)
    Beats {
        #[arg(short, long, default_value = "romance")]
        genre: String,
        #[arg(short, long, default_value = "Untitled Project")]
        title: String,
        #[arg(short, long, default_value = "Protagonist")]
        protagonist: String,
    },
    /// List the curated story mechanics and TV Tropes catalog
    Tropes,
    /// Parse a Fountain screenplay file (.fountain)
    Screenplay {
        /// Path to the .fountain file
        path: PathBuf,
    },
    /// Audit a manuscript file for Flesch-Kincaid readability, AI tells, and [construction] claims
    Audit {
        /// Path to the markdown file
        path: PathBuf,
    },
    /// Parse a BibTeX bibliography file
    Bibtex {
        /// Path to the .bib file
        path: PathBuf,
    },
    /// Inspect one of the five fleet books (or all of them)
    Book {
        /// Book slug or directory (e.g. on-metacraft, on-ai-takeover, or omit for all)
        slug_or_dir: Option<String>,
    },
    /// Perform deep multi-dimensional analysis on a specific chapter
    Deep {
        /// Book slug (e.g. on-metacraft)
        book: String,
        /// Chapter file name (e.g. 01-the-breach.md)
        chapter: String,
    },
    /// Synthesize continuation beats and prompt for a fleet book chapter
    Continue {
        /// Book slug (e.g. on-metacraft)
        book: String,
        /// Chapter file name (e.g. 01-the-breach.md)
        chapter: String,
    },
    /// List all discovered Calibre libraries on the system
    CalibreLibs,
    /// Search books in a Calibre library
    CalibreSearch {
        /// Search query (title, author, series, tag)
        query: String,
        /// Optional library path override (defaults to first discovered)
        #[arg(short, long)]
        library: Option<PathBuf>,
        /// Max results limit (default 20)
        #[arg(short, long, default_value = "20")]
        limit: usize,
    },
    /// Lint manuscript file against Claude-ese, loadbearing/tapestry tells, triads, and contrast frames
    Lint {
        /// Path to the markdown file
        path: PathBuf,
    },
    /// AI Writing Companion: Draft the next chapter given preceding context, constraints and hints
    DraftNext {
        /// Book slug (e.g. on-metacraft)
        book: String,
        /// Preceding chapter file (e.g. 01-the-breach.md)
        chapter: String,
        /// Writing hints and specific milestones to include
        #[arg(short, long)]
        hint: Option<String>,
        /// Target thesis or autonomy ladder rung
        #[arg(short, long)]
        thesis: Option<String>,
        /// Prose tone (e.g. "Coldly Empirical", "Hardboiled Dialectic")
        #[arg(short, long, default_value = "Coldly Empirical")]
        tone: String,
    },
    /// Generate companion Blogpost & Hacker News notes staged to local outbox (never sent unasked)
    Syndicate {
        /// Path to the source markdown chapter
        path: PathBuf,
        /// Staging directory for generated drafts (defaults to ./outbox)
        #[arg(short, long, default_value = "outbox")]
        out_dir: PathBuf,
    },
    /// List all available Craft Delectations and literary essays
    Essays,
    /// Read a specific Craft Delectation essay by slug (dialectic, canon, tropes, publishing)
    Essay {
        /// Essay slug
        slug: String,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let mut engine = WriterEngine::new();
    let repos_dir = if Path::new("on-metacraft").exists() {
        Path::new(".")
    } else {
        Path::new("..")
    };

    match cli.command {
        Commands::Scenes => {
            println!("{}", serde_json::to_string_pretty(&engine.active_scenes)?);
        }
        Commands::Arc => {
            let arc = engine.analyze_narrative_arc()?;
            println!("{}", serde_json::to_string_pretty(&arc)?);
        }
        Commands::Beats { genre, title, protagonist } => {
            let sheet = engine.generate_beats(&genre, &title, &protagonist);
            println!("{}", serde_json::to_string_pretty(&sheet)?);
        }
        Commands::Tropes => {
            let tropes = engine.get_tropes();
            println!("{}", serde_json::to_string_pretty(&tropes)?);
        }
        Commands::Screenplay { path } => {
            let content = fs::read_to_string(&path)?;
            let analysis = engine.parse_fountain(&content);
            println!("{}", serde_json::to_string_pretty(&analysis)?);
        }
        Commands::Audit { path } => {
            let content = fs::read_to_string(&path)?;
            let fname = path.file_name().and_then(|n| n.to_str()).unwrap_or("document.md");
            let read_audit = engine.audit_readability(&content);
            let claims = engine.audit_claims(fname, &content);

            let combined = serde_json::json!({
                "file": fname,
                "readability": read_audit,
                "unverified_claims": claims,
            });
            println!("{}", serde_json::to_string_pretty(&combined)?);
        }
        Commands::Bibtex { path } => {
            let content = fs::read_to_string(&path)?;
            let entries = engine.parse_bibtex(&content);
            println!("{}", serde_json::to_string_pretty(&entries)?);
        }
        Commands::Book { slug_or_dir } => {
            match slug_or_dir {
                Some(target) => {
                    let p = if Path::new(&target).exists() {
                        PathBuf::from(target)
                    } else {
                        repos_dir.join(&target)
                    };
                    match engine.inspect_fleet_book(&p) {
                        Ok(res) => println!("{}", serde_json::to_string_pretty(&res)?),
                        Err(e) => eprintln!("Error inspecting fleet book: {}", e),
                    }
                }
                None => {
                    let books = engine.scan_all_fleet_books(repos_dir);
                    println!("{}", serde_json::to_string_pretty(&books)?);
                }
            }
        }
        Commands::Deep { book, chapter } => {
            let book_dir = repos_dir.join(&book);
            match engine.read_chapter_content(&book_dir, &chapter) {
                Ok(content) => {
                    let analysis = engine.analyze_chapter_deep(&chapter, &content);
                    println!("{}", serde_json::to_string_pretty(&analysis)?);
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Commands::Continue { book, chapter } => {
            let book_dir = repos_dir.join(&book);
            match engine.read_chapter_content(&book_dir, &chapter) {
                Ok(content) => {
                    let analysis = engine.analyze_chapter_deep(&chapter, &content);
                    let scaffold = engine.synthesize_continuation(&book, &analysis, None);
                    println!("{}", serde_json::to_string_pretty(&scaffold)?);
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Commands::CalibreLibs => {
            let libs = engine.discover_calibre_libraries();
            println!("{}", serde_json::to_string_pretty(&libs)?);
        }
        Commands::CalibreSearch { query, library, limit } => {
            let lib_path = match library {
                Some(p) => p,
                None => {
                    let libs = engine.discover_calibre_libraries();
                    if let Some(first) = libs.first() {
                        PathBuf::from(&first.path)
                    } else {
                        PathBuf::from("L:/Multimedia Files/Written Word/Calibre-Bibliothek")
                    }
                }
            };
            match engine.search_calibre(&lib_path, &query, limit) {
                Ok(books) => println!("{}", serde_json::to_string_pretty(&books)?),
                Err(e) => eprintln!("Calibre search error: {}", e),
            }
        }
        Commands::Lint { path } => {
            let content = fs::read_to_string(&path)?;
            let audit = engine.audit_style_guide(&content);
            println!("{}", serde_json::to_string_pretty(&audit)?);
        }
        Commands::DraftNext { book, chapter, hint, thesis, tone } => {
            let book_dir = repos_dir.join(&book);
            match engine.read_chapter_content(&book_dir, &chapter) {
                Ok(content) => {
                    let analysis = engine.analyze_chapter_deep(&chapter, &content);
                    let req = writercraft_core::companion::ChapterDraftRequest {
                        book_slug: book,
                        preceding_chapter_file: chapter,
                        preceding_chapter_title: analysis.title,
                        ending_excerpt: analysis.closing_excerpt,
                        writer_hints: hint.unwrap_or_else(|| "Confront automated software speed with physical firewall constraints".into()),
                        target_rung_or_thesis: thesis.unwrap_or_else(|| "The Physical Hardware Firewall".into()),
                        tone,
                        target_word_count: 1200,
                        enforce_zero_slop: true,
                    };
                    let drafted = engine.companion_draft_chapter(&req);
                    println!("{}", serde_json::to_string_pretty(&drafted)?);
                }
                Err(e) => eprintln!("Error reading chapter: {}", e),
            }
        }
        Commands::Syndicate { path, out_dir } => {
            let content = fs::read_to_string(&path)?;
            let title = path.file_stem().and_then(|s| s.to_str()).unwrap_or("Chapter");
            match engine.companion_syndicate(title, &content, Some(&out_dir)) {
                Ok(syn) => println!("{}", serde_json::to_string_pretty(&syn)?),
                Err(e) => eprintln!("Error staging syndication drafts: {}", e),
            }
        }
        Commands::Essays => {
            let list = engine.list_craft_essays();
            println!("{}", serde_json::to_string_pretty(&list)?);
        }
        Commands::Essay { slug } => {
            match engine.get_craft_essay(&slug) {
                Some(essay) => {
                    println!("\n=== {} ===", essay.title);
                    println!("{}\n", essay.subtitle);
                    println!("Category: {} | Reading Time: ~{} min | Words: {}\n", essay.category, essay.reading_time_mins, essay.word_count);
                    println!("--------------------------------------------------");
                    println!("{}", essay.content_markdown);
                }
                None => {
                    eprintln!("Unknown essay slug '{}'. Available: dialectic, canon, tropes, publishing", slug);
                    std::process::exit(1);
                }
            }
        }
    }
    Ok(())
}
