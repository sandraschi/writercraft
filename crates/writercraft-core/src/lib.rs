//! WriterCraft Core Engine
//!
//! Provides corkboard scene graph management, narrative tension curve analysis,
//! Fountain screenplay parsing, academic BibTeX / claim auditing, readability scoring,
//! Calibre library connector, deep chapter analysis, and fleet continuation scaffolding.

pub mod calibre;
pub mod companion;
pub mod continuation;
pub mod delectation;
pub mod fleet_book;
pub mod fountain;
pub mod i18n;
pub mod readability;
pub mod scholar;
pub mod style_guide;

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use thiserror::Error;
use writercraft_narrative::{BeatSheet, NarrativeModel, SceneCard, Trope};

#[derive(Error, Debug)]
pub enum WriterError {
    #[error("Manuscript error: {0}")]
    Manuscript(String),
    #[error("IO error: {0}")]
    Io(String),
    #[error("Calibre error: {0}")]
    Calibre(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NarrativeArcAnalysis {
    pub total_scenes: usize,
    pub average_valence: f64,
    pub max_tension_scene_id: String,
    pub tropes_utilized: Vec<String>,
}

pub struct WriterEngine {
    pub active_scenes: Vec<SceneCard>,
}

impl WriterEngine {
    pub fn new() -> Self {
        Self {
            active_scenes: NarrativeModel::mock_act_one_scenes(),
        }
    }

    pub fn set_scenes(&mut self, scenes: Vec<SceneCard>) {
        self.active_scenes = scenes;
    }

    pub fn analyze_narrative_arc(&self) -> Result<NarrativeArcAnalysis, WriterError> {
        if self.active_scenes.is_empty() {
            return Err(WriterError::Manuscript("No scenes in project".to_string()));
        }

        let mut sum_valence = 0.0;
        let mut min_val = 1.0;
        let mut min_id = String::new();
        let mut tropes = Vec::new();

        for s in &self.active_scenes {
            sum_valence += s.emotional_valence;
            if s.emotional_valence < min_val {
                min_val = s.emotional_valence;
                min_id = s.scene_id.clone();
            }
            for t in &s.associated_tropes {
                if !tropes.contains(t) {
                    tropes.push(t.clone());
                }
            }
        }

        Ok(NarrativeArcAnalysis {
            total_scenes: self.active_scenes.len(),
            average_valence: sum_valence / (self.active_scenes.len() as f64),
            max_tension_scene_id: min_id,
            tropes_utilized: tropes,
        })
    }

    pub fn generate_beats(&mut self, genre: &str, title: &str, protagonist: &str) -> BeatSheet {
        let sheet = NarrativeModel::generate_beat_sheet(genre, title, protagonist);
        self.active_scenes = sheet.beats.clone();
        sheet
    }

    pub fn get_tropes(&self) -> Vec<Trope> {
        NarrativeModel::trope_catalog()
    }

    pub fn parse_fountain(&self, script_text: &str) -> fountain::ScreenplayAnalysis {
        fountain::FountainParser::parse(script_text)
    }

    pub fn audit_readability(&self, text: &str) -> readability::ReadabilityAudit {
        readability::ReadabilityAuditor::audit(text)
    }

    pub fn audit_style_guide(&self, text: &str) -> style_guide::StyleAudit {
        style_guide::StyleLinter::lint(text)
    }

    pub fn companion_draft_chapter(&self, req: &companion::ChapterDraftRequest) -> companion::ChapterDraftResponse {
        companion::CompanionEngine::draft_next_chapter(req)
    }

    pub fn companion_syndicate(&self, source_title: &str, source_text: &str, out_dir: Option<&Path>) -> Result<companion::SyndicationDrafts, String> {
        companion::CompanionEngine::generate_syndication(source_title, source_text, out_dir)
    }

    pub fn parse_bibtex(&self, bib_content: &str) -> Vec<scholar::BibEntry> {
        scholar::ScholarAuditor::parse_bibtex(bib_content)
    }

    pub fn audit_claims(&self, file_name: &str, text: &str) -> Vec<scholar::ConstructionClaim> {
        scholar::ScholarAuditor::extract_claims(file_name, text)
    }

    pub fn inspect_fleet_book(&self, book_dir: &Path) -> Result<fleet_book::FleetBookInspection, String> {
        fleet_book::FleetBookAuditor::inspect_book(book_dir)
    }

    pub fn scan_all_fleet_books(&self, repos_dir: &Path) -> Vec<fleet_book::FleetBookInspection> {
        fleet_book::FleetBookAuditor::scan_fleet_books(repos_dir)
    }

    pub fn read_chapter_content(&self, book_dir: &Path, file_name: &str) -> Result<String, String> {
        let direct_path = book_dir.join("chapters").join(file_name);
        if direct_path.exists() {
            return fs::read_to_string(&direct_path)
                .map_err(|e| format!("Failed to read {}: {}", direct_path.display(), e));
        }

        let fallback_parent = Path::new("..").join(book_dir).join("chapters").join(file_name);
        if fallback_parent.exists() {
            return fs::read_to_string(&fallback_parent)
                .map_err(|e| format!("Failed to read {}: {}", fallback_parent.display(), e));
        }

        let fallback_curr = Path::new(".").join(book_dir).join("chapters").join(file_name);
        if fallback_curr.exists() {
            return fs::read_to_string(&fallback_curr)
                .map_err(|e| format!("Failed to read {}: {}", fallback_curr.display(), e));
        }

        Err(format!("Chapter file '{}' not found in {}", file_name, book_dir.display()))
    }

    pub fn analyze_chapter_deep(&self, file_name: &str, content: &str) -> continuation::ChapterDeepAnalysis {
        continuation::ContinuationEngine::analyze_chapter(file_name, content)
    }

    pub fn synthesize_continuation(
        &self,
        book_slug: &str,
        current_chapter: &continuation::ChapterDeepAnalysis,
        next_file_name: Option<&str>,
    ) -> continuation::ContinuationScaffold {
        continuation::ContinuationEngine::synthesize_continuation(book_slug, current_chapter, next_file_name)
    }

    // Calibre Connector functions
    pub fn discover_calibre_libraries(&self) -> Vec<calibre::CalibreLibraryInfo> {
        calibre::CalibreConnector::discover_libraries()
    }

    pub fn search_calibre(&self, library_dir: &Path, query: &str, limit: usize) -> Result<Vec<calibre::CalibreBook>, String> {
        calibre::CalibreConnector::search_books(library_dir, query, limit)
    }

    pub fn get_calibre_book(&self, library_dir: &Path, book_id: i64) -> Result<Option<calibre::CalibreBook>, String> {
        calibre::CalibreConnector::get_book(library_dir, book_id)
    }

    // Craft Delectations & Trope Essays
    pub fn list_craft_essays(&self) -> Vec<delectation::CraftEssayInfo> {
        delectation::list_essay_infos()
    }

    pub fn get_craft_essay(&self, slug: &str) -> Option<delectation::CraftEssay> {
        delectation::get_essay(slug)
    }

    pub fn generate_frankenstein_beats(&self, title: &str, protagonist: &str) -> BeatSheet {
        NarrativeModel::generate_beat_sheet("frankenstein", title, protagonist)
    }
}
