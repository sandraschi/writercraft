//! Epistemic Knowledge & Secret Matrix Engine ("Who Knows What When")
//!
//! Tracks facts, secrets, dramatic irony, character knowledge states,
//! and timeline reconciliation (Fabula vs. Syuzhet) to detect epistemic plot holes.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum KnowledgeState {
    Ignorant,
    Suspected,
    Known,
    Misinformed { belief: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Fact {
    pub fact_id: String,
    pub summary: String,
    pub is_secret: bool,
    pub origin_scene_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SceneEpistemicEvent {
    pub scene_id: String,
    pub syuzhet_order: usize, // Narrative presentation order (Chapter 1, 2, ...)
    pub fabula_time: i64,     // Chronological timeline timestamp (hours/days relative to zero)
    pub pov_character: String,
    pub present_characters: Vec<String>,
    pub revealed_facts: Vec<String>,
    pub planted_lies: Vec<(String, String)>, // (fact_id, false_belief)
    pub action_dependencies: Vec<String>,    // facts required for characters to logically take action in this scene
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AnomalySeverity {
    Warning,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EpistemicAnomaly {
    pub severity: AnomalySeverity,
    pub scene_id: String,
    pub character: String,
    pub fact_id: String,
    pub issue_type: String,
    pub explanation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineBeat {
    pub scene_id: String,
    pub narrative_index: usize,
    pub chronological_order: usize,
    pub pov: String,
    pub is_flashback: bool,
    pub facts_disclosed_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EpistemicGraph {
    pub facts: HashMap<String, Fact>,
    pub scenes: Vec<SceneEpistemicEvent>,
}

impl EpistemicGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_fact(&mut self, fact_id: &str, summary: &str, is_secret: bool, origin_scene_id: &str) {
        self.facts.insert(
            fact_id.to_string(),
            Fact {
                fact_id: fact_id.to_string(),
                summary: summary.to_string(),
                is_secret,
                origin_scene_id: origin_scene_id.to_string(),
            },
        );
    }

    pub fn add_scene_event(&mut self, event: SceneEpistemicEvent) {
        self.scenes.push(event);
        self.scenes.sort_by_key(|s| s.syuzhet_order);
    }

    /// Evaluates the sequence of scenes step-by-step to compute character knowledge
    /// and flags epistemic anomalies (plot holes, premature knowledge, unearned actions).
    pub fn validate(&self) -> Vec<EpistemicAnomaly> {
        let mut anomalies = Vec::new();
        // character_name -> (fact_id -> KnowledgeState)
        let mut knowledge_map: HashMap<String, HashMap<String, KnowledgeState>> = HashMap::new();

        let mut prev_fabula_time: Option<i64> = None;

        for scene in &self.scenes {
            // Check chronological flashback consistency
            if let Some(prev_time) = prev_fabula_time {
                if scene.fabula_time < prev_time {
                    // Flashback detected: verify it doesn't accidentally inherit future knowledge
                }
            }
            prev_fabula_time = Some(scene.fabula_time);

            // 1. Check action dependencies: did the POV or present characters act on facts they didn't know?
            for req_fact in &scene.action_dependencies {
                let char_knowledge = knowledge_map
                    .entry(scene.pov_character.clone())
                    .or_default()
                    .get(req_fact);

                match char_knowledge {
                    Some(KnowledgeState::Known) => {}
                    Some(KnowledgeState::Suspected) => {
                        anomalies.push(EpistemicAnomaly {
                            severity: AnomalySeverity::Warning,
                            scene_id: scene.scene_id.clone(),
                            character: scene.pov_character.clone(),
                            fact_id: req_fact.clone(),
                            issue_type: "ActionOnUnconfirmedSuspicion".into(),
                            explanation: format!(
                                "Character '{}' acts decisively in scene '{}' on fact '{}', but only suspected it without proof.",
                                scene.pov_character, scene.scene_id, req_fact
                            ),
                        });
                    }
                    Some(KnowledgeState::Misinformed { belief }) => {
                        anomalies.push(EpistemicAnomaly {
                            severity: AnomalySeverity::Critical,
                            scene_id: scene.scene_id.clone(),
                            character: scene.pov_character.clone(),
                            fact_id: req_fact.clone(),
                            issue_type: "ContradictoryBeliefAction".into(),
                            explanation: format!(
                                "Character '{}' acts as if '{}' is true in '{}', but their current state is Misinformed with belief: '{}'",
                                scene.pov_character, req_fact, scene.scene_id, belief
                            ),
                        });
                    }
                    None | Some(KnowledgeState::Ignorant) => {
                        anomalies.push(EpistemicAnomaly {
                            severity: AnomalySeverity::Critical,
                            scene_id: scene.scene_id.clone(),
                            character: scene.pov_character.clone(),
                            fact_id: req_fact.clone(),
                            issue_type: "PrematureKnowledgeLeak".into(),
                            explanation: format!(
                                "Plot hole: Character '{}' acts upon fact '{}' in scene '{}' before it was ever revealed to them.",
                                scene.pov_character, req_fact, scene.scene_id
                            ),
                        });
                    }
                }
            }

            // 2. Process planted lies
            for (fact_id, lie) in &scene.planted_lies {
                for attendee in &scene.present_characters {
                    let map = knowledge_map.entry(attendee.clone()).or_default();
                    map.insert(fact_id.clone(), KnowledgeState::Misinformed { belief: lie.clone() });
                }
            }

            // 3. Process revealed facts
            for revealed in &scene.revealed_facts {
                if !self.facts.contains_key(revealed) {
                    anomalies.push(EpistemicAnomaly {
                        severity: AnomalySeverity::Warning,
                        scene_id: scene.scene_id.clone(),
                        character: scene.pov_character.clone(),
                        fact_id: revealed.clone(),
                        issue_type: "UnregisteredFact".into(),
                        explanation: format!("Scene '{}' references unregistered fact '{}'.", scene.scene_id, revealed),
                    });
                }

                // All present characters now know this fact
                for attendee in &scene.present_characters {
                    let map = knowledge_map.entry(attendee.clone()).or_default();
                    map.insert(revealed.clone(), KnowledgeState::Known);
                }
            }
        }

        anomalies
    }

    /// Computes the Fabula vs. Syuzhet timeline mapping
    pub fn build_timeline(&self) -> Vec<TimelineBeat> {
        let mut sorted_by_fabula = self.scenes.clone();
        sorted_by_fabula.sort_by_key(|s| s.fabula_time);

        let mut chronological_ranks: HashMap<String, usize> = HashMap::new();
        for (rank, scene) in sorted_by_fabula.iter().enumerate() {
            chronological_ranks.insert(scene.scene_id.clone(), rank + 1);
        }

        let mut prev_fabula = i64::MIN;
        self.scenes
            .iter()
            .enumerate()
            .map(|(idx, s)| {
                let is_flashback = s.fabula_time < prev_fabula;
                prev_fabula = s.fabula_time;
                TimelineBeat {
                    scene_id: s.scene_id.clone(),
                    narrative_index: idx + 1,
                    chronological_order: *chronological_ranks.get(&s.scene_id).unwrap_or(&(idx + 1)),
                    pov: s.pov_character.clone(),
                    is_flashback,
                    facts_disclosed_count: s.revealed_facts.len(),
                }
            })
            .collect()
    }

    /// Returns a complete matrix snapshot: character -> fact -> KnowledgeState
    pub fn snapshot_knowledge(&self) -> HashMap<String, HashMap<String, KnowledgeState>> {
        let mut matrix: HashMap<String, HashMap<String, KnowledgeState>> = HashMap::new();
        for scene in &self.scenes {
            for (fact_id, lie) in &scene.planted_lies {
                for char_name in &scene.present_characters {
                    matrix.entry(char_name.clone()).or_default().insert(
                        fact_id.clone(),
                        KnowledgeState::Misinformed { belief: lie.clone() },
                    );
                }
            }
            for fact_id in &scene.revealed_facts {
                for char_name in &scene.present_characters {
                    matrix
                        .entry(char_name.clone())
                        .or_default()
                        .insert(fact_id.clone(), KnowledgeState::Known);
                }
            }
        }
        matrix
    }

    /// Provides a curated demo graph (e.g. for Fleet / Sci-Fi Thriller plot)
    pub fn mock_scifi_mystery() -> Self {
        let mut graph = Self::new();

        graph.register_fact(
            "FACT-01-BREACH",
            "Autonomous agent compromised Subterranean Substation 4",
            false,
            "SCENE-01",
        );
        graph.register_fact(
            "FACT-02-MOLE",
            "Director Thorne signed the firmware override key",
            true,
            "SCENE-03",
        );
        graph.register_fact(
            "FACT-03-NANITES",
            "The cooling towers are synthesizing self-replicating logic gates",
            true,
            "SCENE-04",
        );

        graph.add_scene_event(SceneEpistemicEvent {
            scene_id: "SCENE-01".into(),
            syuzhet_order: 1,
            fabula_time: 100,
            pov_character: "Dr. Elena Vance".into(),
            present_characters: vec!["Dr. Elena Vance".into(), "Marcus Thorne".into()],
            revealed_facts: vec!["FACT-01-BREACH".into()],
            planted_lies: vec![],
            action_dependencies: vec![],
        });

        graph.add_scene_event(SceneEpistemicEvent {
            scene_id: "SCENE-02".into(),
            syuzhet_order: 2,
            fabula_time: 104,
            pov_character: "Dr. Elena Vance".into(),
            present_characters: vec!["Dr. Elena Vance".into()],
            revealed_facts: vec![],
            planted_lies: vec![],
            action_dependencies: vec!["FACT-01-BREACH".into()],
        });

        graph.add_scene_event(SceneEpistemicEvent {
            scene_id: "SCENE-03".into(),
            syuzhet_order: 3,
            fabula_time: 80, // Flashback to 20 hours prior
            pov_character: "Marcus Thorne".into(),
            present_characters: vec!["Marcus Thorne".into()],
            revealed_facts: vec!["FACT-02-MOLE".into()],
            planted_lies: vec![],
            action_dependencies: vec![],
        });

        graph
    }
}
