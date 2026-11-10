//! Style Guide, Anti-Claude-ese Linter & Rhetorical Cribsheet
//!
//! Provides deterministic linting against AI-writer tells, pseudo-erudition,
//! reflexive triads, contrast frames, monotonic rhythm, and moralizing conclusions.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ViolationKind {
    Claudeism,
    StockWord,
    ReflexiveTriad,
    ContrastFrame,
    MonotonicRhythm,
    MoralizingWrapup,
    HedgingPileup,
    SensoryCliche,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StyleViolation {
    pub kind: ViolationKind,
    pub line_number: usize,
    pub matched_text: String,
    pub explanation: String,
    pub remedy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StyleAudit {
    pub total_violations: usize,
    pub claude_tells_count: usize,
    pub triads_count: usize,
    pub contrast_frames_count: usize,
    pub cadence_score: f64, // Coefficient of variation of sentence lengths
    pub rhetorical_grade: String,
    pub violations: Vec<StyleViolation>,
    pub cribsheet_recommendations: Vec<String>,
}

pub struct StyleLinter;

impl StyleLinter {
    /// Comprehensive catalog of Claude-ese and AI favorite words
    pub const CLAUDE_TELLS: &'static [(&'static str, &'static str)] = &[
        ("loadbearing", "Anthropic favorite. Overused metaphor for structural criticality. Replace with 'structural', 'primary', or show the exact dependency."),
        ("load-bearing", "Anthropic favorite. Overused metaphor for structural criticality. Replace with 'structural', 'primary', or show the exact dependency."),
        ("delve", "Classic generative AI dead giveaway. Replace with 'examine', 'probe', 'inspect', or cut the verb entirely."),
        ("delving", "Classic generative AI dead giveaway. Cut or replace with concrete inspection verb."),
        ("tapestry", "AI crutch word for complex systems. State the actual components rather than calling it a tapestry."),
        ("testament to", "Moralizing cliché. Demonstrate the evidence instead of declaring it a testament."),
        ("a testament", "Moralizing cliché. Demonstrate the evidence directly."),
        ("nuanced", "Overused hedging adjective. Provide the specific distinction instead of praising its nuance."),
        ("multifaceted", "Euphemism for complicated. Name the specific facets."),
        ("interplay", "Vague substitute for direct causality. Describe who acts on whom."),
        ("dance of", "Romanticized abstraction for mechanics or negotiation. Cut."),
        ("symphony", "Trite metaphor for harmonious action. Cut."),
        ("pivotal", "Lazy inflation of importance. Show why the decision turned."),
        ("crucial", "Overused adjective. Demonstrate stakes with consequences."),
        ("beacon", "Sentimental cliché. Name the real guide or standard."),
        ("game-changer", "Marketing hype. Explain the operational cost collapse instead."),
        ("paradigm shift", "Corrupted academic jargon. Explain the concrete technical change."),
        ("ever-evolving", "Filler adjective. Everything in tech changes; specify the rate or direction."),
        ("fast-paced world", "Dead boardroom cliché. Delete."),
        ("in today's", "Empty opener. Cut straight to the subject."),
        ("in essence", "Throat-clearing transition. Delete; write the essence directly."),
        ("at the end of the day", "Colloquial padding. Delete."),
        ("make no mistake", "Pedantic scolding of the reader. State the fact."),
        ("let's be clear", "Condescending rhetorical crutch. State the assertion cleanly."),
        ("buckle up", "Cheap pseudo-conversational hype. Delete."),
        ("here's the thing", "Blogger colloquialism masquerading as insight. Delete."),
        ("resonate", "New-age substitute for persuade, remind, or align. Be precise."),
        ("holistic", "Corporate buzzword. Name the whole system."),
        ("synergy", "Corporate fluff. Explain the compound effect."),
        ("seamlessly", "Almost never true in software or human systems. Show the seam."),
        ("seamless", "False perfection adjective. Show the friction points."),
        ("meticulously", "Adverbial padding. Describe the care taken through concrete steps."),
        ("whispers of", "Fiction AI cliché for faint sounds or rumors. Name the whisperer."),
        ("shivers down", "Cheap romance/thriller somatic reaction. Show the physical shock or chill."),
        ("eyes darkened with", "Formulaic romance fiction tell. Describe facial action or speech."),
        ("breath he didn't realize", "Erotica/romance cliché. Cut."),
        ("breath she didn't realize", "Erotica/romance cliché. Cut."),
    ];

    pub const HEDGES: &'static [&'static str] = &[
        "it is important to note",
        "it's important to note",
        "it is worth noting",
        "worth noting that",
        "it could be argued",
        "arguably",
        "perhaps",
        "in a sense",
        "to some extent",
        "may potentially",
        "might possibly",
    ];

    pub fn lint(text: &str) -> StyleAudit {
        let mut violations = Vec::new();
        let mut claude_tells_count = 0;
        let mut triads_count = 0;
        let mut contrast_frames_count = 0;

        let lines: Vec<&str> = text.lines().collect();

        for (idx, line) in lines.iter().enumerate() {
            let line_num = idx + 1;
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with("```") {
                continue;
            }

            let lower = trimmed.to_lowercase();

            // 1. Claude-ese & Stock terms
            for (tell, remedy) in Self::CLAUDE_TELLS {
                let pattern = format!(" {} ", tell);
                let at_start = lower.starts_with(&format!("{} ", tell));
                let at_end = lower.ends_with(&format!(" {}", tell));
                let exact = lower == *tell;

                if lower.contains(&pattern) || at_start || at_end || exact {
                    claude_tells_count += 1;
                    violations.push(StyleViolation {
                        kind: ViolationKind::Claudeism,
                        line_number: line_num,
                        matched_text: tell.to_string(),
                        explanation: format!("AI tell: '{}'. High model frequency.", tell),
                        remedy: remedy.to_string(),
                    });
                }
            }

            // 2. Hedges & qualifier pile-ups
            for hedge in Self::HEDGES {
                if lower.contains(hedge) {
                    violations.push(StyleViolation {
                        kind: ViolationKind::HedgingPileup,
                        line_number: line_num,
                        matched_text: hedge.to_string(),
                        explanation: "Qualifier pile-up draining assertiveness.".to_string(),
                        remedy: "State the proposition unconditionally or delete the throat-clearing clause.".to_string(),
                    });
                }
            }

            // 3. Contrast frames: "not X, but Y" / "it's not just X"
            if (lower.contains("not just") || lower.contains("not merely") || lower.contains("it's not") || lower.contains("it is not"))
                && (lower.contains(", but") || lower.contains("; but") || lower.contains("but rather")) {
                contrast_frames_count += 1;
                violations.push(StyleViolation {
                    kind: ViolationKind::ContrastFrame,
                    line_number: line_num,
                    matched_text: "not X, but Y contrast frame".to_string(),
                    explanation: "Reflexive antithesis frame overused by LLMs to create artificial drama.".to_string(),
                    remedy: "State what Y is directly without first negating X.".to_string(),
                });
            }

            // 4. Reflexive triads: "A, B, and C"
            let words: Vec<&str> = trimmed.split_whitespace().collect();
            for w_idx in 0..words.len().saturating_sub(2) {
                if words[w_idx].ends_with(',') && words[w_idx + 1] == "and" {
                    triads_count += 1;
                    violations.push(StyleViolation {
                        kind: ViolationKind::ReflexiveTriad,
                        line_number: line_num,
                        matched_text: format!("{} {} {}", words[w_idx], words[w_idx + 1], words.get(w_idx + 2).unwrap_or(&"")),
                        explanation: "Tricolon fatigue: LLMs habitually group concepts into neat bundles of three.".to_string(),
                        remedy: "Use one sharp noun or break into asymmetric pairings.".to_string(),
                    });
                    break;
                }
            }

            // 5. Moralizing wrap-ups at end of paragraphs
            if (lower.starts_with("in conclusion") || lower.starts_with("ultimately,") || lower.starts_with("as we look to"))
                && line_num >= lines.len().saturating_sub(5) {
                violations.push(StyleViolation {
                    kind: ViolationKind::MoralizingWrapup,
                    line_number: line_num,
                    matched_text: trimmed.chars().take(40).collect::<String>(),
                    explanation: "Hallmark-card style moralizing conclusion.".to_string(),
                    remedy: "End on the final empirical fact or tactical consequence. Don't summarize.".to_string(),
                });
            }
        }

        // 6. Cadence & Rhythm variance calculation
        let sentences: Vec<&str> = text
            .split(|c| c == '.' || c == '!' || c == '?')
            .map(|s| s.trim())
            .filter(|s| s.split_whitespace().count() > 2)
            .collect();

        let sentence_lengths: Vec<f64> = sentences
            .iter()
            .map(|s| s.split_whitespace().count() as f64)
            .collect();

        let cadence_score = if sentence_lengths.len() > 3 {
            let mean = sentence_lengths.iter().sum::<f64>() / sentence_lengths.len() as f64;
            let variance = sentence_lengths.iter().map(|l| (l - mean).powi(2)).sum::<f64>() / sentence_lengths.len() as f64;
            let std_dev = variance.sqrt();
            (std_dev / mean.max(1.0)).clamp(0.0, 2.0)
        } else {
            0.8
        };

        let total_violations = violations.len();

        let rhetorical_grade = if total_violations == 0 && cadence_score > 0.45 {
            "A+ (Sharp, Idiosyncratic, Zero-Slop)".to_string()
        } else if total_violations <= 2 && cadence_score > 0.35 {
            "B (Readable & Disciplined)".to_string()
        } else if total_violations <= 5 {
            "C (Noticeable AI Cadence & Tells)".to_string()
        } else {
            "D (Severe Claude-ese / Generative Slop)".to_string()
        };

        let mut recommendations = Vec::new();
        if claude_tells_count > 0 {
            recommendations.push(format!("Purge {} instances of Claude-favourite crutches (e.g. 'loadbearing', 'tapestry', 'delve').", claude_tells_count));
        }
        if contrast_frames_count > 0 {
            recommendations.push("Subvert the 'not X, but Y' contrast frame: state positive claims without rehearsing the negative counterpart.".to_string());
        }
        if triads_count > 1 {
            recommendations.push("Break up reflexive triads: tricolon repetition creates a singsong academic drone.".to_string());
        }
        if cadence_score < 0.35 {
            recommendations.push("Vary sentence length: text currently exhibits machine cadence (monotonous sentence lengths). Introduce 3-word punchlines and 40-word periodic waves.".to_string());
        }
        if recommendations.is_empty() {
            recommendations.push("Maintain rigorous factual anchoring and preserve asymmetric paragraph rhythms.".to_string());
        }

        StyleAudit {
            total_violations,
            claude_tells_count,
            triads_count,
            contrast_frames_count,
            cadence_score,
            rhetorical_grade,
            violations,
            cribsheet_recommendations: recommendations,
        }
    }
}
