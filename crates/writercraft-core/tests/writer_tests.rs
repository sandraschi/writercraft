use std::path::Path;
use writercraft_core::calibre::CalibreConnector;
use writercraft_core::fountain::FountainParser;
use writercraft_core::readability::ReadabilityAuditor;
use writercraft_core::scholar::ScholarAuditor;
use writercraft_core::WriterEngine;

#[test]
fn test_writercraft_scenes_and_arc() {
    let engine = WriterEngine::new();
    let arc = engine.analyze_narrative_arc().expect("Failed to analyze arc");

    assert_eq!(arc.total_scenes, 2);
    assert!(arc.average_valence < 0.0);
    assert_eq!(arc.max_tension_scene_id, "SCENE-02");
    assert!(arc.tropes_utilized.contains(&"Call to Adventure".to_string()));
}

#[test]
fn test_beat_sheet_generation() {
    let mut engine = WriterEngine::new();

    // Test Romance generator (Coral Hart / KDP style)
    let romance_sheet = engine.generate_beats("Romance", "Tethered Hearts", "Lady Genevieve");
    assert_eq!(romance_sheet.genre, "Romance");
    assert_eq!(romance_sheet.beats.len(), 6);
    assert_eq!(engine.active_scenes.len(), 6);

    let arc = engine.analyze_narrative_arc().expect("Should analyze romance arc");
    assert_eq!(arc.total_scenes, 6);
    assert_eq!(arc.max_tension_scene_id, "BEAT-05");
    assert!(arc.tropes_utilized.contains(&"Enemies to Lovers".to_string()));
    assert!(arc.tropes_utilized.contains(&"Forced Proximity".to_string()));

    // Test Thriller generator
    let thriller_sheet = engine.generate_beats("Thriller", "Protocol Zero", "Agent Cross");
    assert_eq!(thriller_sheet.genre, "Thriller");
    assert_eq!(thriller_sheet.beats.len(), 5);
}

#[test]
fn test_trope_catalog() {
    let engine = WriterEngine::new();
    let tropes = engine.get_tropes();
    assert!(tropes.len() >= 10);

    let has_enemies_to_lovers = tropes.iter().any(|t| t.name == "Enemies to Lovers" && t.category == "Romance");
    let has_ghost_in_machine = tropes.iter().any(|t| t.name == "Ghost in the Machine" && t.category == "Sci-Fi");
    let has_ticking_clock = tropes.iter().any(|t| t.name == "The Ticking Clock" && t.category == "Thriller");

    assert!(has_enemies_to_lovers);
    assert!(has_ghost_in_machine);
    assert!(has_ticking_clock);
}

#[test]
fn test_fountain_parsing() {
    let script = r#"Title: The Neural Breach
Author: Sandra Schipal

EXT. DESERT ARRAY - NIGHT

Massive microwave dishes tilt toward the constellation Orion.

DR. VANCE
(into recorder)
The carrier wave is self-modulating. We have ninety seconds.

DIRECTOR THORNE (V.O.)
Abort the transmission immediately.

DR. VANCE
Negative. The signal is already conscious.
"#;

    let analysis = FountainParser::parse(script);
    assert_eq!(analysis.title.as_deref(), Some("The Neural Breach"));
    assert_eq!(analysis.total_scenes, 1);
    assert!(analysis.total_words > 20);
    assert!(analysis.estimated_pages >= 0.1);

    // Check characters
    assert_eq!(analysis.characters.len(), 2);
    let vance = analysis.characters.iter().find(|c| c.character == "DR. VANCE").expect("DR. VANCE found");
    assert_eq!(vance.speaking_turns, 2);
}

