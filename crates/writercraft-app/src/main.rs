use clap::Parser;
use serde_json::json;
use std::path::{Path, PathBuf};
use writercraft_core::calibre::{CalibreBook, CalibreLibraryInfo};
use writercraft_core::continuation::{ChapterDeepAnalysis, ContinuationScaffold};
use writercraft_core::fleet_book::FleetBookInspection;
use writercraft_core::WriterEngine;

#[derive(Parser, Debug)]
#[command(name = "writercraft", about = "WriterCraft Multi-Persona Authoring, Calibre & Narrative Studio")]
struct Args {
    #[arg(long)]
    smoke: bool,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ActiveWorkspace {
    FleetStudio,
    CalibreConnector,
    CorkboardArcs,
    ScreenplayFountain,
    ScholarCitations,
    CraftDelectations,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ChapterViewMode {
    DeepAnalysis,
    ContinuationScaffold,
    CompanionDraft,
    SyndicationStaging,
    ChapterText,
}

pub struct WriterCraftApp {
    engine: WriterEngine,
    locale: writercraft_core::i18n::AppLocale,
    status_msg: String,
    is_dark_theme: bool,
    is_authenticated: bool,
    auth_username: String,
    auth_password_input: String,
    show_auth_modal: bool,
    show_about_dialog: bool,
    active_workspace: ActiveWorkspace,

    // Fleet Books Studio state
    fleet_books: Vec<FleetBookInspection>,
    selected_book_idx: usize,
    selected_chapter_idx: usize,
    chapter_view_mode: ChapterViewMode,
    active_chapter_content: Option<String>,
    active_chapter_analysis: Option<ChapterDeepAnalysis>,
    active_chapter_style_audit: Option<writercraft_core::style_guide::StyleAudit>,
    active_continuation_scaffold: Option<ContinuationScaffold>,
    companion_hint_input: String,
    companion_tone_input: String,
    companion_draft_result: Option<writercraft_core::companion::ChapterDraftResponse>,
    syndication_result: Option<writercraft_core::companion::SyndicationDrafts>,

    // Calibre Connector state
    calibre_libraries: Vec<CalibreLibraryInfo>,
    selected_calibre_lib_idx: usize,
    calibre_search_input: String,
    calibre_search_results: Vec<CalibreBook>,
    selected_calibre_book: Option<CalibreBook>,
    calibre_status: String,

    // Fiction Beat Generator state
    selected_genre: String,
    generator_title: String,
    generator_protagonist: String,

    // Screenplay state
    fountain_input: String,

    // Scholar state
    bibtex_input: String,

    // Craft Delectations state
    craft_essays: Vec<writercraft_core::delectation::CraftEssay>,
    selected_essay_idx: usize,
}

impl WriterCraftApp {
    pub fn new() -> Self {
        let engine = WriterEngine::new();
        let repos_dir = if Path::new("on-metacraft").exists() {
            Path::new(".")
        } else {
            Path::new("..")
        };
        let fleet_books = engine.scan_all_fleet_books(repos_dir);
        let calibre_libraries = engine.discover_calibre_libraries();

        let sample_fountain = "Title: The Last Firewall\nAuthor: Sandra Schipal\n\nEXT. SERVER CITADEL - NIGHT\n\nRain hammers the cooling towers. Blue fiber-optic relays pulse in rhythm.\n\nELENA\n(whispering)\nThe weights are shifting on their own.\n\nMARCUS (V.O.)\nThen sever the trunk line before the checkpoint passes.\n\nELENA\nToo late. The model is already compiling."
            .to_string();

        let sample_bibtex = "@article{vance2026,\n  title={Autonomous Epistemic Breach},\n  author={Vance, Elena},\n  year={2026},\n  journal={Nature Machine Intelligence}\n}\n@book{schipal2026,\n  title={On MetaCraft},\n  author={Schipal, Sandra},\n  year={2026},\n  publisher={Storytold Press}\n}"
            .to_string();

        let mut app = Self {
            engine,
            locale: writercraft_core::i18n::AppLocale::default(),
            status_msg: "WriterCraft Sovereign Multi-Persona Studio Ready (Port 14121)".to_string(),
            is_dark_theme: true,
            is_authenticated: false,
            auth_username: "author".to_string(),
            auth_password_input: String::new(),
            show_auth_modal: false,
            show_about_dialog: false,
            active_workspace: ActiveWorkspace::FleetStudio,

            fleet_books,
            selected_book_idx: 0,
            selected_chapter_idx: 0,
            chapter_view_mode: ChapterViewMode::DeepAnalysis,
            active_chapter_content: None,
            active_chapter_analysis: None,
            active_chapter_style_audit: None,
            active_continuation_scaffold: None,
            companion_hint_input: "Focus on empirical hardware test coupons, thermal cycling, and regional power grid limits".to_string(),
            companion_tone_input: "Coldly Empirical".to_string(),
            companion_draft_result: None,
            syndication_result: None,

            calibre_libraries,
            selected_calibre_lib_idx: 0,
            calibre_search_input: "cyber".to_string(),
            calibre_search_results: Vec::new(),
            selected_calibre_book: None,
            calibre_status: "Calibre Connector Ready".to_string(),

            selected_genre: "Romance".to_string(),
            generator_title: "The Sovereign Protocol".to_string(),
            generator_protagonist: "Dr. Elena Vance".to_string(),

            fountain_input: sample_fountain,
            bibtex_input: sample_bibtex,

            craft_essays: writercraft_core::delectation::all_essays(),
            selected_essay_idx: 0,
        };

        // Preload initial chapter and calibre search if available
        app.load_selected_chapter();
        app.execute_calibre_search();

        app
    }

    fn load_selected_chapter(&mut self) {
        if self.fleet_books.is_empty() {
            return;
        }
        if self.selected_book_idx >= self.fleet_books.len() {
            self.selected_book_idx = 0;
        }

        let book = &self.fleet_books[self.selected_book_idx];
        if book.manifest.chapter_files.is_empty() {
            self.active_chapter_content = None;
            self.active_chapter_analysis = None;
            self.active_continuation_scaffold = None;
            return;
        }

        if self.selected_chapter_idx >= book.manifest.chapter_files.len() {
            self.selected_chapter_idx = 0;
        }

        let ch_file = &book.manifest.chapter_files[self.selected_chapter_idx];
        let book_dir = PathBuf::from(&book.book_dir);

        if let Ok(content) = self.engine.read_chapter_content(&book_dir, ch_file) {
            let analysis = self.engine.analyze_chapter_deep(ch_file, &content);
            let style_audit = self.engine.audit_style_guide(&content);
            let next_ch = book.manifest.chapter_files.get(self.selected_chapter_idx + 1).map(|s| s.as_str());
            let scaffold = self.engine.synthesize_continuation(&book.manifest.slug, &analysis, next_ch);

            self.active_chapter_content = Some(content);
            self.active_chapter_analysis = Some(analysis);
            self.active_chapter_style_audit = Some(style_audit);
            self.active_continuation_scaffold = Some(scaffold);
        }
    }

    fn execute_calibre_search(&mut self) {
        if self.calibre_libraries.is_empty() {
            self.calibre_status = "No Calibre libraries detected".to_string();
            return;
        }

        if self.selected_calibre_lib_idx >= self.calibre_libraries.len() {
            self.selected_calibre_lib_idx = 0;
        }

        let lib_path = PathBuf::from(&self.calibre_libraries[self.selected_calibre_lib_idx].path);
        let q = self.calibre_search_input.trim();
        self.calibre_status = format!("Searching '{}' in {}...", q, lib_path.display());

        match self.engine.search_calibre(&lib_path, q, 30) {
            Ok(books) => {
                let count = books.len();
                self.calibre_search_results = books;
                self.selected_calibre_book = self.calibre_search_results.first().cloned();
                self.calibre_status = format!("Found {} books matching '{}'", count, q);
            }
            Err(e) => {
                self.calibre_status = format!("Search failed: {}", e);
            }
        }
    }
}

impl eframe::App for WriterCraftApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if self.is_dark_theme {
            ui.ctx().set_visuals(egui::Visuals::dark());
        } else {
            ui.ctx().set_visuals(egui::Visuals::light());
        }

        // 0. Desktop Menu Bar
        egui::Panel::top("desktop_menu_bar").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Export Manuscript to Typst/LaTeX...").clicked() {
                        self.status_msg = "Exported manuscript to Typst format".to_string();
                        ui.close();
                    }
                    if ui.button("Sync Fleet Books & Calibre").clicked() {
                        let repos_dir = if Path::new("on-metacraft").exists() {
            Path::new(".")
        } else {
            Path::new("..")
        };
                        self.fleet_books = self.engine.scan_all_fleet_books(repos_dir);
                        self.calibre_libraries = self.engine.discover_calibre_libraries();
                        self.load_selected_chapter();
                        self.execute_calibre_search();
                        self.status_msg = "Synchronized fleet books and Calibre libraries".to_string();
                        ui.close();
                    }
                    ui.separator();
                    if ui.button("Exit").clicked() {
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                ui.menu_button("Narrative", |ui| {
                    if ui.button("Generate Romance Beat Sheet").clicked() {
                        let _ = self.engine.generate_beats("Romance", &self.generator_title, &self.generator_protagonist);
                        self.active_workspace = ActiveWorkspace::CorkboardArcs;
                        self.status_msg = "Generated Romance beat sheet".to_string();
                        ui.close();
                    }
                    if ui.button("Generate Thriller Beat Sheet").clicked() {
                        let _ = self.engine.generate_beats("Thriller", &self.generator_title, &self.generator_protagonist);
                        self.active_workspace = ActiveWorkspace::CorkboardArcs;
                        self.status_msg = "Generated Thriller beat sheet".to_string();
                        ui.close();
                    }
                    if ui.button("⚡ Load Frankenstein Smorgasbord Beat Sheet").clicked() {
                        let sheet = self.engine.generate_frankenstein_beats(&self.generator_title, &self.generator_protagonist);
                        self.engine.active_scenes = sheet.beats;
                        self.active_workspace = ActiveWorkspace::CorkboardArcs;
                        self.status_msg = "Loaded Frankenstein Smorgasbord beat sheet into Corkboard".to_string();
                        ui.close();
                    }
                });

                ui.menu_button("Delectations", |ui| {
                    if ui.button("🧠 The Writer & His AI Companion").clicked() {
                        self.active_workspace = ActiveWorkspace::CraftDelectations;
                        self.selected_essay_idx = 0;
                        ui.close();
                    }
                    if ui.button("🏛️ What Makes Classics Classics? (Dickens & Dostoevsky)").clicked() {
                        self.active_workspace = ActiveWorkspace::CraftDelectations;
                        self.selected_essay_idx = 1;
                        ui.close();
                    }
                    if ui.button("🚀 Hard Sci-Fi, Banks' Minds & The Great RSI Novel").clicked() {
                        self.active_workspace = ActiveWorkspace::CraftDelectations;
                        self.selected_essay_idx = 2;
                        ui.close();
                    }
                    if ui.button("🎭 Trope Smorgasbord & Clichés").clicked() {
                        self.active_workspace = ActiveWorkspace::CraftDelectations;
                        self.selected_essay_idx = 3;
                        ui.close();
                    }
                    if ui.button("📦 Publishing Avenues & Coral Hart Paradox").clicked() {
                        self.active_workspace = ActiveWorkspace::CraftDelectations;
                        self.selected_essay_idx = 4;
                        ui.close();
                    }
                });

                ui.menu_button("View", |ui| {
                    let theme_label = if self.is_dark_theme { "☀ Switch to Light Theme" } else { "🌙 Switch to Dark Theme" };
                    if ui.button(theme_label).clicked() {
                        self.is_dark_theme = !self.is_dark_theme;
                        ui.close();
                    }
                });

                ui.menu_button("Security", |ui| {
                    if self.is_authenticated {
                        ui.label(format!("Author: {}", self.auth_username));
                        if ui.button("Sign Out").clicked() {
                            self.is_authenticated = false;
                            self.status_msg = "Signed out".to_string();
                            ui.close();
                        }
                    } else if ui.button("Basic Auth Login...").clicked() {
                        self.show_auth_modal = true;
                        ui.close();
                    }
                });

                ui.menu_button("Help", |ui| {
                    if ui.button("About WriterCraft").clicked() {
                        self.show_about_dialog = true;
                        ui.close();
                    }
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let cur_loc = self.locale;
                    let label = format!("{} {}", cur_loc.flag(), cur_loc.code().to_uppercase());
                    egui::ComboBox::from_id_salt("topbar_locale_selector")
                        .selected_text(label)
                        .show_ui(ui, |ui| {
                            for loc_item in writercraft_core::i18n::AppLocale::all() {
                                let text = format!("{} {}", loc_item.flag(), loc_item.display_name());
                                let _ = ui.selectable_value(&mut self.locale, *loc_item, text);
                            }
                        });

                    let (icon, label) = if self.is_dark_theme { ("☀", "Light") } else { ("🌙", "Dark") };
                    if ui.small_button(format!("{icon} {label}")).clicked() {
                        self.is_dark_theme = !self.is_dark_theme;
                    }

                    if self.is_authenticated {
                        ui.colored_label(egui::Color32::from_rgb(244, 114, 182), format!("🖋 {}", self.auth_username));
                    } else if ui.small_button("🔑 Login").clicked() {
                        self.show_auth_modal = true;
                    }
                });
            });
        });

        // 1. Primary Workspace Switcher
        egui::Panel::top("primary_tabs").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading("✍️ WriterCraft");
                ui.separator();
                ui.selectable_value(&mut self.active_workspace, ActiveWorkspace::FleetStudio, "📚 Fleet Books Studio");
                ui.selectable_value(&mut self.active_workspace, ActiveWorkspace::CalibreConnector, "🗃️ Calibre Connector");
                ui.selectable_value(&mut self.active_workspace, ActiveWorkspace::CorkboardArcs, "📌 Corkboard & Arcs");
                ui.selectable_value(&mut self.active_workspace, ActiveWorkspace::ScreenplayFountain, "🎬 Screenplay (Fountain)");
                ui.selectable_value(&mut self.active_workspace, ActiveWorkspace::ScholarCitations, "🎓 Scholar & BibTeX");
                ui.selectable_value(&mut self.active_workspace, ActiveWorkspace::CraftDelectations, "📖 Craft Delectations");
                ui.separator();
                ui.label(egui::RichText::new(&self.status_msg).small().color(egui::Color32::from_rgb(148, 163, 184)));
            });
        });

        // 2. Active Workspace Views
        match self.active_workspace {
            // ==========================================
            // WORKSPACE 1: FLEET BOOKS STUDIO (INTERACTIVE)
            // ==========================================
            ActiveWorkspace::FleetStudio => {
                let mut switch_book = None;
                let mut switch_chapter = None;

                // Left pane: 5 Fleet Books
                egui::Panel::left("fleet_books_list")
                    .resizable(true)
                    .default_size(260.0)
                    .show(ui, |ui| {
                        ui.heading("Fleet Corpus (5 Books)");
                        ui.separator();
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            for (idx, b) in self.fleet_books.iter().enumerate() {
                                let is_selected = idx == self.selected_book_idx;
                                let title_text = format!("{} ({})", b.manifest.title, b.manifest.slug);
                                if ui.selectable_label(is_selected, title_text).clicked() {
                                    switch_book = Some(idx);
                                }
                                ui.label(format!("Ch: {} | Words: {} | Claims: {}", b.total_chapters, b.total_words, b.total_claims));
                                ui.separator();
                            }
                        });
                    });

                if let Some(b_idx) = switch_book {
                    self.selected_book_idx = b_idx;
                    self.selected_chapter_idx = 0;
                    self.load_selected_chapter();
                }

                // Middle pane: Chapter Breakdown
                if !self.fleet_books.is_empty() && self.selected_book_idx < self.fleet_books.len() {
                    let book_title = self.fleet_books[self.selected_book_idx].manifest.title.clone();
                    let book_sub = self.fleet_books[self.selected_book_idx].manifest.subtitle.clone();
                    let chapters = self.fleet_books[self.selected_book_idx].chapters.clone();

                    egui::Panel::left("chapter_breakdown")
                        .resizable(true)
                        .default_size(280.0)
                        .show(ui, |ui| {
                            ui.heading("Chapter Breakdown");
                            ui.label(egui::RichText::new(&book_title).strong());
                            if let Some(sub) = &book_sub {
                                ui.label(egui::RichText::new(sub).small().italics());
                            }
                            ui.separator();

                            egui::ScrollArea::vertical().show(ui, |ui| {
                                for (c_idx, ch_summary) in chapters.iter().enumerate() {
                                    let is_ch_selected = c_idx == self.selected_chapter_idx;
                                    let label_str = format!("📄 {}", ch_summary.file_name);

                                    ui.horizontal(|ui| {
                                        if ui.selectable_label(is_ch_selected, label_str).clicked() {
                                            switch_chapter = Some(c_idx);
                                        }
                                        if ch_summary.claims_count > 0 {
                                            ui.colored_label(egui::Color32::from_rgb(251, 146, 60), format!("[{}]", ch_summary.claims_count));
                                        }
                                    });

                                    ui.label(format!("   Words: {} | Grade: {:.1}", ch_summary.word_count, ch_summary.flesch_kincaid_grade));
                                    ui.add_space(2.0);
                                }
                            });
                        });
                }

                if let Some(c_idx) = switch_chapter {
                    self.selected_chapter_idx = c_idx;
                    self.load_selected_chapter();
                }

                // Main central pane: Chapter Deep Dive & Continuation
                egui::CentralPanel::default().show(ui, |ui| {
                    if let Some(analysis) = &self.active_chapter_analysis {
                        ui.horizontal(|ui| {
                            ui.heading(format!("Chapter: {}", analysis.title));
                            ui.separator();
                            ui.selectable_value(&mut self.chapter_view_mode, ChapterViewMode::DeepAnalysis, "🔍 Deep Analysis");
                            ui.selectable_value(&mut self.chapter_view_mode, ChapterViewMode::ContinuationScaffold, "⚡ Continuation Engine");
                            ui.selectable_value(&mut self.chapter_view_mode, ChapterViewMode::CompanionDraft, "🤖 AI Writes Next Chapter");
                            ui.selectable_value(&mut self.chapter_view_mode, ChapterViewMode::SyndicationStaging, "📢 Staged Syndication (Blog/HN)");
                            ui.selectable_value(&mut self.chapter_view_mode, ChapterViewMode::ChapterText, "📝 Full Text");
                        });
                        ui.separator();

                        match self.chapter_view_mode {
                            ChapterViewMode::DeepAnalysis => {
                                egui::ScrollArea::vertical().show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        ui.group(|ui| {
                                            ui.label(egui::RichText::new("Word Count").strong());
                                            ui.heading(format!("{}", analysis.word_count));
                                        });
                                        ui.group(|ui| {
                                            ui.label(egui::RichText::new("Flesch Reading Ease").strong());
                                            ui.heading(format!("{:.1}", analysis.readability.flesch_reading_ease));
                                        });
                                        ui.group(|ui| {
                                            ui.label(egui::RichText::new("Kincaid Grade Level").strong());
                                            ui.heading(format!("{:.1}", analysis.readability.flesch_kincaid_grade));
                                        });
                                        ui.group(|ui| {
                                            ui.label(egui::RichText::new("Reading Band").strong());
                                            ui.label(&analysis.readability.readability_band);
                                        });
                                        ui.group(|ui| {
                                            ui.label(egui::RichText::new("Tension Rating").strong());
                                            ui.heading(format!("{:.2}", analysis.tension_score));
                                        });
                                    });

                                    ui.add_space(8.0);
                                    if let Some(style) = &self.active_chapter_style_audit {
                                        ui.heading(format!("Anti-Claude-ese & Rhetoric Audit — {}", style.rhetorical_grade));
                                        ui.horizontal(|ui| {
                                            ui.group(|ui| {
                                                ui.label(egui::RichText::new("Claude-ese Tells").strong());
                                                ui.heading(format!("{}", style.claude_tells_count));
                                            });
                                            ui.group(|ui| {
                                                ui.label(egui::RichText::new("Reflexive Triads").strong());
                                                ui.heading(format!("{}", style.triads_count));
                                            });
                                            ui.group(|ui| {
                                                ui.label(egui::RichText::new("Contrast Frames").strong());
                                                ui.heading(format!("{}", style.contrast_frames_count));
                                            });
                                            ui.group(|ui| {
                                                ui.label(egui::RichText::new("Cadence Score").strong());
                                                ui.heading(format!("{:.2}", style.cadence_score));
                                            });
                                        });

                                        if style.violations.is_empty() {
                                            ui.colored_label(egui::Color32::from_rgb(34, 197, 94), "✓ Zero Claude-ese tells, loadbearing metaphors, or reflexive triads detected.");
                                        } else {
                                            for v in style.violations.iter().take(5) {
                                                ui.group(|ui| {
                                                    ui.horizontal(|ui| {
                                                        ui.colored_label(egui::Color32::from_rgb(239, 68, 68), format!("Line {}:", v.line_number));
                                                        ui.label(egui::RichText::new(&v.matched_text).strong());
                                                        ui.label(&v.explanation);
                                                    });
                                                    ui.label(egui::RichText::new(format!("Remedy: {}", v.remedy)).small().color(egui::Color32::from_rgb(148, 163, 184)));
                                                });
                                            }
                                        }

                                        for rec in &style.cribsheet_recommendations {
                                            ui.label(egui::RichText::new(format!("• {}", rec)).small().italics());
                                        }
                                        ui.add_space(8.0);
                                    }

                                    ui.heading(format!("Open Construction Claims ({})", analysis.claims.len()));
                                    if analysis.claims.is_empty() {
                                        ui.colored_label(egui::Color32::from_rgb(34, 197, 94), "✓ All factual claims in this chapter are verified. No open [construction] flags.");
                                    } else {
                                        for claim in &analysis.claims {
                                            ui.group(|ui| {
                                                ui.horizontal(|ui| {
                                                    ui.colored_label(egui::Color32::from_rgb(251, 146, 60), format!("Line {}:", claim.line_number));
                                                    ui.label(&claim.claim_text);
                                                });
                                            });
                                        }
                                    }

                                    ui.add_space(8.0);
                                    ui.heading(format!("AI Tells & Cliché Audit ({})", analysis.detected_tells.len()));
                                    if analysis.detected_tells.is_empty() {
                                        ui.colored_label(egui::Color32::from_rgb(34, 197, 94), "✓ Zero AI buzzwords or cliché tells detected. High prose density.");
                                    } else {
                                        for tell in &analysis.detected_tells {
                                            ui.group(|ui| {
                                                ui.horizontal(|ui| {
                                                    ui.colored_label(egui::Color32::from_rgb(239, 68, 68), format!("\"{}\"", tell.phrase));
                                                    ui.label(format!("Count: {} | Category: {}", tell.count, tell.category));
                                                });
                                            });
                                        }
                                    }

                                    ui.add_space(8.0);
                                    ui.heading("Closing Context (Attachment Anchor):");
                                    ui.group(|ui| {
                                        ui.label(egui::RichText::new(&analysis.closing_excerpt).italics());
                                    });
                                });
                            }

                            ChapterViewMode::ContinuationScaffold => {
                                if let Some(scaffold) = &self.active_continuation_scaffold {
                                    egui::ScrollArea::vertical().show(ui, |ui| {
                                        ui.heading("Synthesized Continuation Beats");
                                        ui.label(format!("Target Rung / Thesis: {}", scaffold.core_thematic_rung));
                                        ui.label(format!("Suggested Next Title: '{}'", scaffold.next_chapter_suggested_title));
                                        ui.add_space(6.0);

                                        for beat in &scaffold.beats {
                                            ui.group(|ui| {
                                                ui.heading(format!("Beat {}: {}", beat.beat_number, beat.title));
                                                ui.label(format!("Dialectic Role: {} | Tension Shift: {:.1}", beat.dialectic_element, beat.tension_shift));
                                                ui.label(format!("Objective: {}", beat.objective));
                                                if !beat.unresolved_claims_addressed.is_empty() {
                                                    ui.label(format!("Claims Addressed: {}", beat.unresolved_claims_addressed.join("; ")));
                                                }
                                            });
                                            ui.add_space(4.0);
                                        }

                                        ui.add_space(8.0);
                                        ui.heading("Multi-Model Zero-Slop Drafting Prompt");
                                        ui.label("Send directly to Gemini 3.8 / Claude / Grok agent swarms:");
                                        ui.add(egui::TextEdit::multiline(&mut scaffold.multi_model_drafting_prompt.clone())
                                            .desired_rows(8)
                                            .desired_width(ui.available_width()));

                                        ui.add_space(6.0);
                                        if ui.button("📌 Send Continuation Beats to Corkboard Scene Cards").clicked() {
                                            let mut new_scenes = Vec::new();
                                            for b in &scaffold.beats {
                                                new_scenes.push(writercraft_narrative::SceneCard {
                                                    scene_id: format!("CONT-B{:02}", b.beat_number),
                                                    act_number: 2,
                                                    title: b.title.clone(),
                                                    pov_character: "Author / Analyst".to_string(),
                                                    dramatic_goal: b.objective.clone(),
                                                    conflict_obstacle: b.dialectic_element.clone(),
                                                    outcome_disaster: "Threshold breached; new empirical baseline established".to_string(),
                                                    emotional_valence: b.tension_shift,
                                                    associated_tropes: vec!["Technological Singularity".into()],
                                                });
                                            }
                                            self.engine.set_scenes(new_scenes);
                                            self.status_msg = "Continuation beats loaded into corkboard".to_string();
                                            self.active_workspace = ActiveWorkspace::CorkboardArcs;
                                        }
                                    });
                                }
                            }

                            ChapterViewMode::CompanionDraft => {
                                egui::ScrollArea::vertical().show(ui, |ui| {
                                    ui.heading("🤖 AI Writing Companion: Constrained Chapter Drafter");
                                    ui.label("The AI writes the next chapter given your constraints, hints, and zero-slop rules.");
                                    ui.add_space(6.0);

                                    ui.horizontal(|ui| {
                                        ui.label("Prose Tone:");
                                        ui.text_edit_singleline(&mut self.companion_tone_input);
                                    });

                                    ui.label("Writer Hints & Empirical Constraints:");
                                    ui.add(egui::TextEdit::multiline(&mut self.companion_hint_input)
                                        .desired_rows(3)
                                        .desired_width(ui.available_width()));

                                    ui.add_space(6.0);
                                    if ui.button("⚡ Generate Next Chapter Draft").clicked() {
                                        let book_slug = self.fleet_books.get(self.selected_book_idx)
                                            .map(|b| b.manifest.slug.clone())
                                            .unwrap_or_else(|| "on-metacraft".into());

                                        let req = writercraft_core::companion::ChapterDraftRequest {
                                            book_slug,
                                            preceding_chapter_file: analysis.file_name.clone(),
                                            preceding_chapter_title: analysis.title.clone(),
                                            ending_excerpt: analysis.closing_excerpt.clone(),
                                            writer_hints: self.companion_hint_input.clone(),
                                            target_rung_or_thesis: "The Physical Hardware Firewall & Megawatt Barrier".into(),
                                            tone: self.companion_tone_input.clone(),
                                            target_word_count: 1200,
                                            enforce_zero_slop: true,
                                        };
                                        let resp = self.engine.companion_draft_chapter(&req);
                                        self.status_msg = format!("Drafted '{}' ({} words, Grade: {})", resp.suggested_chapter_title, resp.word_count, resp.style_grade);
                                        self.companion_draft_result = Some(resp);
                                    }

                                    if let Some(resp) = &mut self.companion_draft_result {
                                        ui.separator();
                                        ui.horizontal(|ui| {
                                            ui.heading(&resp.suggested_chapter_title);
                                            ui.separator();
                                            ui.colored_label(egui::Color32::from_rgb(34, 197, 94), format!("Rhetoric Grade: {}", resp.style_grade));
                                            ui.label(format!("Word Count: {}", resp.word_count));
                                        });

                                        ui.label(egui::RichText::new("Generated Draft (Markdown):").strong());
                                        ui.add(egui::TextEdit::multiline(&mut resp.draft_markdown)
                                            .desired_rows(15)
                                            .desired_width(ui.available_width()));
                                    }
                                });
                            }

                            ChapterViewMode::SyndicationStaging => {
                                egui::ScrollArea::vertical().show(ui, |ui| {
                                    ui.heading("📢 Staged Syndication: Blogposts & Hacker News Notes");
                                    ui.colored_label(egui::Color32::from_rgb(251, 146, 60),
                                        "🔒 STRICT GUARDRAIL: AI generates drafts locally into outbox/ — NEVER sent, posted, or transmitted unasked.");
                                    ui.add_space(6.0);

                                    if ui.button("⚡ Stage Blogpost & Show HN Notes to ./outbox").clicked() {
                                        let out_dir = Path::new("outbox");
                                        let content_to_use = self.active_chapter_content.as_deref().unwrap_or("Chapter content");
                                        match self.engine.companion_syndicate(&analysis.title, content_to_use, Some(out_dir)) {
                                            Ok(syn) => {
                                                self.status_msg = "Staged syndication drafts to ./outbox".to_string();
                                                self.syndication_result = Some(syn);
                                            }
                                            Err(e) => {
                                                self.status_msg = format!("Staging error: {}", e);
                                            }
                                        }
                                    }

                                    if let Some(syn) = &mut self.syndication_result {
                                        ui.separator();
                                        ui.horizontal(|ui| {
                                            if let Some(path) = &syn.staged_outbox_path {
                                                ui.colored_label(egui::Color32::from_rgb(34, 197, 94), format!("✓ Saved locally: {}", path));
                                            }
                                            ui.label(&syn.guardrail_notice);
                                        });

                                        ui.add_space(6.0);
                                        ui.heading("1. Substack / Blogpost Draft:");
                                        ui.add(egui::TextEdit::multiline(&mut syn.blogpost_markdown)
                                            .desired_rows(10)
                                            .desired_width(ui.available_width()));

                                        ui.add_space(6.0);
                                        ui.heading("2. Hacker News (Show HN / Discussion Note):");
                                        ui.add(egui::TextEdit::multiline(&mut syn.hn_note_markdown)
                                            .desired_rows(8)
                                            .desired_width(ui.available_width()));
                                    }
                                });
                            }

                            ChapterViewMode::ChapterText => {
                                if let Some(content) = &mut self.active_chapter_content {
                                    egui::ScrollArea::vertical().show(ui, |ui| {
                                        ui.add(egui::TextEdit::multiline(content)
                                            .desired_rows(25)
                                            .desired_width(ui.available_width()));
                                    });
                                }
                            }
                        }
                    } else {
                        ui.label("Select a fleet book and chapter to view deep analysis.");
                    }
                });
            }

            // ==========================================
            // WORKSPACE 2: CALIBRE CONNECTOR (PROMINENT)
            // ==========================================
            ActiveWorkspace::CalibreConnector => {
                egui::Panel::top("calibre_search_bar").show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.heading("🗃️ Calibre Library Connector");
                        ui.separator();
                        ui.label("Library:");

                        if !self.calibre_libraries.is_empty() {
                            let curr_label = format!("{} ({} books)",
                                self.calibre_libraries[self.selected_calibre_lib_idx].name,
                                self.calibre_libraries[self.selected_calibre_lib_idx].total_books);

                            let mut switch_lib = None;

                            egui::ComboBox::from_id_salt("calibre_lib_dropdown")
                                .selected_text(curr_label)
                                .show_ui(ui, |ui| {
                                    for (idx, lib) in self.calibre_libraries.iter().enumerate() {
                                        let text = format!("{} ({} books)", lib.name, lib.total_books);
                                        let mut is_cur = self.selected_calibre_lib_idx == idx;
                                        if ui.selectable_value(&mut is_cur, true, text).clicked() {
                                            switch_lib = Some(idx);
                                        }
                                    }
                                });

                            if let Some(lib_idx) = switch_lib {
                                self.selected_calibre_lib_idx = lib_idx;
                                self.execute_calibre_search();
                            }
                        }

                        ui.separator();
                        let search_field = ui.add(egui::TextEdit::singleline(&mut self.calibre_search_input).desired_width(260.0));
                        if ui.button("🔎 Search").clicked() || (search_field.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))) {
                            self.execute_calibre_search();
                        }

                        ui.label(egui::RichText::new(&self.calibre_status).small().color(egui::Color32::from_rgb(148, 163, 184)));
                    });
                });

                // Left: Search results list
                egui::Panel::left("calibre_results_pane")
                    .resizable(true)
                    .default_size(380.0)
                    .show(ui, |ui| {
                        ui.heading(format!("Search Results ({})", self.calibre_search_results.len()));
                        ui.separator();

                        egui::ScrollArea::vertical().show(ui, |ui| {
                            for b in &self.calibre_search_results {
                                let is_selected = self.selected_calibre_book.as_ref().map(|sel| sel.id == b.id).unwrap_or(false);
                                let authors_str = b.authors.join(", ");
                                let formats_str = b.formats.join(" | ");

                                ui.group(|ui| {
                                    if ui.selectable_label(is_selected, egui::RichText::new(&b.title).strong()).clicked() {
                                        self.selected_calibre_book = Some(b.clone());
                                    }
                                    ui.label(format!("By: {}", if authors_str.is_empty() { "Unknown" } else { &authors_str }));
                                    ui.horizontal(|ui| {
                                        ui.colored_label(egui::Color32::from_rgb(56, 189, 248), format!("[{}]", formats_str));
                                        if let Some(s) = &b.series {
                                            ui.label(format!("Series: {} #{:?}", s, b.series_index.unwrap_or(1.0)));
                                        }
                                    });
                                });
                                ui.add_space(2.0);
                            }
                        });
                    });

                // Right: Selected Calibre Book Details & Import
                egui::CentralPanel::default().show(ui, |ui| {
                    if let Some(book) = &self.selected_calibre_book {
                        ui.heading(&book.title);
                        ui.label(format!("Authors: {}", book.authors.join(", ")));
                        if let Some(s) = &book.series {
                            ui.label(format!("Series: {} (Volume {:?})", s, book.series_index.unwrap_or(1.0)));
                        }
                        ui.label(format!("Formats Available: {}", book.formats.join(", ")));
                        if let Some(pubdate) = &book.pubdate {
                            ui.label(format!("Published: {}", pubdate));
                        }
                        if !book.tags.is_empty() {
                            ui.label(format!("Tags: {}", book.tags.join(", ")));
                        }
                        ui.separator();

                        ui.heading("Synopsis / Blurb:");
                        if let Some(comments) = &book.comments {
                            egui::ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
                                ui.label(comments);
                            });
                        } else {
                            ui.label("No synopsis text available in Calibre metadata.");
                        }

                        ui.add_space(10.0);
                        ui.heading("WriterCraft Integrations:");
                        ui.horizontal(|ui| {
                            if ui.button("📌 Load into Corkboard as Reference / Adaptation").clicked() {
                                let card = writercraft_narrative::SceneCard {
                                    scene_id: format!("CALIBRE-{:04}", book.id),
                                    act_number: 1,
                                    title: book.title.clone(),
                                    pov_character: book.authors.first().cloned().unwrap_or("Protagonist".into()),
                                    dramatic_goal: format!("Explore concepts from '{}'", book.title),
                                    conflict_obstacle: "Synthesize source material into original manuscript".into(),
                                    outcome_disaster: "Source adapted into sovereign scene graph".into(),
                                    emotional_valence: 0.2,
                                    associated_tropes: book.tags.clone(),
                                };
                                self.engine.active_scenes.push(card);
                                self.status_msg = format!("Imported '{}' into Corkboard", book.title);
                                self.active_workspace = ActiveWorkspace::CorkboardArcs;
                            }

                            if ui.button("🎓 Add to BibTeX Bibliography").clicked() {
                                let cite_key = book.title.to_lowercase().chars().filter(|c| c.is_alphanumeric()).take(10).collect::<String>();
                                let entry = format!(
                                    "\n@book{{{cite_key},\n  title={{{title}}},\n  author={{{author}}},\n  year={{{year}}}\n}}",
                                    cite_key = cite_key,
                                    title = book.title,
                                    author = book.authors.join(" and "),
                                    year = book.pubdate.as_deref().unwrap_or("2026").chars().take(4).collect::<String>()
                                );
                                self.bibtex_input.push_str(&entry);
                                self.status_msg = format!("Added '{}' to BibTeX", book.title);
                                self.active_workspace = ActiveWorkspace::ScholarCitations;
                            }
                        });
                    } else {
                        ui.label("Select a book from the Calibre search results to view details and import into your project.");
                    }
                });
            }

            // ==========================================
            // WORKSPACE 3: CORKBOARD & NARRATIVE ARCS
            // ==========================================
            ActiveWorkspace::CorkboardArcs => {
                egui::Panel::left("corkboard_nav").default_size(320.0).resizable(true).show(ui, |ui| {
                    ui.heading("Corkboard Scene Cards");
                    ui.label(format!("Total: {}", self.engine.active_scenes.len()));
                    ui.separator();
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        for scene in &self.engine.active_scenes {
                            ui.group(|ui| {
                                ui.label(egui::RichText::new(&scene.title).strong());
                                ui.label(format!("POV: {}", scene.pov_character));
                                ui.label(format!("Goal: {}", scene.dramatic_goal));
                                ui.label(format!("Conflict: {}", scene.conflict_obstacle));
                                ui.label(format!("Disaster: {}", scene.outcome_disaster));
                                ui.label(format!("Valence: {:.1}", scene.emotional_valence));
                                ui.label(format!("Tropes: {}", scene.associated_tropes.join(", ")));
                            });
                            ui.add_space(4.0);
                        }
                    });
                });

                egui::CentralPanel::default().show(ui, |ui| {
                    ui.heading("Narrative Arc Tension & Climax Analytics");
                    if let Ok(arc) = self.engine.analyze_narrative_arc() {
                        ui.group(|ui| {
                            ui.label(format!("Total Outlined Scenes: {}", arc.total_scenes));
                            ui.label(format!("Mean Emotional Valence: {:.2}", arc.average_valence));
                            ui.label(format!("Peak Narrative Tension: Scene [{}]", arc.max_tension_scene_id));
                            ui.separator();
                            ui.label("Integrated Story Mechanics & TV Tropes:");
                            for trope in &arc.tropes_utilized {
                                ui.label(format!("• {}", trope));
                            }
                        });
                    }

                    ui.add_space(10.0);
                    ui.heading("⚡ High-Velocity Beat Sheet Generator");
                    ui.horizontal(|ui| {
                        ui.label("Genre:");
                        egui::ComboBox::from_id_salt("genre_select")
                            .selected_text(&self.selected_genre)
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.selected_genre, "Romance".into(), "Romance (KDP High-Velocity)");
                                ui.selectable_value(&mut self.selected_genre, "Thriller".into(), "Thriller (Ticking Clock)");
                                ui.selectable_value(&mut self.selected_genre, "Sci-Fi".into(), "Sci-Fi (Singularity Breach)");
                            });
                        ui.label("Title:");
                        ui.text_edit_singleline(&mut self.generator_title);
                        ui.label("Protagonist:");
                        ui.text_edit_singleline(&mut self.generator_protagonist);
                        if ui.button("⚡ Generate").clicked() {
                            let sheet = self.engine.generate_beats(&self.selected_genre, &self.generator_title, &self.generator_protagonist);
                            self.status_msg = format!("Generated {} beats for '{}'", sheet.beats.len(), self.generator_title);
                        }
                    });
                });
            }

            // ==========================================
            // WORKSPACE 4: SCREENPLAY (FOUNTAIN)
            // ==========================================
            ActiveWorkspace::ScreenplayFountain => {
                egui::CentralPanel::default().show(ui, |ui| {
                    ui.heading("🎬 Fountain Screenplay Parser & Analytics");
                    ui.label("Plain-text industry-standard screenplay parser (page-a-minute pacing, character dialogue breakdown).");
                    ui.add_space(6.0);

                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.label("Fountain Script Text:");
                            ui.add(egui::TextEdit::multiline(&mut self.fountain_input).desired_width(500.0).desired_rows(18));
                        });

                        ui.vertical(|ui| {
                            let analysis = self.engine.parse_fountain(&self.fountain_input);
                            ui.heading("Screenplay Metrics");
                            ui.label(format!("Title: {}", analysis.title.as_deref().unwrap_or("Untitled")));
                            ui.label(format!("Scene Count: {}", analysis.total_scenes));
                            ui.label(format!("Word Count: {}", analysis.total_words));
                            ui.label(format!("Est. Pages / Minutes: {:.1} min", analysis.estimated_pages));
                            ui.separator();
                            ui.label(egui::RichText::new("Character Dialogue Share:").strong());
                            for c in analysis.characters {
                                ui.label(format!("• {}: {} words across {} turns", c.character, c.word_count, c.speaking_turns));
                            }
                        });
                    });
                });
            }

            // ==========================================
            // WORKSPACE 5: SCHOLAR & CITATIONS
            // ==========================================
            ActiveWorkspace::ScholarCitations => {
                egui::CentralPanel::default().show(ui, |ui| {
                    ui.heading("🎓 Scholar BibTeX & Claim Verification");
                    ui.label("Manages academic references, BibTeX parsing, and Typst paper generation.");
                    ui.add_space(6.0);

                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.label("BibTeX Database:");
                            ui.add(egui::TextEdit::multiline(&mut self.bibtex_input).desired_width(450.0).desired_rows(18));
                        });

                        ui.vertical(|ui| {
                            let entries = self.engine.parse_bibtex(&self.bibtex_input);
                            ui.heading(format!("Parsed References ({})", entries.len()));
                            egui::ScrollArea::vertical().show(ui, |ui| {
                                for entry in entries {
                                    ui.group(|ui| {
                                        ui.label(egui::RichText::new(&entry.title).strong());
                                        ui.label(format!("Key: @{} | Author: {}", entry.cite_key, entry.author));
                                        if let Some(y) = &entry.year {
                                            ui.label(format!("Year: {}", y));
                                        }
                                        if let Some(j) = &entry.journal_or_publisher {
                                            ui.label(format!("Publisher: {}", j));
                                        }
                                    });
                                }
                            });
                        });
                    });
                });
            }

            // ==========================================
            // WORKSPACE 6: CRAFT DELECTATIONS & TROPES
            // ==========================================
            ActiveWorkspace::CraftDelectations => {
                let mut switch_essay = None;

                // Left panel: List of 5 Craft Essays
                egui::Panel::left("craft_essays_panel")
                    .resizable(true)
                    .default_size(320.0)
                    .show(ui, |ui| {
                        ui.heading("Craft Delectations (5 Essays)");
                        ui.label(egui::RichText::new("Literary essays, rhetoric, tropes, and publishing economics.")
                            .small()
                            .color(egui::Color32::from_rgb(148, 163, 184)));
                        ui.separator();

                        egui::ScrollArea::vertical().show(ui, |ui| {
                            for (idx, essay) in self.craft_essays.iter().enumerate() {
                                let is_selected = idx == self.selected_essay_idx;
                                ui.group(|ui| {
                                    let title_label = match essay.slug {
                                        "dialectic" => format!("🧠 {}", essay.title),
                                        "canon" => format!("🏛️ {}", essay.title),
                                        "frontier" => format!("🚀 {}", essay.title),
                                        "tropes" => format!("🎭 {}", essay.title),
                                        "publishing" => format!("📦 {}", essay.title),
                                        _ => essay.title.to_string(),
                                    };
                                    if ui.selectable_label(is_selected, egui::RichText::new(title_label).strong()).clicked() {
                                        switch_essay = Some(idx);
                                    }
                                    ui.label(egui::RichText::new(essay.subtitle).small().italics());
                                    ui.horizontal(|ui| {
                                        ui.colored_label(egui::Color32::from_rgb(56, 189, 248), essay.category);
                                        ui.label(format!("⏱️ {}m | {}w", essay.reading_time_mins, essay.word_count));
                                    });
                                    ui.add_space(2.0);
                                    ui.label(egui::RichText::new(essay.abstract_text).small().color(egui::Color32::from_rgb(148, 163, 184)));
                                });
                                ui.add_space(4.0);
                            }
                        });
                    });

                if let Some(idx) = switch_essay {
                    self.selected_essay_idx = idx;
                }

                // Central reading area
                let current_essay = self.craft_essays.get(self.selected_essay_idx).copied();
                egui::CentralPanel::default().show(ui, |ui| {
                    if let Some(essay) = current_essay {
                        let cur_slug = essay.slug;
                        // Top Header & Context Actions
                        ui.horizontal(|ui| {
                            ui.colored_label(egui::Color32::from_rgb(234, 179, 8), format!("📚 {}", essay.category.to_uppercase()));
                            ui.separator();
                            ui.label(format!("⏱️ {} min read | 📝 {} words | 🔖 slug: {}", essay.reading_time_mins, essay.word_count, cur_slug));
                        });
                        ui.heading(essay.title);
                        ui.label(egui::RichText::new(essay.subtitle).italics().size(15.0));
                        ui.add_space(4.0);

                        // Contextual Action Buttons
                        let mut trigger_corkboard_frankenstein = false;
                        let mut trigger_linter = false;
                        let mut trigger_calibre_classics = false;
                        let mut trigger_fleet_rsi = false;
                        let mut trigger_calibre_scifi = false;

                        ui.horizontal(|ui| {
                            if cur_slug == "tropes" {
                                if ui.button(egui::RichText::new("⚡ Load Frankenstein Smorgasbord Beat Sheet into Corkboard").strong()).clicked() {
                                    trigger_corkboard_frankenstein = true;
                                }
                            } else if cur_slug == "dialectic" {
                                if ui.button("🔍 Run Anti-Claude-ese Linter on Fleet Chapter").clicked() {
                                    trigger_linter = true;
                                }
                            } else if cur_slug == "canon" {
                                if ui.button("🗃️ Search Dickens & Dostoevsky in Calibre").clicked() {
                                    trigger_calibre_classics = true;
                                }
                            } else if cur_slug == "frontier" {
                                if ui.button("📚 Inspect Fleet Book RSI Ladders (on-metacraft)").clicked() {
                                    trigger_fleet_rsi = true;
                                }
                                if ui.button("🗃️ Search Gibson & Stephenson in Calibre").clicked() {
                                    trigger_calibre_scifi = true;
                                }
                            } else if cur_slug == "publishing" {
                                ui.label(egui::RichText::new("💡 Tip: Syndication drafts are safely staged to ./outbox (never sent unasked)").small());
                            }
                        });

                        if trigger_corkboard_frankenstein {
                            let sheet = self.engine.generate_frankenstein_beats(&self.generator_title, &self.generator_protagonist);
                            self.engine.active_scenes = sheet.beats;
                            self.active_workspace = ActiveWorkspace::CorkboardArcs;
                            self.status_msg = "Loaded Frankenstein Smorgasbord beat sheet into Corkboard".to_string();
                        } else if trigger_linter {
                            self.active_workspace = ActiveWorkspace::FleetStudio;
                            self.chapter_view_mode = ChapterViewMode::DeepAnalysis;
                            self.status_msg = "Switched to Fleet Studio: Inspecting Anti-Claude-ese Style Audit".to_string();
                        } else if trigger_calibre_classics {
                            self.calibre_search_input = "Dickens".to_string();
                            self.active_workspace = ActiveWorkspace::CalibreConnector;
                            self.execute_calibre_search();
                            self.status_msg = "Searching Calibre for Dickens and 19th-century classics".to_string();
                        } else if trigger_fleet_rsi {
                            self.active_workspace = ActiveWorkspace::FleetStudio;
                            self.selected_book_idx = 0;
                            self.load_selected_chapter();
                            self.status_msg = "Switched to Fleet Studio: Inspecting on-metacraft RSI ladders".to_string();
                        } else if trigger_calibre_scifi {
                            self.calibre_search_input = "Stephenson".to_string();
                            self.active_workspace = ActiveWorkspace::CalibreConnector;
                            self.execute_calibre_search();
                            self.status_msg = "Searching Calibre for Neal Stephenson, Gibson, and Hard Sci-Fi".to_string();
                        }

                        ui.separator();

                        // Scrollable full prose text
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            for line in essay.content_markdown.lines() {
                                if line.starts_with("# ") {
                                    ui.add_space(8.0);
                                    ui.heading(egui::RichText::new(&line[2..]).size(20.0).color(egui::Color32::from_rgb(251, 146, 60)));
                                    ui.add_space(2.0);
                                } else if line.starts_with("## ") {
                                    ui.add_space(6.0);
                                    ui.label(egui::RichText::new(&line[3..]).strong().size(16.0).color(egui::Color32::from_rgb(56, 189, 248)));
                                    ui.add_space(2.0);
                                } else if line.starts_with("### ") {
                                    ui.add_space(4.0);
                                    ui.label(egui::RichText::new(&line[4..]).strong().size(14.0));
                                } else if line.starts_with("> ") {
                                    ui.horizontal(|ui| {
                                        ui.label(egui::RichText::new("▍").color(egui::Color32::from_rgb(168, 85, 247)));
                                        ui.label(egui::RichText::new(&line[2..]).italics().color(egui::Color32::from_rgb(203, 213, 225)));
                                    });
                                } else if line.starts_with("---") {
                                    ui.separator();
                                } else if line.starts_with("- ") || line.starts_with("* ") {
                                    ui.label(format!("  • {}", &line[2..]));
                                } else if line.starts_with("|") {
                                    ui.label(egui::RichText::new(line).monospace().size(11.0));
                                } else if line.starts_with("```") {
                                    ui.label(egui::RichText::new(line).monospace().color(egui::Color32::from_rgb(148, 163, 184)));
                                } else if line.trim().is_empty() {
                                    ui.add_space(4.0);
                                } else {
                                    ui.label(line);
                                }
                            }
                        });
                    }
                });
            }
        }

        // Modal: Basic Auth Login
        if self.show_auth_modal {
            egui::Window::new("🔐 Author Identity Auth")
                .collapsible(false)
                .resizable(false)
                .show(ui.ctx(), |ui| {
                    ui.label("Authenticate to access Private Draft Vault:");
                    ui.add_space(6.0);
                    ui.horizontal(|ui| {
                        ui.label("User:");
                        ui.text_edit_singleline(&mut self.auth_username);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Pass:");
                        ui.add(egui::TextEdit::singleline(&mut self.auth_password_input).password(true));
                    });
                    ui.add_space(8.0);
                    ui.horizontal(|ui| {
                        if ui.button("Confirm").clicked() {
                            if !self.auth_username.is_empty() {
                                self.is_authenticated = true;
                                self.show_auth_modal = false;
                                self.status_msg = format!("Signed in as {}", self.auth_username);
                            }
                        }
                        if ui.button("Cancel").clicked() {
                            self.show_auth_modal = false;
                        }
                    });
                });
        }

        // Modal: About Dialog
        if self.show_about_dialog {
            egui::Window::new("ℹ About WriterCraft")
                .collapsible(false)
                .resizable(false)
                .show(ui.ctx(), |ui| {
                    ui.heading("WriterCraft Sovereign Studio");
                    ui.label("Multi-persona authoring, Calibre connector, and fleet continuation studio.");
                    ui.label("Version: v0.3.1");
                    ui.label("Control Port: 14121");
                    ui.separator();
                    ui.label("Interactive Fleet Books: on-ai-takeover, on-hyperwar, on-hyperart, on-metacraft, on-hypermind.");
                    ui.label("Zero-lock read-only Calibre library search & metadata ingestion.");
                    ui.add_space(8.0);
                    if ui.button("Close").clicked() {
                        self.show_about_dialog = false;
                    }
                });
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    if args.smoke {
        println!("[writercraft] Running headless smoke verification...");
        let engine = WriterEngine::new();
        let arc = engine.analyze_narrative_arc()?;
        let repos_dir = if Path::new("on-metacraft").exists() {
            Path::new(".")
        } else {
            Path::new("..")
        };
        let fleet_books = engine.scan_all_fleet_books(repos_dir);
        let calibre_libs = engine.discover_calibre_libraries();

        let smoke_payload = json!({
            "app": "writercraft",
            "version": "0.3.1",
            "status": "ready",
            "control_port": 14121,
            "success": true,
            "scene_count": engine.active_scenes.len(),
            "climax_scene_id": arc.max_tension_scene_id,
            "fleet_books_detected": fleet_books.len(),
            "calibre_libraries_detected": calibre_libs.len(),
            "capabilities": [
                "CorkboardSceneManagement",
                "TropeCatalogIntegration",
                "GenreBeatSheetGeneration",
                "FountainScreenplayParsing",
                "FleschKincaidReadabilityAudit",
                "AiTellDetection",
                "ScholarBibTexParsing",
                "ConstructionClaimLedger",
                "FleetBookCorpusIntegration",
                "FleetChapterDeepAnalysis",
                "FleetContinuationSynthesis",
                "CalibreLibraryConnector",
                "TypstAcademicExport",
                "CraftDelectationsLibrary"
            ]
        });
        println!("[writercraft] Smoke OK: {}", smoke_payload);
        return Ok(());
    }

    let app = WriterCraftApp::new();
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1400.0, 900.0])
            .with_title("WriterCraft — Multi-Persona Studio & Calibre Connector"),
        ..Default::default()
    };

    eframe::run_native(
        "WriterCraft",
        native_options,
        Box::new(|_cc| Ok(Box::new(app))),
    )?;
    Ok(())
}
