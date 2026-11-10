//! Scholar & Epistemic Tools for WriterCraft
//!
//! Provides BibTeX citation parsing, [construction] claim ledger extraction,
//! and Typst / LaTeX academic paper scaffolding.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BibEntry {
    pub entry_type: String, // "article", "book", "misc"
    pub cite_key: String,
    pub title: String,
    pub author: String,
    pub year: Option<String>,
    pub journal_or_publisher: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConstructionClaim {
    pub file_name: String,
    pub line_number: usize,
    pub claim_text: String,
}

pub struct ScholarAuditor;

impl ScholarAuditor {
    /// Extracts `[construction: ...]` tags from text
    pub fn extract_claims(file_name: &str, content: &str) -> Vec<ConstructionClaim> {
        let mut claims = Vec::new();
        for (idx, line) in content.lines().enumerate() {
            let mut search_from = 0;
            while let Some(start_idx) = line[search_from..].find("[construction:") {
                let abs_start = search_from + start_idx;
                if let Some(end_idx) = line[abs_start..].find(']') {
                    let abs_end = abs_start + end_idx;
                    let tag_content = &line[abs_start + "[construction:".len()..abs_end].trim();
                    claims.push(ConstructionClaim {
                        file_name: file_name.to_string(),
                        line_number: idx + 1,
                        claim_text: tag_content.to_string(),
                    });
                    search_from = abs_end + 1;
                } else {
                    break;
                }
            }
        }
        claims
    }

    /// Parses standard BibTeX entries
    pub fn parse_bibtex(bib_content: &str) -> Vec<BibEntry> {
        let mut entries = Vec::new();
        let mut current_type = String::new();
        let mut current_key = String::new();
        let mut current_title = String::new();
        let mut current_author = String::new();
        let mut current_year: Option<String> = None;
        let mut current_journal: Option<String> = None;
        let mut inside_entry = false;

        for raw_line in bib_content.lines() {
            let line = raw_line.trim();
            if line.starts_with('@') {
                if inside_entry && !current_key.is_empty() {
                    entries.push(BibEntry {
                        entry_type: std::mem::take(&mut current_type),
                        cite_key: std::mem::take(&mut current_key),
                        title: std::mem::take(&mut current_title),
                        author: std::mem::take(&mut current_author),
                        year: current_year.take(),
                        journal_or_publisher: current_journal.take(),
                    });
                }
                inside_entry = true;
                if let Some(brace_idx) = line.find('{') {
                    current_type = line[1..brace_idx].trim().to_lowercase();
                    let rest = &line[brace_idx + 1..];
                    current_key = rest.trim_end_matches(',').trim().to_string();
                }
                continue;
            }

            if inside_entry {
                if line.starts_with('}') && !line.contains('=') {
                    if !current_key.is_empty() {
                        entries.push(BibEntry {
                            entry_type: std::mem::take(&mut current_type),
                            cite_key: std::mem::take(&mut current_key),
                            title: std::mem::take(&mut current_title),
                            author: std::mem::take(&mut current_author),
                            year: current_year.take(),
                            journal_or_publisher: current_journal.take(),
                        });
                    }
                    inside_entry = false;
                    continue;
                }

                if let Some(eq_idx) = line.find('=') {
                    let field = line[..eq_idx].trim().to_lowercase();
                    let val = line[eq_idx + 1..]
                        .trim()
                        .trim_matches(|c| c == '{' || c == '}' || c == '"' || c == ',');
                    match field.as_str() {
                        "title" => current_title = val.to_string(),
                        "author" => current_author = val.to_string(),
                        "year" => current_year = Some(val.to_string()),
                        "journal" | "publisher" | "booktitle" => {
                            current_journal = Some(val.to_string())
                        }
                        _ => {}
                    }
                }
            }
        }

        if inside_entry && !current_key.is_empty() {
            entries.push(BibEntry {
                entry_type: current_type,
                cite_key: current_key,
                title: current_title,
                author: current_author,
                year: current_year,
                journal_or_publisher: current_journal,
            });
        }

        entries
    }

    /// Exports a markdown manuscript to clean Typst markup for compilation
    pub fn export_to_typst(
        title: &str,
        authors: &[&str],
        abstract_text: &str,
        markdown_body: &str,
    ) -> String {
        let mut out = String::new();
        out.push_str(&format!("#set document(title: \"{title}\")\n"));
        out.push_str("#set page(paper: \"a4\", numbering: \"1\")\n");
        out.push_str("#set text(font: \"Libertinus Serif\", size: 10pt)\n\n");
        out.push_str(&format!("= {title}\n\n"));

        let authors_joined = authors.join(", ");
        out.push_str(&format!("*Authors:* {authors_joined}\n\n"));

        if !abstract_text.is_empty() {
            out.push_str("== Abstract\n\n");
            out.push_str(abstract_text);
            out.push_str("\n\n");
        }

        out.push_str("== Introduction & Main Text\n\n");
        for line in markdown_body.lines() {
            if line.starts_with("# ") {
                out.push_str(&format!("= {}\n\n", &line[2..]));
            } else if line.starts_with("## ") {
                out.push_str(&format!("== {}\n\n", &line[3..]));
            } else if line.starts_with("### ") {
                out.push_str(&format!("=== {}\n\n", &line[4..]));
            } else {
                out.push_str(line);
                out.push('\n');
            }
        }

        out
    }
}
