//! Narrative & Non-Fiction Continuation Engine for Fleet Books
//!
//! Provides deep chapter analysis (tension, claims, AI-tells, readability)
//! and synthesizes structured continuation scaffolds (thematic progression,
//! claim-resolution beats, and zero-slop multi-model drafting prompts).

use crate::readability::{AiTellMatch, ReadabilityAudit, ReadabilityAuditor};
use crate::scholar::{ConstructionClaim, ScholarAuditor};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChapterDeepAnalysis {
    pub file_name: String,
    pub title: String,
    pub word_count: usize,
    pub readability: ReadabilityAudit,
    pub claims: Vec<ConstructionClaim>,
    pub closing_excerpt: String,
    pub detected_tells: Vec<AiTellMatch>,
    pub tension_score: f64, // [-1.0, 1.0]
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ContinuationBeat {
    pub beat_number: usize,
    pub title: String,
    pub objective: String,
    pub tension_shift: f64,
    pub dialectic_element: String, // "Thesis Breach", "Physical Firewall Obstacle", "Irreversible Baseline"
    pub unresolved_claims_addressed: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ContinuationScaffold {
    pub source_book_slug: String,
    pub source_chapter_file: String,
    pub next_chapter_suggested_title: String,
    pub core_thematic_rung: String,
    pub beats: Vec<ContinuationBeat>,
    pub multi_model_drafting_prompt: String,
}

pub struct ContinuationEngine;

impl ContinuationEngine {
    /// Performs deep multi-dimensional analysis on chapter content
    pub fn analyze_chapter(file_name: &str, content: &str) -> ChapterDeepAnalysis {
        let title = content
            .lines()
            .find(|l| l.trim_start().starts_with("# "))
            .map(|l| l.trim_start()[2..].trim().to_string())
            .unwrap_or_else(|| file_name.replace(".md", ""));

        let word_count = content.split_whitespace().count();
        let readability = ReadabilityAuditor::audit(content);
        let claims = ScholarAuditor::extract_claims(file_name, content);
        let detected_tells = readability.detected_cliches_and_ai_tells.clone();

        // Closing excerpt (last 3 non-empty lines)
        let non_empty_lines: Vec<&str> = content
            .lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .collect();

        let closing_excerpt = if non_empty_lines.is_empty() {
            String::new()
        } else {
            let start = non_empty_lines.len().saturating_sub(3);
            non_empty_lines[start..].join("\n")
        };

        // Tension score heuristic:
        // More unresolved claims and lower reading ease/high grade level correlates with critical tension
        let claim_factor = (claims.len() as f64 * -0.15).max(-0.6);
        let readability_factor = if readability.flesch_reading_ease < 40.0 { -0.3 } else { 0.1 };
        let tension_score = (claim_factor + readability_factor).clamp(-1.0, 1.0);

        ChapterDeepAnalysis {
            file_name: file_name.to_string(),
            title,
            word_count,
            readability,
            claims,
            closing_excerpt,
            detected_tells,
            tension_score,
        }
    }

    /// Synthesizes continuation beats and a drafting prompt for the next chapter
    pub fn synthesize_continuation(
        book_slug: &str,
        current_chapter: &ChapterDeepAnalysis,
        next_file_name: Option<&str>,
    ) -> ContinuationScaffold {
        let next_title = match next_file_name {
            Some(f) => f.replace(".md", "").replace('-', " "),
            None => format!("Next Chapter following {}", current_chapter.file_name),
        };

        let target_rung = match book_slug {
            "on-ai-takeover" => "Kahn Escalation Rung: Autonomous Strategic Release Threshold",
            "on-hyperwar" => "Operational Speed Horizon: Autonomous Kill-Chain Authority",
            "on-hyperart" => "Aesthetic Singularity: Curation Oracle & Style Commodification",
            "on-metacraft" => "Autonomy Ladder L3->L4: The Physical Firewall & Hardware Test-Coupon Barrier",
            "on-hypermind" => "Spectator Epistemology: Proof Verification without Human Comprehension",
            _ => "The Next Dialectic Threshold",
        };

        let mut claims_to_address: Vec<String> = current_chapter
            .claims
            .iter()
            .map(|c| c.claim_text.clone())
            .take(3)
            .collect();

        if claims_to_address.is_empty() {
            claims_to_address.push("Anchor chapter with dated empirical facts and named institutions".to_string());
        }

        let beats = vec![
            ContinuationBeat {
                beat_number: 1,
                title: "The Opening Extrapolation".to_string(),
                objective: format!("Extend the argument past the threshold established in '{}'", current_chapter.title),
                tension_shift: -0.2,
                dialectic_element: "Thesis Breach".to_string(),
                unresolved_claims_addressed: claims_to_address.clone(),
            },
            ContinuationBeat {
                beat_number: 2,
                title: "The Firewall Collision".to_string(),
                objective: "Confront the automated momentum with empirical friction (hardware, energy, regulatory checks)".to_string(),
                tension_shift: -0.6,
                dialectic_element: "Physical Firewall Obstacle".to_string(),
                unresolved_claims_addressed: vec!["Audit institutional levers and physical test coupons".to_string()],
            },
            ContinuationBeat {
                beat_number: 3,
                title: "The Irreversible Baseline".to_string(),
                objective: "Establish the new equilibrium where human agency retains or surrenders the sovereign brake".to_string(),
                tension_shift: 0.4,
                dialectic_element: "Irreversible Baseline".to_string(),
                unresolved_claims_addressed: vec!["Confirm milestone date and governance outcome".to_string()],
            },
        ];

        let drafting_prompt = format!(
            "You are drafting the continuation for the fleet book '{book_slug}' following chapter '{current_title}'.\n\
             Core Thematic Rung: {target_rung}\n\
             Ending Context:\n\"{closing}\"\n\n\
             Rules & Guardrails:\n\
             1. Strict No-AI-Tells policy: Never write 'a testament to', 'delve into', 'tapestry of', 'beacon of hope'.\n\
             2. Anchor all claims with dated verifiable facts (Current fleet year: 2026).\n\
             3. Mark any unverified empirical claim with `[construction: <task>]`.\n\
             4. Follow the 3-beat structure: Thesis Breach -> Physical Firewall -> Irreversible Baseline.",
            book_slug = book_slug,
            current_title = current_chapter.title,
            target_rung = target_rung,
            closing = current_chapter.closing_excerpt
        );

        ContinuationScaffold {
            source_book_slug: book_slug.to_string(),
            source_chapter_file: current_chapter.file_name.clone(),
            next_chapter_suggested_title: next_title,
            core_thematic_rung: target_rung.to_string(),
            beats,
            multi_model_drafting_prompt: drafting_prompt,
        }
    }
}