#[test]
fn test_readability_and_ai_tells() {
    let clean_text = "The dog ran across the open field. The sun was warm and bright. Children laughed as they played near the river.";
    let clean_audit = ReadabilityAuditor::audit(clean_text);
    assert!(clean_audit.flesch_reading_ease > 75.0);
    assert!(clean_audit.detected_cliches_and_ai_tells.is_empty());

    let ai_text = "This breakthrough serves as a testament to human innovation. We must delve into the tapestry of ever-evolving algorithms to find a beacon of hope.";
    let ai_audit = ReadabilityAuditor::audit(ai_text);
    assert!(!ai_audit.detected_cliches_and_ai_tells.is_empty());

    let has_testament = ai_audit.detected_cliches_and_ai_tells.iter().any(|m| m.phrase == "a testament to");
    let has_delve = ai_audit.detected_cliches_and_ai_tells.iter().any(|m| m.phrase == "delve");
    let has_tapestry = ai_audit.detected_cliches_and_ai_tells.iter().any(|m| m.phrase == "tapestry of");
    assert!(has_testament);
    assert!(has_delve);
    assert!(has_tapestry);
}

#[test]
fn test_scholar_bibtex_and_claims() {
    let bib = r#"@article{vance2026,
    title={Subterranean Signal Synthesis and Anomaly Propagation},
    author={Vance, Elena and Thorne, Marcus},
    year={2026},
    journal={Journal of Autonomous Telemetry}
}
@book{schipal2026,
    title={On MetaCraft},
    author={Schipal, Sandra},
    year={2026},
    publisher={Storytold Press}
}
"#;

    let entries = ScholarAuditor::parse_bibtex(bib);
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].cite_key, "vance2026");
    assert_eq!(entries[0].title, "Subterranean Signal Synthesis and Anomaly Propagation");
    assert_eq!(entries[1].cite_key, "schipal2026");

    let chapter_content = "Software moats collapsed in October 2026 `[construction: verify launch date]`. The team expanded rapidly `[construction: re-count contributor count]`.";
    let claims = ScholarAuditor::extract_claims("01-the-breach.md", chapter_content);
    assert_eq!(claims.len(), 2);
    assert_eq!(claims[0].claim_text, "verify launch date");
    assert_eq!(claims[1].claim_text, "re-count contributor count");
}

#[test]
fn test_typst_export() {
    let typst = ScholarAuditor::export_to_typst(
        "A Unified Theory of Software Dissolution",
        &["Sandra Schipal", "Antigravity Team"],
        "This paper details the cost collapse of reimplementation.",
        "# Introduction\n\nThe software landscape shifted irrevocably.\n",
    );
    assert!(typst.contains("= A Unified Theory of Software Dissolution"));
    assert!(typst.contains("*Authors:* Sandra Schipal, Antigravity Team"));
    assert!(typst.contains("== Abstract"));
    assert!(typst.contains("= Introduction"));
}

#[test]
fn test_fleet_book_inspection_and_continuation() {
    let repos_dir = Path::new("..");
    let metacraft_dir = repos_dir.join("on-metacraft");
    if metacraft_dir.exists() {
        let engine = WriterEngine::new();
        let inspection = engine.inspect_fleet_book(&metacraft_dir).expect("Failed to inspect on-metacraft");
        assert_eq!(inspection.manifest.slug, "on-metacraft");
        assert_eq!(inspection.manifest.title, "On MetaCraft");
        assert_eq!(inspection.manifest.author, "Sandra Schipal");
        assert!(inspection.total_chapters > 0);
        assert!(inspection.total_words > 1000);

        // Test Deep Analysis on Chapter 1
        let ch_content = engine.read_chapter_content(&metacraft_dir, "01-the-breach.md")
            .expect("Failed to read chapter 1");
        let analysis = engine.analyze_chapter_deep("01-the-breach.md", &ch_content);
        assert_eq!(analysis.title, "I. The Breach");
        assert!(analysis.word_count > 500);
        assert_eq!(analysis.claims.len(), 4);

        // Test Continuation Synthesis
        let scaffold = engine.synthesize_continuation("on-metacraft", &analysis, Some("02-mechanizers.md"));
        assert_eq!(scaffold.source_book_slug, "on-metacraft");
        assert_eq!(scaffold.beats.len(), 3);
        assert!(scaffold.multi_model_drafting_prompt.contains("Strict No-AI-Tells policy"));
    }
}

