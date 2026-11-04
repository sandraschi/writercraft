//! WriterCraft Core Engine
//!
//! Provides corkboard scene graph management, narrative tension curve analysis,
//! and book manuscript project persistence backed by DatabaseCraft.

use serde::{Deserialize, Serialize};
use thiserror::Error;
use writercraft_narrative::{NarrativeModel, SceneCard};

#[derive(Error, Debug)]
pub enum WriterError {
    #[error("Manuscript error: {0}")]
    Manuscript(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
}
