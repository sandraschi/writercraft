//! WriterCraft Narrative & Epistemic Graph Engine
//!
//! Provides corkboard scene cards, TV Tropes indexation,
//! academic citation graphs, and narrative tension tracking.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneCard {
    pub scene_id: String,
    pub act_number: u32,
    pub title: String,
    pub pov_character: String,
    pub dramatic_goal: String,
    pub conflict_obstacle: String,
    pub outcome_disaster: String,
    pub emotional_valence: f64, // -1.0 (tragedy/despair) to +1.0 (triumph/joy)
    pub associated_tropes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpistemicReference {
    pub citation_key: String,
    pub title: String,
    pub author_or_source: String,
    pub confidence_score: f64,
    pub dialectic_status: String, // "PROVEN", "HYPOTHESIS", "REFUTED"
}

pub struct NarrativeModel;

impl NarrativeModel {
    pub fn mock_act_one_scenes() -> Vec<SceneCard> {
        vec![
            SceneCard {
                scene_id: "SCENE-01".to_string(),
                act_number: 1,
                title: "The Static in the Wire".to_string(),
                pov_character: "Dr. Elena Vance".to_string(),
                dramatic_goal: "Calibrate the deep-crust radio observatory".to_string(),
                conflict_obstacle: "Unexpected non-random microwave harmonics disrupt baseline".to_string(),
                outcome_disaster: "Signal locks onto local power grid; blackouts ripple across the valley".to_string(),
                emotional_valence: -0.4,
                associated_tropes: vec!["Call to Adventure".to_string(), "Ghost in the Machine".to_string()],
            },
            SceneCard {
                scene_id: "SCENE-02".to_string(),
                act_number: 1,
                title: "The Emergency Directorate".to_string(),
                pov_character: "Director Marcus Thorne".to_string(),
                dramatic_goal: "Suppress panic and isolate the subterranean observatory".to_string(),
                conflict_obstacle: "Elena refuses to shut down the buffer without archiving telemetry".to_string(),
                outcome_disaster: "Elena flees into the salt tunnels with the sovereign data drive".to_string(),
                emotional_valence: -0.7,
                associated_tropes: vec!["Point of No Return".to_string(), "The Cassandra Truth".to_string()],
            },
        ]
    }
}
