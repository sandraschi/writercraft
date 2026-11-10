//! WriterCraft Literary Delectations & Craft Library
//!
//! Provides in-app access to deep essays, rhetorical treatises, trope catalogs,
//! and publishing economics for the writer and reader's delectation.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CraftEssayInfo {
    pub slug: String,
    pub title: String,
    pub subtitle: String,
    pub category: String,
    pub reading_time_mins: usize,
    pub word_count: usize,
    pub abstract_text: String,
}

#[derive(Debug, Clone, Copy)]
pub struct CraftEssay {
    pub slug: &'static str,
    pub title: &'static str,
    pub subtitle: &'static str,
    pub category: &'static str,
    pub reading_time_mins: usize,
    pub word_count: usize,
    pub abstract_text: &'static str,
    pub content_markdown: &'static str,
}

impl CraftEssay {
    pub fn to_info(&self) -> CraftEssayInfo {
        CraftEssayInfo {
            slug: self.slug.to_string(),
            title: self.title.to_string(),
            subtitle: self.subtitle.to_string(),
            category: self.category.to_string(),
            reading_time_mins: self.reading_time_mins,
            word_count: self.word_count,
            abstract_text: self.abstract_text.to_string(),
        }
    }
}

pub static ESSAY_DIALECTIC: &str = include_str!("../../../docs/WRITER_AND_AI_DIALECTIC.md");
pub static ESSAY_CANON: &str = include_str!("../../../docs/THE_CLASSICAL_CANON.md");
pub static ESSAY_TROPES: &str = include_str!("../../../docs/TROPE_SMORGASBORD.md");
pub static ESSAY_PUBLISHING: &str = include_str!("../../../docs/PUBLISHING_AVENUES.md");
pub static ESSAY_FRONTIER: &str = include_str!("../../../docs/THE_SPECULATIVE_FRONTIER.md");

pub fn all_essays() -> Vec<CraftEssay> {
    vec![
        CraftEssay {
            slug: "dialectic",
            title: "The Writer & His AI Companion",
            subtitle: "Dialectic, Rhetoric, and the Anti-Claude-ese Cribsheet",
            category: "Dialectic & AI Craft",
            reading_time_mins: 8,
            word_count: count_words(ESSAY_DIALECTIC),
            abstract_text: "Explores the asymmetric dialectic between writer and machine, cataloging 35+ Claude-ese tells ('loadbearing', 'delve', 'tapestry'), classical rhetoric subversion, and a 20-antipattern human correction matrix.",
            content_markdown: ESSAY_DIALECTIC,
        },
        CraftEssay {
            slug: "canon",
            title: "What Makes Classics Classics?",
            subtitle: "Dickens, Dostoevsky, and the Brontës: Polyphony, Guilt, and Civilizational Seismology",
            category: "Literary Canon",
            reading_time_mins: 9,
            word_count: count_words(ESSAY_CANON),
            abstract_text: "Examines why Dickens, Dostoevsky, and the Brontës survived the meat-grinder of history while peers faded: Bakhtinian polyphony, grotesque particularity, unpayable moral debts, psychobabble with iron discipline, and historical cataclysms.",
            content_markdown: ESSAY_CANON,
        },
        CraftEssay {
            slug: "frontier",
            title: "The Speculative Frontier & The RSI Novel",
            subtitle: "Hard Sci-Fi Classics (Gibson, Stephenson, Egan), Banksian Minds, and the Post-AI Hiatus",
            category: "Speculative Canon & RSI",
            reading_time_mins: 12,
            word_count: count_words(ESSAY_FRONTIER),
            abstract_text: "Analyzes the sci-fi hiatus as writers digest the AI tsunami, the exhaustion of post-AI dystopia, how Iain M. Banks bypassed the superior-mind paradox in The Culture, and what the Great LLM and Recursive Self-Improvement novel requires.",
            content_markdown: ESSAY_FRONTIER,
        },
        CraftEssay {
            slug: "tropes",
            title: "The Cliché Motherlode & Trope Smorgasbord",
            subtitle: "Romance, Sci-Fi, Thrillers, and the Frankenstein Smorgasbord Scaffold",
            category: "Trope Mechanics & Satire",
            reading_time_mins: 10,
            word_count: count_words(ESSAY_TROPES),
            abstract_text: "A tongue-in-cheek anatomy of commercial tropes across Romance, Sci-Fi, Action/Thrillers, and Prestige Gloom, featuring the Frankenstein Smorgasbord Scaffold and the Golden Ratio of Fiction.",
            content_markdown: ESSAY_TROPES,
        },
        CraftEssay {
            slug: "publishing",
            title: "Publishing Avenues & The Cautionary Tale",
            subtitle: "The Coral Hart Paradox: 370 Books in a Year vs. Durable Backlist Craft",
            category: "Publishing Economics",
            reading_time_mins: 11,
            word_count: count_words(ESSAY_PUBLISHING),
            abstract_text: "Compares 4 modern publishing models (KDP Select, Wide/IngramSpark, Serial/Ream, Academic/Leanpub), dissecting the South African 370-book-a-year case study as an abschreckendes Beispiel against durable backlist architecture.",
            content_markdown: ESSAY_PUBLISHING,
        },
    ]
}

pub fn get_essay(slug: &str) -> Option<CraftEssay> {
    all_essays().into_iter().find(|e| {
        e.slug.eq_ignore_ascii_case(slug) || (e.slug == "frontier" && slug.eq_ignore_ascii_case("scifi"))
    })
}

pub fn list_essay_infos() -> Vec<CraftEssayInfo> {
    all_essays().into_iter().map(|e| e.to_info()).collect()
}

fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}