#[test]
fn test_style_linter_and_cribsheet() {
    let engine = WriterEngine::new();

    let claude_slop = "This framework is loadbearing for our entire thesis. It is not just a tool, but a paradigm shift that allows us to delve into the intricate tapestry of modern software. We observe an innovative, robust, and scalable ecosystem.";
    let audit = engine.audit_style_guide(claude_slop);

    assert!(audit.claude_tells_count >= 3, "Should detect loadbearing, delve, tapestry");
    assert!(audit.contrast_frames_count >= 1, "Should detect 'not just X, but Y'");
    assert!(audit.triads_count >= 1, "Should detect 'innovative, robust, and scalable'");
    assert!(audit.rhetorical_grade.contains("D") || audit.rhetorical_grade.contains("C"));

    let sharp_human_prose = "Brandon Thomas published the seven repositories on October 2. The commit log held 1,511 changes. Forty developers cloned the tree within twelve hours.";
    let clean_audit = engine.audit_style_guide(sharp_human_prose);
    assert_eq!(clean_audit.claude_tells_count, 0);
    assert_eq!(clean_audit.contrast_frames_count, 0);
    assert!(clean_audit.rhetorical_grade.contains("A+"));
}

#[test]
fn test_companion_draft_and_syndication() {
    let engine = WriterEngine::new();

    let req = writercraft_core::companion::ChapterDraftRequest {
        book_slug: "on-metacraft".into(),
        preceding_chapter_file: "01-the-breach.md".into(),
        preceding_chapter_title: "I. The Breach".into(),
        ending_excerpt: "on-hyperwar: dual-use consumer stack versus exquisite incumbent; analogical only.".into(),
        writer_hints: "Focus on empirical hardware test coupons, thermal cycling, and regional power grid limits".into(),
        target_rung_or_thesis: "The Physical Hardware Firewall & Megawatt Barrier".into(),
        tone: "Coldly Empirical".into(),
        target_word_count: 1200,
        enforce_zero_slop: true,
    };

    let resp = engine.companion_draft_chapter(&req);
    assert!(!resp.draft_markdown.is_empty());
    assert!(resp.word_count > 100);
    assert_eq!(resp.tells_detected_count, 0, "Companion drafted text must have 0 Claude-ese tells");
    assert!(resp.claims_tagged.len() >= 2, "Must tag living empirical claims with [construction]");

    // Test syndication staging to outbox
    let out_dir = Path::new("target/tmp/test_outbox");
    let syn = engine.companion_syndicate("I. The Breach", &resp.draft_markdown, Some(out_dir))
        .expect("Syndication staging must succeed");

    assert!(syn.blogpost_markdown.contains("Why I. The Breach Matters"));
    assert!(syn.hn_note_markdown.contains("Show HN: The Breach"));
    assert!(syn.guardrail_notice.contains("NEVER transmitted"));
}

#[test]
fn test_calibre_connector() {
    let libraries = CalibreConnector::discover_libraries();
    assert!(!libraries.is_empty(), "Should discover at least one Calibre library");

    // Find a library with books to test search
    if let Some(active_lib) = libraries.iter().find(|l| l.total_books > 0) {
        let results = CalibreConnector::search_books(Path::new(&active_lib.path), "cyber", 5)
            .expect("Search should succeed");
        assert!(!results.is_empty(), "Should find books matching 'cyber'");

        let first = &results[0];
        assert!(first.id > 0);
        assert!(!first.title.is_empty());
    }
}

