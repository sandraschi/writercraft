//! Fountain Screenplay Parser for WriterCraft
//!
//! Parses plain-text Fountain screenplays (.fountain) into structured scenes,
//! character dialogue statistics, and page-count / run-time estimates.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScreenplayScene {
    pub scene_number: usize,
    pub heading: String,
    pub action_lines: Vec<String>,
    pub dialogues: Vec<ScreenplayDialogue>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScreenplayDialogue {
    pub character: String,
    pub parenthetical: Option<String>,
    pub lines: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CharacterDialogueStat {
    pub character: String,
    pub speaking_turns: usize,
    pub word_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScreenplayAnalysis {
    pub title: Option<String>,
    pub total_scenes: usize,
    pub total_words: usize,
    pub estimated_pages: f64, // ~250 words per standard screenplay page / 1 min per page
    pub characters: Vec<CharacterDialogueStat>,
    pub scenes: Vec<ScreenplayScene>,
}

pub struct FountainParser;

impl FountainParser {
    pub fn parse(text: &str) -> ScreenplayAnalysis {
        let lines: Vec<&str> = text.lines().collect();
        let mut title: Option<String> = None;
        let mut scenes: Vec<ScreenplayScene> = Vec::new();
        let mut char_map: HashMap<String, (usize, usize)> = HashMap::new(); // char -> (turns, words)

        let mut current_scene: Option<ScreenplayScene> = None;
        let mut current_character: Option<String> = None;
        let mut current_parenthetical: Option<String> = None;
        let mut current_dialogue_lines: Vec<String> = Vec::new();
        let mut total_words = 0;

        let is_scene_heading = |line: &str| -> bool {
            let upper = line.trim().to_uppercase();
            upper.starts_with("INT.")
                || upper.starts_with("EXT.")
                || upper.starts_with("INT/EXT.")
                || upper.starts_with("I/E.")
        };

        let is_character_cue = |line: &str| -> bool {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.len() > 30 {
                return false;
            }
            // All uppercase, letters/spaces/parentheses like "ELENA (V.O.)"
            let without_parens = trimmed.split('(').next().unwrap_or("").trim();
            !without_parens.is_empty()
                && without_parens.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_whitespace() || c == '.' || c == '\'')
        };

        let commit_dialogue = |curr_char: &mut Option<String>,
                               curr_paren: &mut Option<String>,
                               curr_lines: &mut Vec<String>,
                               scene: &mut ScreenplayScene,
                               char_map: &mut HashMap<String, (usize, usize)>| {
            if let Some(char_name) = curr_char.take() {
                if !curr_lines.is_empty() {
                    let d_words: usize = curr_lines.iter().map(|l| l.split_whitespace().count()).sum();
                    let stat = char_map.entry(char_name.clone()).or_insert((0, 0));
                    stat.0 += 1;
                    stat.1 += d_words;

                    scene.dialogues.push(ScreenplayDialogue {
                        character: char_name,
                        parenthetical: curr_paren.take(),
                        lines: std::mem::take(curr_lines),
                    });
                }
            }
        };

        for raw_line in lines {
            let trimmed = raw_line.trim();
            if trimmed.is_empty() {
                if let Some(ref mut scene) = current_scene {
                    commit_dialogue(
                        &mut current_character,
                        &mut current_parenthetical,
                        &mut current_dialogue_lines,
                        scene,
                        &mut char_map,
                    );
                }
                continue;
            }

            // Title page metadata detection (e.g. "Title: ...", "Author: ...", "Draft date: ...")
            if current_scene.is_none() && trimmed.contains(':') && !is_scene_heading(trimmed) {
                let lower = trimmed.to_lowercase();
                if lower.starts_with("title:") && title.is_none() {
                    title = Some(trimmed["title:".len()..].trim().to_string());
                }
                // Ignore other title page metadata lines like Author, Credit, Draft date before scene 1
                continue;
            }

            let word_count = trimmed.split_whitespace().count();
            total_words += word_count;

            if is_scene_heading(trimmed) {
                // Finish current scene if open
                if let Some(mut scene) = current_scene.take() {
                    commit_dialogue(
                        &mut current_character,
                        &mut current_parenthetical,
                        &mut current_dialogue_lines,
                        &mut scene,
                        &mut char_map,
                    );
                    scenes.push(scene);
                }

                current_scene = Some(ScreenplayScene {
                    scene_number: scenes.len() + 1,
                    heading: trimmed.to_string(),
                    action_lines: Vec::new(),
                    dialogues: Vec::new(),
                });
                continue;
            }

            if is_character_cue(trimmed) {
                if let Some(ref mut scene) = current_scene {
                    commit_dialogue(
                        &mut current_character,
                        &mut current_parenthetical,
                        &mut current_dialogue_lines,
                        scene,
                        &mut char_map,
                    );
                }
                let clean_name = trimmed.split('(').next().unwrap_or(trimmed).trim().to_string();
                current_character = Some(clean_name);
                if trimmed.contains('(') && trimmed.contains(')') {
                    if let Some(start) = trimmed.find('(') {
                        if let Some(end) = trimmed.find(')') {
                            current_parenthetical = Some(trimmed[start + 1..end].to_string());
                        }
                    }
                }
                continue;
            }

            if current_character.is_some() {
                if trimmed.starts_with('(') && trimmed.ends_with(')') {
                    current_parenthetical = Some(trimmed[1..trimmed.len() - 1].to_string());
                } else {
                    current_dialogue_lines.push(trimmed.to_string());
                }
                continue;
            }

            // Action line
            if let Some(ref mut scene) = current_scene {
                scene.action_lines.push(trimmed.to_string());
            } else {
                // If scene hasn't started yet, create Scene 1
                let scene = ScreenplayScene {
                    scene_number: 1,
                    heading: "COLD OPEN".to_string(),
                    action_lines: vec![trimmed.to_string()],
                    dialogues: Vec::new(),
                };
                current_scene = Some(scene);
            }
        }

        // Flush remaining
        if let Some(mut scene) = current_scene.take() {
            commit_dialogue(
                &mut current_character,
                &mut current_parenthetical,
                &mut current_dialogue_lines,
                &mut scene,
                &mut char_map,
            );
            scenes.push(scene);
        }

        let mut character_stats: Vec<CharacterDialogueStat> = char_map
            .into_iter()
            .map(|(character, (speaking_turns, word_count))| CharacterDialogueStat {
                character,
                speaking_turns,
                word_count,
            })
            .collect();

        // Sort characters by word count descending
        character_stats.sort_by(|a, b| b.word_count.cmp(&a.word_count));

        let estimated_pages = (total_words as f64 / 220.0).max(1.0);

        ScreenplayAnalysis {
            title,
            total_scenes: scenes.len(),
            total_words,
            estimated_pages,
            characters: character_stats,
            scenes,
        }
    }
}
