//! Character Idiolect & Dialogue Voice Profiler
//!
//! Analyzes dialogue turns across characters in Fountain and prose manuscripts,
//! measuring cadence, lexical diversity (Type-Token Ratio), contraction frequency,
//! distinctiveness scores, and third-person limited POV head-hopping.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CharacterVoiceProfile {
    pub character: String,
    pub total_words: usize,
    pub speaking_turns: usize,
    pub mean_turn_length_words: f64,
    pub type_token_ratio: f64,
    pub contraction_ratio: f64,
    pub mean_sentence_length: f64,
    pub cadence_variance: f64,
    pub distinctive_terms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HeadHoppingIncident {
    pub line_number: usize,
    pub declared_pov: String,
    pub intruding_character: String,
    pub cognitive_verb: String,
    pub context_snippet: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IdiolectAuditReport {
    pub character_profiles: Vec<CharacterVoiceProfile>,
    pub overall_voice_divergence: f64, // 0.0 (all characters sound identical) to 1.0 (highly distinct idiolects)
    pub head_hopping_alerts: Vec<HeadHoppingIncident>,
}

pub struct IdiolectAnalyzer;

impl IdiolectAnalyzer {
    /// Common English contractions for formal/informal dialogue analysis
    const CONTRACTIONS: &'static [&'static str] = &[
        "don't", "can't", "won't", "didn't", "haven't", "hasn't", "isn't", "aren't",
        "wasn't", "weren't", "i'm", "i've", "i'll", "you're", "you've", "you'll",
        "he's", "she's", "it's", "we're", "they're", "that's", "what's", "there's",
    ];

    /// Internal cognitive verbs that betray POV head-hopping when applied to non-POV characters
    const COGNITIVE_VERBS: &'static [&'static str] = &[
        "thought", "felt", "knew", "wondered", "realized", "remembered", "secretly wanted",
        "hoped", "believed in his heart", "believed in her heart", "dreaded",
    ];

    /// Analyzes character dialogue turns extracted from Fountain screenplay text
    pub fn audit_dialogue_turns(turns: &[(String, String)]) -> IdiolectAuditReport {
        let mut char_words: HashMap<String, Vec<String>> = HashMap::new();
        let mut char_turn_counts: HashMap<String, usize> = HashMap::new();
        let mut corpus_word_counts: HashMap<String, usize> = HashMap::new();

        for (char_name, speech) in turns {
            *char_turn_counts.entry(char_name.clone()).or_insert(0) += 1;
            let words: Vec<String> = speech
                .split_whitespace()
                .map(|w| {
                    w.trim_matches(|c: char| !c.is_alphanumeric() && c != '\'')
                        .to_lowercase()
                })
                .filter(|w| !w.is_empty())
                .collect();

            for w in &words {
                *corpus_word_counts.entry(w.clone()).or_insert(0) += 1;
            }
            char_words.entry(char_name.clone()).or_default().extend(words);
        }

        let mut profiles = Vec::new();
        for (char_name, words) in &char_words {
            let total_words = words.len();
            if total_words == 0 {
                continue;
            }

            let turns_count = *char_turn_counts.get(char_name).unwrap_or(&1);
            let mean_turn_length = total_words as f64 / turns_count as f64;

            // Unique words (Type-Token Ratio)
            let unique_words: HashSet<String> = words.iter().cloned().collect();
            let ttr = unique_words.len() as f64 / total_words as f64;

            // Contraction ratio
            let contraction_count = words
                .iter()
                .filter(|w| Self::CONTRACTIONS.contains(&w.as_str()))
                .count();
            let contraction_ratio = contraction_count as f64 / total_words as f64;

            // Sentence lengths (approximate based on punctuation from raw turns)
            let mut sentence_lengths = Vec::new();
            for (c, raw_turn) in turns {
                if c == char_name {
                    for sent in raw_turn.split(|c| c == '.' || c == '!' || c == '?') {
                        let count = sent.split_whitespace().count();
                        if count > 0 {
                            sentence_lengths.push(count);
                        }
                    }
                }
            }

            let (mean_sentence_length, cadence_variance) = if !sentence_lengths.is_empty() {
                let mean = sentence_lengths.iter().sum::<usize>() as f64 / sentence_lengths.len() as f64;
                let var = sentence_lengths
                    .iter()
                    .map(|len| (*len as f64 - mean).powi(2))
                    .sum::<f64>()
                    / sentence_lengths.len() as f64;
                (mean, var.sqrt())
            } else {
                (10.0, 3.0)
            };

            // Distinctive terms: terms used heavily by this character compared to the rest of the corpus
            let mut distinctive = Vec::new();
            let mut char_freq: HashMap<&str, usize> = HashMap::new();
            for w in words {
                *char_freq.entry(w.as_str()).or_insert(0) += 1;
            }

            for (w, count) in char_freq {
                if count >= 2 && w.len() >= 4 {
                    let total_in_corpus = *corpus_word_counts.get(w).unwrap_or(&count);
                    let ratio = count as f64 / total_in_corpus as f64;
                    if ratio >= 0.7 {
                        distinctive.push(w.to_string());
                    }
                }
            }
            distinctive.sort();
            distinctive.truncate(5);

            profiles.push(CharacterVoiceProfile {
                character: char_name.clone(),
                total_words,
                speaking_turns: turns_count,
                mean_turn_length_words: (mean_turn_length * 10.0).round() / 10.0,
                type_token_ratio: (ttr * 1000.0).round() / 1000.0,
                contraction_ratio: (contraction_ratio * 1000.0).round() / 1000.0,
                mean_sentence_length: (mean_sentence_length * 10.0).round() / 10.0,
                cadence_variance: (cadence_variance * 10.0).round() / 10.0,
                distinctive_terms: distinctive,
            });
        }

        profiles.sort_by(|a, b| b.total_words.cmp(&a.total_words));

        // Compute voice divergence: variance in mean turn length and contraction ratio among top characters
        let divergence = if profiles.len() >= 2 {
            let top2 = &profiles[..2];
            let diff_contractions = (top2[0].contraction_ratio - top2[1].contraction_ratio).abs() * 3.0;
            let diff_cadence = (top2[0].mean_turn_length_words - top2[1].mean_turn_length_words).abs() / 20.0;
            (diff_contractions + diff_cadence).min(1.0)
        } else {
            0.5
        };

        IdiolectAuditReport {
            character_profiles: profiles,
            overall_voice_divergence: (divergence * 100.0).round() / 100.0,
            head_hopping_alerts: Vec::new(),
        }
    }

    /// Scans markdown narrative prose for third-person limited POV head-hopping
    pub fn scan_pov_head_hopping(text: &str, declared_pov: &str, known_characters: &[&str]) -> Vec<HeadHoppingIncident> {
        let mut alerts = Vec::new();
        let normalized_pov = declared_pov.to_lowercase();
        let mut last_non_pov_char: Option<String> = None;

        for (idx, line) in text.lines().enumerate() {
            let lower = line.to_lowercase();
            // Skip lines that are dialogue quotes
            if line.trim().starts_with('"') || line.trim().starts_with('“') {
                continue;
            }

            // Update last_non_pov_char if any known other character is mentioned
            for other_char in known_characters {
                let other_lower = other_char.to_lowercase();
                if other_lower == normalized_pov {
                    continue;
                }
                let first_name = other_lower.split_whitespace().next().unwrap_or(&other_lower);
                if lower.contains(&other_lower) || lower.split_whitespace().any(|w| w.trim_matches(|c: char| !c.is_alphanumeric()) == first_name) {
                    last_non_pov_char = Some(other_char.to_string());
                    break;
                }
            }

            // Check if other character is performing an internal cognitive verb
            for other_char in known_characters {
                let other_lower = other_char.to_lowercase();
                if other_lower == normalized_pov {
                    continue;
                }
                let first_name = other_lower.split_whitespace().next().unwrap_or(&other_lower);
                let char_in_line = lower.contains(&other_lower)
                    || lower.split_whitespace().any(|w| w.trim_matches(|c: char| !c.is_alphanumeric()) == first_name);

                for verb in Self::COGNITIVE_VERBS {
                    if lower.contains(verb) {
                        if char_in_line {
                            alerts.push(HeadHoppingIncident {
                                line_number: idx + 1,
                                declared_pov: declared_pov.to_string(),
                                intruding_character: other_char.to_string(),
                                cognitive_verb: verb.to_string(),
                                context_snippet: line.trim().to_string(),
                            });
                            break;
                        } else if let Some(ref last_char) = last_non_pov_char {
                            if last_char == *other_char {
                                let has_pronoun = lower.split_whitespace().any(|w| {
                                    let clean = w.trim_matches(|c: char| !c.is_alphanumeric());
                                    clean == "he" || clean == "she" || clean == "they" || clean == "his" || clean == "her"
                                });
                                if has_pronoun {
                                    alerts.push(HeadHoppingIncident {
                                        line_number: idx + 1,
                                        declared_pov: declared_pov.to_string(),
                                        intruding_character: other_char.to_string(),
                                        cognitive_verb: verb.to_string(),
                                        context_snippet: line.trim().to_string(),
                                    });
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }

        alerts
    }
}