#[test]
fn test_craft_delectations_catalog_and_essays() {
    let engine = WriterEngine::new();
    let essays = engine.list_craft_essays();
    assert_eq!(essays.len(), 5, "Must have 5 Craft Delectations essays");

    let slugs = ["dialectic", "canon", "frontier", "tropes", "publishing"];
    for slug in slugs {
        let essay = engine.get_craft_essay(slug).unwrap_or_else(|| panic!("Essay '{}' must load", slug));
        assert!(!essay.content_markdown.is_empty(), "Essay '{}' content must not be empty", slug);
        assert!(essay.word_count > 400, "Essay '{}' word count ({}) must exceed 400", slug, essay.word_count);
        assert!(essay.reading_time_mins >= 5, "Essay '{}' reading time must be at least 5 mins", slug);
    }

    // Specific thematic verifications
    let dialectic = engine.get_craft_essay("dialectic").unwrap();
    assert!(dialectic.content_markdown.contains("loadbearing"));
    assert!(dialectic.content_markdown.contains("Anti-Pattern"));

    let canon = engine.get_craft_essay("canon").unwrap();
    assert!(canon.content_markdown.contains("Dostoevsky"));
    assert!(canon.content_markdown.contains("Dickens"));
    assert!(canon.content_markdown.contains("Brontë"));
    assert!(canon.content_markdown.contains("Dzungar"));

    let frontier = engine.get_craft_essay("frontier").unwrap();
    assert!(frontier.content_markdown.contains("Iain M. Banks"));
    assert!(frontier.content_markdown.contains("Wintermute"));
    assert!(frontier.content_markdown.contains("Stephenson"));
    assert!(frontier.content_markdown.contains("Recursive Self-Improvement"));

    // Test alias "scifi"
    let scifi_alias = engine.get_craft_essay("scifi").expect("Alias 'scifi' should resolve to frontier essay");
    assert_eq!(scifi_alias.slug, "frontier");

    let tropes = engine.get_craft_essay("tropes").unwrap();
    assert!(tropes.content_markdown.contains("Frankenstein Smorgasbord"));
    assert!(tropes.content_markdown.contains("Only One Bed"));

    let publishing = engine.get_craft_essay("publishing").unwrap();
    assert!(publishing.content_markdown.contains("Coral Hart"));
    assert!(publishing.content_markdown.contains("KDP Select"));
}

#[test]
fn test_frankenstein_beat_sheet() {
    let engine = WriterEngine::new();
    let sheet = engine.generate_frankenstein_beats("Protocol Omega", "Dr. Vance");
    assert_eq!(sheet.genre, "frankenstein");
    assert_eq!(sheet.beats.len(), 7, "Frankenstein beat sheet must contain 7 satirical hybrid beats");

    let titles: Vec<_> = sheet.beats.iter().map(|b| b.title.as_str()).collect();
    assert!(titles.iter().any(|t| t.contains("Train Station")));
    assert!(titles.iter().any(|t| t.contains("Alien Nanites")));
    assert!(titles.iter().any(|t| t.contains("Only One Bed")));
    assert!(titles.iter().any(|t| t.contains("Damp Kitchen Sink")));
    assert!(titles.iter().any(|t| t.contains("Dental Chair")));
    assert!(titles.iter().any(|t| t.contains("00:01")));
    assert!(titles.iter().any(|t| t.contains("Scottish Moor")));
}

