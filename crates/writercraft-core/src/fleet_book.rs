//! Fleet Book Integration for WriterCraft
//!
//! Inspects and connects the five "On..." book repositories:
//! (on-ai-takeover, on-hyperwar, on-hyperart, on-metacraft, on-hypermind)
//! by reading `book.toml`, chapter manifests, word targets, and [construction] claim ledgers.

use crate::readability::ReadabilityAuditor;
use crate::scholar::ScholarAuditor;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FleetBookManifest {
    pub slug: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub author: String,
    pub port: Option<u16>,
    pub theme: Option<String>,
    pub chapter_files: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChapterSummary {
    pub file_name: String,
    pub word_count: usize,
    pub claims_count: usize,
    pub flesch_reading_ease: f64,
    pub flesch_kincaid_grade: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FleetBookInspection {
    pub manifest: FleetBookManifest,
    pub book_dir: String,
    pub total_words: usize,
    pub total_claims: usize,
    pub total_chapters: usize,
    pub chapters: Vec<ChapterSummary>,
}

pub struct FleetBookAuditor;

impl FleetBookAuditor {
    /// Inspects a book directory containing a `book.toml`
    pub fn inspect_book(dir: &Path) -> Result<FleetBookInspection, String> {
        let toml_path = dir.join("book.toml");
        if !toml_path.exists() {
            return Err(format!("No book.toml found in {}", dir.display()));
        }

        let toml_content = fs::read_to_string(&toml_path)
            .map_err(|e| format!("Failed to read {}: {}", toml_path.display(), e))?;

        let manifest = Self::parse_book_toml(&toml_content)?;

        let chapters_dir = dir.join("chapters");
        let mut chapter_summaries = Vec::new();
        let mut total_words = 0;
        let mut total_claims = 0;

        for ch_file in &manifest.chapter_files {
            let ch_path = chapters_dir.join(ch_file);
            if ch_path.exists() {
                if let Ok(content) = fs::read_to_string(&ch_path) {
                    let words = content.split_whitespace().count();
                    let claims = ScholarAuditor::extract_claims(ch_file, &content);
                    let read_audit = ReadabilityAuditor::audit(&content);

                    total_words += words;
                    total_claims += claims.len();

                    chapter_summaries.push(ChapterSummary {
                        file_name: ch_file.clone(),
                        word_count: words,
                        claims_count: claims.len(),
                        flesch_reading_ease: read_audit.flesch_reading_ease,
                        flesch_kincaid_grade: read_audit.flesch_kincaid_grade,
                    });
                }
            }
        }

        Ok(FleetBookInspection {
            manifest,
            book_dir: dir.to_string_lossy().to_string(),
            total_words,
            total_claims,
            total_chapters: chapter_summaries.len(),
            chapters: chapter_summaries,
        })
    }

    /// Finds all fleet books relative to a base repos directory, with fallback to current directory
    pub fn scan_fleet_books(repos_dir: &Path) -> Vec<FleetBookInspection> {
        let candidates = [
            "on-ai-takeover",
            "on-hyperwar",
            "on-hyperart",
            "on-metacraft",
            "on-hypermind",
        ];

        let mut check_dirs = vec![repos_dir.to_path_buf()];
        if repos_dir != Path::new(".") {
            check_dirs.push(Path::new(".").to_path_buf());
        }
        if repos_dir != Path::new("..") {
            check_dirs.push(Path::new("..").to_path_buf());
        }

        for base in check_dirs {
            let mut found = Vec::new();
            for slug in candidates {
                let book_path = base.join(slug);
                if book_path.is_dir() {
                    if let Ok(inspection) = Self::inspect_book(&book_path) {
                        found.push(inspection);
                    }
                }
            }
            if !found.is_empty() {
                return found;
            }
        }
        Vec::new()
    }

    fn parse_book_toml(content: &str) -> Result<FleetBookManifest, String> {
        let mut slug = String::new();
        let mut title = String::new();
        let mut subtitle = None;
        let mut author = "Sandra Schipal".to_string();
        let mut port = None;
        let mut theme = None;
        let mut chapter_files = Vec::new();

        let mut in_book_sec = false;
        let mut in_chapters_sec = false;
        let mut in_order_array = false;

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with('#') || trimmed.is_empty() {
                continue;
            }

            if trimmed.starts_with('[') {
                in_book_sec = trimmed == "[book]";
                in_chapters_sec = trimmed == "[chapters]";
                in_order_array = false;
                continue;
            }

            if in_book_sec {
                if let Some(eq) = trimmed.find('=') {
                    let key = trimmed[..eq].trim();
                    let val = trimmed[eq + 1..].trim().trim_matches('"');
                    match key {
                        "slug" => slug = val.to_string(),
                        "title" => title = val.to_string(),
                        "subtitle" => subtitle = Some(val.to_string()),
                        "author" => author = val.to_string(),
                        "theme" => theme = Some(val.to_string()),
                        "port" => port = val.parse::<u16>().ok(),
                        _ => {}
                    }
                }
            } else if in_chapters_sec {
                if trimmed.starts_with("order = [") || trimmed == "order = [" {
                    in_order_array = true;
                    continue;
                }
                if in_order_array {
                    if trimmed.starts_with(']') {
                        in_order_array = false;
                        continue;
                    }
                    let clean = trimmed.trim_matches(|c| c == ',' || c == '"' || c == ' ');
                    if !clean.is_empty() && clean.ends_with(".md") {
                        chapter_files.push(clean.to_string());
                    }
                }
            }
        }

        if title.is_empty() {
            title = slug.clone();
        }

        Ok(FleetBookManifest {
            slug,
            title,
            subtitle,
            author,
            port,
            theme,
            chapter_files,
        })
    }
}
