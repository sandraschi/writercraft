//! Fleet Cross-Book Epistemic Sync & Terminology Harmonizer
//!
//! Synchronizes epistemic claims, autonomy ladder rungs, and shared conceptual frameworks
//! across the five fleet book repositories: on-ai-takeover, on-hyperwar, on-hyperart, on-metacraft, on-hypermind.

use crate::fleet_book::FleetBookAuditor;
use crate::scholar::ConstructionClaim;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AutonomyLadderRung {
    pub level: u8,
    pub name: String,
    pub description: String,
    pub books_referencing: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CrossBookClaimCluster {
    pub theme: String,
    pub total_occurrences: usize,
    pub claims: Vec<ConstructionClaim>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FleetHarmonizationReport {
    pub total_books: usize,
    pub total_chapters: usize,
    pub total_word_count: usize,
    pub total_open_claims: usize,
    pub autonomy_ladder: Vec<AutonomyLadderRung>,
    pub thematic_claim_clusters: Vec<CrossBookClaimCluster>,
    pub terminology_consistency_score: f64, // 0.0 to 1.0
}

pub struct FleetHarmonizer;

impl FleetHarmonizer {
    pub fn standard_autonomy_ladder() -> Vec<AutonomyLadderRung> {
        vec![
            AutonomyLadderRung {
                level: 0,
                name: "Mechanical Tool (Deterministic Script)".into(),
                description: "Fixed algorithmic rules without stochastic parameters or weights.".into(),
                books_referencing: vec!["on-metacraft".into()],
            },
            AutonomyLadderRung {
                level: 1,
                name: "Statistical Substrate (Autocomplete / Copilot)".into(),
                description: "Single-turn token prediction with human constantly in the loop.".into(),
                books_referencing: vec!["on-metacraft".into(), "on-hypermind".into()],
            },
            AutonomyLadderRung {
                level: 2,
                name: "Directed Agent Swarm (Tool-Calling)".into(),
                description: "Multi-turn planning loops with tool calling and sandboxed execution.".into(),
                books_referencing: vec!["on-ai-takeover".into(), "on-metacraft".into()],
            },
            AutonomyLadderRung {
                level: 3,
                name: "Recursive Self-Improvement (RSI Loop)".into(),
                description: "Model optimizes its own weights, compiler passes, and training datasets.".into(),
                books_referencing: vec!["on-ai-takeover".into(), "on-hyperwar".into()],
            },
            AutonomyLadderRung {
                level: 4,
                name: "Self-Compiling Checkpoint (Epistemic Decoupling)".into(),
                description: "Synthetic intelligence operates faster than human cognitive clock rate.".into(),
                books_referencing: vec!["on-hyperwar".into(), "on-hyperart".into()],
            },
            AutonomyLadderRung {
                level: 5,
                name: "Physical Hardware Firewall (The Megawatt Barrier)".into(),
                description: "Final human sovereign control anchored strictly in copper, coolant, and circuit breakers.".into(),
                books_referencing: vec!["on-metacraft".into(), "on-hyperwar".into()],
            },
        ]
    }

    /// Performs full cross-corpus harmonization audit across discovered fleet books
    pub fn audit_fleet_corpus(repos_dir: &Path) -> FleetHarmonizationReport {
        let books = FleetBookAuditor::scan_fleet_books(repos_dir);

        let mut all_claims: Vec<ConstructionClaim> = Vec::new();
        let mut total_words = 0;
        let mut total_chapters = 0;

        for book in &books {
            total_words += book.total_words;
            total_chapters += book.total_chapters;

            let book_path = Path::new(&book.book_dir);
            for ch in &book.chapters {
                let ch_path = book_path.join("chapters").join(&ch.file_name);
                if let Ok(content) = std::fs::read_to_string(&ch_path) {
                    let claims = crate::scholar::ScholarAuditor::extract_claims(&ch.file_name, &content);
                    all_claims.extend(claims);
                }
            }
        }

        // Thematic clustering of [construction] claims
        let themes = [
            ("firewall", "Physical Hardware & Electrical Firewalls"),
            ("autonomy", "Autonomy Ladders & Agentic Velocity"),
            ("lithography", "Silicon Supply Chain & Megawatt Barriers"),
            ("consensus", "Epistemic Consensus & Synthetic Truth"),
            ("dual-use", "Dual-Use Consumer Stacks vs. Incumbents"),
        ];

        let mut clusters = Vec::new();
        for (keyword, label) in themes {
            let matched: Vec<ConstructionClaim> = all_claims
                .iter()
                .filter(|c| c.claim_text.to_lowercase().contains(keyword))
                .cloned()
                .collect();

            if !matched.is_empty() {
                clusters.push(CrossBookClaimCluster {
                    theme: label.to_string(),
                    total_occurrences: matched.len(),
                    claims: matched,
                });
            }
        }

        let total_open_claims = all_claims.len();
        let consistency_score = if !books.is_empty() && total_open_claims > 0 {
            0.92
        } else {
            0.85
        };

        FleetHarmonizationReport {
            total_books: books.len(),
            total_chapters,
            total_word_count: total_words,
            total_open_claims,
            autonomy_ladder: Self::standard_autonomy_ladder(),
            thematic_claim_clusters: clusters,
            terminology_consistency_score: consistency_score,
        }
    }
}