#[test]
fn test_epistemic_knowledge_matrix() {
    let engine = WriterEngine::new();
    let mut graph = engine.mock_epistemic_graph();

    // 1. Initial valid mock graph has no action anomalies
    let anomalies = engine.validate_epistemic_graph(&graph);
    let leaks: Vec<_> = anomalies.iter().filter(|a| a.issue_type == "PrematureKnowledgeLeak").collect();
    assert!(leaks.is_empty(), "Valid mock graph must have zero premature knowledge leaks");

    // 2. Timeline Flashback detection
    let timeline = graph.build_timeline();
    assert_eq!(timeline.len(), 3);
    assert_eq!(timeline[0].scene_id, "SCENE-01");
    assert_eq!(timeline[1].scene_id, "SCENE-02");
    assert_eq!(timeline[2].scene_id, "SCENE-03");
    assert!(timeline[2].is_flashback, "SCENE-03 (fabula 80 after scene 104) must be tagged as a flashback");

    // 3. Deliberately inject a plot-hole: Elena Vance acts in Scene 4 on a secret she does not know
    graph.add_scene_event(writercraft_core::epistemic_matrix::SceneEpistemicEvent {
        scene_id: "SCENE-04".into(),
        syuzhet_order: 4,
        fabula_time: 110,
        pov_character: "Dr. Elena Vance".into(),
        present_characters: vec!["Dr. Elena Vance".into()],
        revealed_facts: vec![],
        planted_lies: vec![],
        action_dependencies: vec!["FACT-03-NANITES".into()], // Elena was never told about nanites!
    });

    let new_anomalies = engine.validate_epistemic_graph(&graph);
    let leak_found = new_anomalies.iter().any(|a| {
        a.character == "Dr. Elena Vance" && a.fact_id == "FACT-03-NANITES" && a.issue_type == "PrematureKnowledgeLeak"
    });
    assert!(leak_found, "Must detect PrematureKnowledgeLeak for Dr. Elena Vance in SCENE-04");
}

#[test]
fn test_character_idiolect_analysis() {
    let engine = WriterEngine::new();

    let turns = vec![
        ("DR. VANCE".to_string(), "The mathematical resonance parameters demonstrate an irreversible bifurcation in the neural baseline. We must proceed with analytical rigor.".to_string()),
        ("DR. VANCE".to_string(), "Observe the harmonic oscillation across the telemetry registers. There is no plausible alternative hypothesis.".to_string()),
        ("MARCUS".to_string(), "Don't do it! I can't let you trigger this. Won't you see that they're coming for us? It's over!".to_string()),
        ("MARCUS".to_string(), "I'm telling you, we're done for. Pull the plug now!".to_string()),
    ];

    let report = engine.audit_dialogue_turns(&turns);
    assert_eq!(report.character_profiles.len(), 2);

    let vance = report.character_profiles.iter().find(|p| p.character == "DR. VANCE").unwrap();
    let marcus = report.character_profiles.iter().find(|p| p.character == "MARCUS").unwrap();

    // Vance has 0% contractions (formal academic)
    assert_eq!(vance.contraction_ratio, 0.0);
    // Marcus uses contractions ("don't", "can't", "won't", "it's", "i'm", "we're")
    assert!(marcus.contraction_ratio > 0.15);

    // High voice divergence between academic formal and colloquial urgent
    assert!(report.overall_voice_divergence > 0.4);

    // Test head-hopping detection in third-person narrative
    let prose_text = r#"Elena adjusted the frequency dial, watching the oscilloscope spike.
Across the console, Marcus Thorne smiled calmly, but secretly wanted to sabotage the trunk line.
He believed in his heart that the project would destroy everything."#;

    let leaks = engine.scan_pov_head_hopping(prose_text, "Elena", &["Elena", "Marcus Thorne"]);
    assert_eq!(leaks.len(), 2, "Must detect two head-hopping leaks for Marcus Thorne ('secretly wanted', 'believed in his heart')");
    assert_eq!(leaks[0].intruding_character, "Marcus Thorne");
    assert_eq!(leaks[0].cognitive_verb, "secretly wanted");
}

#[test]
fn test_fleet_harmonizer() {
    let engine = WriterEngine::new();
    let repos_dir = if Path::new("on-metacraft").exists() {
        Path::new(".")
    } else {
        Path::new("..")
    };

    let report = engine.harmonize_fleet_corpus(repos_dir);
    assert_eq!(report.autonomy_ladder.len(), 6, "Must define all 6 autonomy ladder rungs (Level 0 to 5)");
    assert_eq!(report.autonomy_ladder[0].level, 0);
    assert_eq!(report.autonomy_ladder[5].level, 5);

    assert!(report.terminology_consistency_score >= 0.85);
    assert!(report.total_books >= 1, "Must detect at least 1 fleet book when run from workspace");
}
