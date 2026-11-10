//! Readability & Prose Quality Auditor for WriterCraft
//!
//! Computes Flesch Reading Ease, Flesch-Kincaid Grade Level,
//! sentence complexity distribution, and AI cliché / tell heuristics
//! (aligned with the fleet BookKit critic standard).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReadabilityAudit {
    pub total_words: usize,
    pub total_sentences: usize,
    pub total_syllables: usize,
    pub words_per_sentence: f64,
    pub syllables_per_word: f64,
    pub flesch_reading_ease: f64,
    pub flesch_kincaid_grade: f64,
    pub readability_band: String, // e.g. "College / Academic", "Plain English", "Elementary"
    pub detected_cliches_and_ai_tells: Vec<AiTellMatch>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AiTellMatch {
    pub phrase: String,
    pub count: usize,
    pub category: String, // "AI Buzzword", "Sensory Cliche", "Overly Formal Diction"
}

pub struct ReadabilityAuditor;

impl ReadabilityAuditor {
    /// Counts approximate syllables in an English word
    pub fn count_syllables_in_word(raw: &str) -> usize {
        let clean: String = raw
            .chars()
            .filter(|c| c.is_alphabetic())
            .flat_map(|c| c.to_lowercase())
            .collect();

        if clean.is_empty() {
            return 1;
        }
        if clean.len() <= 3 {
            return 1;
        }

        let vowels = ['a', 'e', 'i', 'o', 'u', 'y'];
        let mut count = 0;
        let mut prev_is_vowel = false;

        for c in clean.chars() {
            let is_v = vowels.contains(&c);
            if is_v && !prev_is_vowel {
                count += 1;
            }
            prev_is_vowel = is_v;
        }

        // Silent 'e' at end
        if clean.ends_with('e') && !clean.ends_with("le") && count > 1 {
            count -= 1;
        }

        count.max(1)
    }

    pub fn audit(text: &str) -> ReadabilityAudit {
        let words: Vec<&str> = text
            .split_whitespace()
            .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()))
            .filter(|w| !w.is_empty())
            .collect();

        let total_words = words.len().max(1);

        // Count sentences based on punctuation
        let sentence_delimiters = ['.', '!', '?'];
        let total_sentences = text
            .chars()
            .filter(|c| sentence_delimiters.contains(c))
            .count()
            .max(1);

        let total_syllables: usize = words
            .iter()
            .map(|w| Self::count_syllables_in_word(w))
            .sum();

        let words_per_sentence = total_words as f64 / total_sentences as f64;
        let syllables_per_word = total_syllables as f64 / total_words as f64;

        // Flesch Reading Ease = 206.835 - (1.015 * ASL) - (84.6 * ASW)
        let flesch_reading_ease =
            206.835 - (1.015 * words_per_sentence) - (84.6 * syllables_per_word);

        // Flesch-Kincaid Grade Level = (0.39 * ASL) + (11.8 * ASW) - 15.59
        let flesch_kincaid_grade =
            (0.39 * words_per_sentence) + (11.8 * syllables_per_word) - 15.59;

        let readability_band = if flesch_reading_ease >= 90.0 {
            "5th Grade / Very Easy (Kids Edition)".to_string()
        } else if flesch_reading_ease >= 70.0 {
            "7th-8th Grade / Plain English".to_string()
        } else if flesch_reading_ease >= 60.0 {
            "High School / Standard Fiction & Non-Fiction".to_string()
        } else if flesch_reading_ease >= 40.0 {
            "College / Difficult".to_string()
        } else {
            "Academic / Post-Graduate / ArXiv Paper".to_string()
        };

        // Scan for AI tells & cliches
        let tell_patterns = [
            ("a testament to", "AI Buzzword"),
            ("beacon of hope", "AI Buzzword"),
            ("delve into", "AI Buzzword"),
            ("delve", "AI Buzzword"),
            ("tapestry of", "AI Buzzword"),
            ("ever-evolving", "AI Buzzword"),
            ("in summary", "Overly Formal Diction"),
            ("pivotal moment", "AI Buzzword"),
            ("eyes darkened with", "Sensory Cliche"),
            ("shivers ran down", "Sensory Cliche"),
            ("breath he didn't realize", "Sensory Cliche"),
            ("breath she didn't realize", "Sensory Cliche"),
            ("it is important to remember", "Overly Formal Diction"),
            ("nexus of", "AI Buzzword"),
            ("uncharted waters", "AI Buzzword"),
        ];

        let lower = text.to_lowercase();
        let mut detected_cliches_and_ai_tells = Vec::new();

        for (pattern, category) in tell_patterns {
            let mut matches = 0;
            let mut search_from = 0;
            while let Some(idx) = lower[search_from..].find(pattern) {
                matches += 1;
                search_from += idx + pattern.len();
            }
            if matches > 0 {
                detected_cliches_and_ai_tells.push(AiTellMatch {
                    phrase: pattern.to_string(),
                    count: matches,
                    category: category.to_string(),
                });
            }
        }

        ReadabilityAudit {
            total_words,
            total_sentences,
            total_syllables,
            words_per_sentence,
            syllables_per_word,
            flesch_reading_ease,
            flesch_kincaid_grade,
            readability_band,
            detected_cliches_and_ai_tells,
        }
    }
}
