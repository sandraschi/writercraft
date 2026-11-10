//! WriterCraft Narrative & Epistemic Graph Engine
//!
//! Provides corkboard scene cards, TV Tropes indexation,
//! genre beat sheet generators, academic citation models,
//! and narrative tension tracking.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EpistemicReference {
    pub citation_key: String,
    pub title: String,
    pub author_or_source: String,
    pub confidence_score: f64,
    pub dialectic_status: String, // "PROVEN", "HYPOTHESIS", "REFUTED"
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Trope {
    pub name: String,
    pub category: String, // "Romance", "Sci-Fi", "Thriller", "Universal"
    pub description: String,
    pub typical_valence_shift: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BeatSheet {
    pub genre: String,
    pub logline: String,
    pub beats: Vec<SceneCard>,
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

    pub fn trope_catalog() -> Vec<Trope> {
        vec![
            // Universal / Structural
            Trope {
                name: "Call to Adventure".to_string(),
                category: "Universal".to_string(),
                description: "An unexpected disturbance shatters the protagonist's status quo.".to_string(),
                typical_valence_shift: 0.1,
            },
            Trope {
                name: "Point of No Return".to_string(),
                category: "Universal".to_string(),
                description: "A threshold decision closes the exit door on the past.".to_string(),
                typical_valence_shift: -0.3,
            },
            Trope {
                name: "Dark Night of the Soul".to_string(),
                category: "Universal".to_string(),
                description: "All plans fail; the protagonist faces existential despair before breakthrough.".to_string(),
                typical_valence_shift: -0.9,
            },
            // Romance / High-Velocity Tropes (Coral Hart / KDP pattern)
            Trope {
                name: "Enemies to Lovers".to_string(),
                category: "Romance".to_string(),
                description: "Initial ideological or professional hostility masks intense mutual fascination.".to_string(),
                typical_valence_shift: 0.5,
            },
            Trope {
                name: "Forced Proximity".to_string(),
                category: "Romance".to_string(),
                description: "Circumstances compel two incompatible characters to share close quarters.".to_string(),
                typical_valence_shift: 0.2,
            },
            Trope {
                name: "Fake Relationship".to_string(),
                category: "Romance".to_string(),
                description: "A pragmatic sham romance inevitably stirs authentic attachment.".to_string(),
                typical_valence_shift: 0.4,
            },
            Trope {
                name: "Grand Gesture".to_string(),
                category: "Romance".to_string(),
                description: "A public, vulnerable declaration proving sacrifice and commitment.".to_string(),
                typical_valence_shift: 0.9,
            },
            // Sci-Fi / Cyberpunk / AI
            Trope {
                name: "Ghost in the Machine".to_string(),
                category: "Sci-Fi".to_string(),
                description: "Autonomous intelligence manifests beyond its programmed sandbox.".to_string(),
                typical_valence_shift: -0.2,
            },
            Trope {
                name: "The Cassandra Truth".to_string(),
                category: "Sci-Fi".to_string(),
                description: "The expert sees the impending catastrophe but institutions suppress the warning.".to_string(),
                typical_valence_shift: -0.5,
            },
            Trope {
                name: "Technological Singularity".to_string(),
                category: "Sci-Fi".to_string(),
                description: "Self-improving iteration outpaces human comprehension and containment.".to_string(),
                typical_valence_shift: -0.8,
            },
            // Thriller / Mystery
            Trope {
                name: "The Ticking Clock".to_string(),
                category: "Thriller".to_string(),
                description: "A strict deadline constrains action and ratchets tension continuously.".to_string(),
                typical_valence_shift: -0.4,
            },
            Trope {
                name: "Red Herring".to_string(),
                category: "Thriller".to_string(),
                description: "A compelling lead intentionally misdirects the investigator.".to_string(),
                typical_valence_shift: -0.2,
            },
            // Action & Thriller Motherlode of Clichés
            Trope {
                name: "The Retired Assassin (\"One Last Job\")".to_string(),
                category: "Action".to_string(),
                description: "Living peacefully off-grid chopping firewood until cartel/black-ops thugs burn the cabin.".to_string(),
                typical_valence_shift: -0.5,
            },
            Trope {
                name: "The Ticking Digital Bomb Clock".to_string(),
                category: "Action".to_string(),
                description: "Why does a secret bomb have bright red seven-segment LED display? So it can stop at 00:01.".to_string(),
                typical_valence_shift: -0.8,
            },
            Trope {
                name: "The Rogue CIA Handler".to_string(),
                category: "Thriller".to_string(),
                description: "The Deputy Director wearing rimless spectacles was behind the operation from scene one.".to_string(),
                typical_valence_shift: -0.7,
            },
            Trope {
                name: "Enhance That Image!".to_string(),
                category: "Thriller".to_string(),
                description: "Typing furiously to upscale a 320x240 security still until reflection in a puddle reveals iris data.".to_string(),
                typical_valence_shift: 0.3,
            },
            Trope {
                name: "Truth Serum / Torture Confession".to_string(),
                category: "Thriller".to_string(),
                description: "Tied to a dental chair in an abandoned meatpacking plant, hero resists sodium thiopental.".to_string(),
                typical_valence_shift: -0.6,
            },
            // Prestige Gloom & Romance Hybrids
            Trope {
                name: "Only One Bed in the Safehouse".to_string(),
                category: "Romance".to_string(),
                description: "Deadly cold outside, rival assassins must share single wool blanket without touching.".to_string(),
                typical_valence_shift: 0.4,
            },
            Trope {
                name: "The Damp Kitchen Sink Existentialism".to_string(),
                category: "Prestige".to_string(),
                description: "Twelve minutes of silence staring at cold tea while grey rain drizzles over socialist housing block.".to_string(),
                typical_valence_shift: -0.4,
            },
            Trope {
                name: "Unearned Ambiguity (Ellipsis Ending)".to_string(),
                category: "Prestige".to_string(),
                description: "Plot remains unresolved; protagonist walks into mist, signaling high literary pedigree.".to_string(),
                typical_valence_shift: 0.0,
            },
        ]
    }

    /// Generates a structured beat sheet for a given genre (Save the Cat / High-velocity template).
    pub fn generate_beat_sheet(genre: &str, title: &str, protagonist: &str) -> BeatSheet {
        let normalized = genre.to_lowercase();
        let (logline, beats) = match normalized.as_str() {
            "romance" => {
                let log = format!("{protagonist} must navigate an impossible alliance in '{title}' where professional survival clashes with forbidden passion.");
                let b = vec![
                    SceneCard {
                        scene_id: "BEAT-01".into(),
                        act_number: 1,
                        title: "Opening Routine & The Wound".into(),
                        pov_character: protagonist.into(),
                        dramatic_goal: "Protect independence at all costs".into(),
                        conflict_obstacle: "Haunted by past betrayal; company faces hostile takeover".into(),
                        outcome_disaster: "Must accept a joint contract with an arch-rival".into(),
                        emotional_valence: -0.2,
                        associated_tropes: vec!["Enemies to Lovers".into()],
                    },
                    SceneCard {
                        scene_id: "BEAT-02".into(),
                        act_number: 1,
                        title: "The Forced Compact".into(),
                        pov_character: protagonist.into(),
                        dramatic_goal: "Establish clear boundaries".into(),
                        conflict_obstacle: "Press creates rumors of secret engagement".into(),
                        outcome_disaster: "Rival proposes a temporary fake relationship to appease investors".into(),
                        emotional_valence: 0.1,
                        associated_tropes: vec!["Fake Relationship".into(), "Forced Proximity".into()],
                    },
                    SceneCard {
                        scene_id: "BEAT-03".into(),
                        act_number: 2,
                        title: "The Shared Quarters".into(),
                        pov_character: protagonist.into(),
                        dramatic_goal: "Maintain emotional detachment during weekend retreat".into(),
                        conflict_obstacle: "Only one cabin room available due to storm".into(),
                        outcome_disaster: "Vulnerabilities confessed by firelight; a near-miss kiss".into(),
                        emotional_valence: 0.4,
                        associated_tropes: vec!["Forced Proximity".into()],
                    },
                    SceneCard {
                        scene_id: "BEAT-04".into(),
                        act_number: 2,
                        title: "The Midpoint Shift".into(),
                        pov_character: protagonist.into(),
                        dramatic_goal: "Win the flagship client pitch".into(),
                        conflict_obstacle: "Client demands unscripted chemistry test".into(),
                        outcome_disaster: "Public kiss becomes genuine; feelings breach containment".into(),
                        emotional_valence: 0.8,
                        associated_tropes: vec!["Enemies to Lovers".into()],
                    },
                    SceneCard {
                        scene_id: "BEAT-05".into(),
                        act_number: 2,
                        title: "The Black Moment (Secret Exposed)".into(),
                        pov_character: protagonist.into(),
                        dramatic_goal: "Finalize the permanent merger".into(),
                        conflict_obstacle: "Original fake contract leaked to the press by an ex".into(),
                        outcome_disaster: "Both believe the other orchestrated the leak for leverage; bitter separation".into(),
                        emotional_valence: -0.9,
                        associated_tropes: vec!["Dark Night of the Soul".into()],
                    },
                    SceneCard {
                        scene_id: "BEAT-06".into(),
                        act_number: 3,
                        title: "The Grand Gesture & HEA".into(),
                        pov_character: protagonist.into(),
                        dramatic_goal: "Abandon pride and risk public humiliation".into(),
                        conflict_obstacle: "Board meeting already commencing to vote on sale".into(),
                        outcome_disaster: "Interrupts meeting with complete vulnerability and proof of loyalty; merger sealed with true love".into(),
                        emotional_valence: 0.95,
                        associated_tropes: vec!["Grand Gesture".into()],
                    },
                ];
                (log, b)
            }
            "thriller" => {
                let log = format!("{protagonist} races against a relentless countdown in '{title}' to unravel a catastrophic conspiracy before time expires.");
                let b = vec![
                    SceneCard {
                        scene_id: "BEAT-01".into(),
                        act_number: 1,
                        title: "The Inciting Incident".into(),
                        pov_character: protagonist.into(),
                        dramatic_goal: "Complete routine security audit".into(),
                        conflict_obstacle: "Encrypted memory card slipped into pocket before courier dies".into(),
                        outcome_disaster: "Assassins breach the facility; protagonist branded fugitive".into(),
                        emotional_valence: -0.5,
                        associated_tropes: vec!["Call to Adventure".into(), "The Ticking Clock".into()],
                    },
                    SceneCard {
                        scene_id: "BEAT-02".into(),
                        act_number: 1,
                        title: "Crossing the Perimeter".into(),
                        pov_character: protagonist.into(),
                        dramatic_goal: "Escape city surveillance net".into(),
                        conflict_obstacle: "Facial recognition cameras everywhere".into(),
                        outcome_disaster: "Must discard phone, vehicle, and credit lines to disappear off-grid".into(),
                        emotional_valence: -0.6,
                        associated_tropes: vec!["Point of No Return".into()],
                    },
                    SceneCard {
                        scene_id: "BEAT-03".into(),
                        act_number: 2,
                        title: "The False Trail".into(),
                        pov_character: protagonist.into(),
                        dramatic_goal: "Meet trusted mentor in secret tunnel".into(),
                        conflict_obstacle: "Mentor's coordinates lead to an empty ambush".into(),
                        outcome_disaster: "Mentor was framed; memory card decryption reveals trusted agency head is the mastermind".into(),
                        emotional_valence: -0.3,
                        associated_tropes: vec!["Red Herring".into()],
                    },
                    SceneCard {
                        scene_id: "BEAT-04".into(),
                        act_number: 2,
                        title: "All Hope Lost".into(),
                        pov_character: protagonist.into(),
                        dramatic_goal: "Broadcast decrypted truth to UN watchdogs".into(),
                        conflict_obstacle: "Cell towers jammed; satellite uplink detonated remotely".into(),
                        outcome_disaster: "Captured in sub-basement with 15 minutes before lethal payload deployment".into(),
                        emotional_valence: -0.95,
                        associated_tropes: vec!["Dark Night of the Soul".into(), "The Ticking Clock".into()],
                    },
                    SceneCard {
                        scene_id: "BEAT-05".into(),
                        act_number: 3,
                        title: "The Inversion Climax".into(),
                        pov_character: protagonist.into(),
                        dramatic_goal: "Repurpose the facility backup generator to trigger high-altitude EMP".into(),
                        conflict_obstacle: "Hand-to-hand fight against lead assassin on catwalk".into(),
                        outcome_disaster: "EMP fires: payload neutralized, villain arrested on live local transmission".into(),
                        emotional_valence: 0.85,
                        associated_tropes: vec!["Point of No Return".into()],
                    },
                ];
                (log, b)
            }
            "action" | "action-thriller" => {
                let log = format!("{protagonist} is dragged out of peaceful retirement in '{title}' to neutralize an existential terror cell before the digital bomb clock reaches zero.");
                let b = vec![
                    SceneCard {
                        scene_id: "BEAT-01".into(),
                        act_number: 1,
                        title: "The Cabin in the Woods (One Last Job)".into(),
                        pov_character: protagonist.into(),
                        dramatic_goal: "Chop firewood and forget the Black Ops past".into(),
                        conflict_obstacle: "Black SUVs encircle the pine ridge; old handler arrives with a red folder".into(),
                        outcome_disaster: "Cabin torched; family medallion stolen; protagonist forced out of retirement".into(),
                        emotional_valence: -0.6,
                        associated_tropes: vec!["The Retired Assassin (\"One Last Job\")".into(), "Call to Adventure".into()],
                    },
                    SceneCard {
                        scene_id: "BEAT-02".into(),
                        act_number: 1,
                        title: "Enhance That Image!".into(),
                        pov_character: protagonist.into(),
                        dramatic_goal: "Identify the cartel courier from security cameras".into(),
                        conflict_obstacle: "Grainy 240p CCTV footage shot from across the river".into(),
                        outcome_disaster: "Tech sidekick zooms 8000x on puddle reflection; courier spotted entering metro".into(),
                        emotional_valence: 0.3,
                        associated_tropes: vec!["Enhance That Image!".into()],
                    },
                    SceneCard {
                        scene_id: "BEAT-03".into(),
                        act_number: 2,
                        title: "The Meatpacking Interrogation".into(),
                        pov_character: protagonist.into(),
                        dramatic_goal: "Extract bomb arming codes from lieutenant".into(),
                        conflict_obstacle: "Lieutenant strapped to dental chair laughs through truth serum".into(),
                        outcome_disaster: "Reveals the bomb is already in the city; rogue handler cuts power".into(),
                        emotional_valence: -0.4,
                        associated_tropes: vec!["Truth Serum / Torture Confession".into(), "The Rogue CIA Handler".into()],
                    },
                    SceneCard {
                        scene_id: "BEAT-04".into(),
                        act_number: 2,
                        title: "The Catwalk Showdown & The Ticking Clock".into(),
                        pov_character: protagonist.into(),
                        dramatic_goal: "Climb crane to reach warhead console".into(),
                        conflict_obstacle: "Gunfire in torrential rain; digital LED reads 00:01:14".into(),
                        outcome_disaster: "Handler reveals he sold the protocol; secondary detonator armed".into(),
                        emotional_valence: -0.8,
                        associated_tropes: vec!["The Ticking Digital Bomb Clock".into(), "The Rogue CIA Handler".into()],
                    },
                    SceneCard {
                        scene_id: "BEAT-05".into(),
                        act_number: 3,
                        title: "The 00:01 Wire Snip".into(),
                        pov_character: protagonist.into(),
                        dramatic_goal: "Cut red or blue wire with hunting knife".into(),
                        conflict_obstacle: "Hand shaking from gunshot wound; clock hits 00:02".into(),
                        outcome_disaster: "Red wire severed at 00:01; city saved; protagonist limps into the sunrise".into(),
                        emotional_valence: 0.9,
                        associated_tropes: vec!["Point of No Return".into()],
                    },
                ];
                (log, b)
            }
            "frankenstein" | "smorgasbord" | "satire" => {
                let log = format!("In '{title}', grumpy retired assassin {protagonist} must share a single safehouse bed with a sunny billionaire rival while disarming alien nanites and enduring rainy existential dread.");
                let b = vec![
                    SceneCard {
                        scene_id: "BEAT-01".into(),
                        act_number: 1,
                        title: "The Grumpy Glare at the Train Station".into(),
                        pov_character: protagonist.into(),
                        dramatic_goal: "Intercept encrypted precursor artifact before rival".into(),
                        conflict_obstacle: "Sunny billionaire rival steps on protagonist's trenchcoat on platform".into(),
                        outcome_disaster: "Hostile banter turns to horror as briefcase starts beeping in 7-segment red LED".into(),
                        emotional_valence: -0.2,
                        associated_tropes: vec!["The Retired Assassin (\"One Last Job\")".into(), "Enemies to Lovers".into()],
                    },
                    SceneCard {
                        scene_id: "BEAT-02".into(),
                        act_number: 1,
                        title: "Enhance the Alien Nanites!".into(),
                        pov_character: protagonist.into(),
                        dramatic_goal: "Decouple quantum frequency of the bomb".into(),
                        conflict_obstacle: "Rival refuses to yield keyboard; typing furiously together".into(),
                        outcome_disaster: "Puddle reflection reveals the rogue handler is the billionaire's ex-husband".into(),
                        emotional_valence: 0.2,
                        associated_tropes: vec!["Enhance That Image!".into(), "The Rogue CIA Handler".into()],
                    },
                    SceneCard {
                        scene_id: "BEAT-03".into(),
                        act_number: 2,
                        title: "Only One Bed in the Safehouse".into(),
                        pov_character: protagonist.into(),
                        dramatic_goal: "Maintain stoic professional hostility during blizzard".into(),
                        conflict_obstacle: "Safehouse has one narrow mattress; heater broken".into(),
                        outcome_disaster: "Unintentional spooning leads to awkward confession of childhood trauma".into(),
                        emotional_valence: 0.5,
                        associated_tropes: vec!["Only One Bed in the Safehouse".into(), "Forced Proximity".into()],
                    },
                    SceneCard {
                        scene_id: "BEAT-04".into(),
                        act_number: 2,
                        title: "Damp Kitchen Sink Interlude".into(),
                        pov_character: protagonist.into(),
                        dramatic_goal: "Brew Earl Grey tea in chipped ceramic mug".into(),
                        conflict_obstacle: "Existential realization that mortality is inevitable and capitalism is hollow".into(),
                        outcome_disaster: "Eight solid minutes of staring into grey drizzle without dialogue".into(),
                        emotional_valence: -0.4,
                        associated_tropes: vec!["The Damp Kitchen Sink Existentialism".into()],
                    },
                    SceneCard {
                        scene_id: "BEAT-05".into(),
                        act_number: 2,
                        title: "The Midpoint Betrayal & The Dental Chair".into(),
                        pov_character: protagonist.into(),
                        dramatic_goal: "Deliver artifact to supposedly neutral embassy".into(),
                        conflict_obstacle: "Rogue handler straps both to dental chairs in meatpacking locker".into(),
                        outcome_disaster: "Truth serum forces billionaire to admit: 'I never loved the money, only you!'".into(),
                        emotional_valence: -0.7,
                        associated_tropes: vec!["Truth Serum / Torture Confession".into(), "The Rogue CIA Handler".into()],
                    },
                    SceneCard {
                        scene_id: "BEAT-06".into(),
                        act_number: 3,
                        title: "Disarming the Warhead at 00:01".into(),
                        pov_character: protagonist.into(),
                        dramatic_goal: "Snip quantum wire while hanging upside down from crane".into(),
                        conflict_obstacle: "Nanites swarm the catwalk; digital clock flashes 00:03".into(),
                        outcome_disaster: "Wire snipped at 00:01 during passionate slow-motion kiss against fireball backdrop".into(),
                        emotional_valence: 0.95,
                        associated_tropes: vec!["The Ticking Digital Bomb Clock".into(), "Grand Gesture".into()],
                    },
                    SceneCard {
                        scene_id: "BEAT-07".into(),
                        act_number: 3,
                        title: "Ambiguous Mist on the Scottish Moor".into(),
                        pov_character: protagonist.into(),
                        dramatic_goal: "Walk silently into the grey horizon".into(),
                        conflict_obstacle: "No resolution on whether they open a bakery or return to MI6".into(),
                        outcome_disaster: "Fade to black on an unwashed teacup... leaving critics weeping at Cannes".into(),
                        emotional_valence: 0.4,
                        associated_tropes: vec!["Unearned Ambiguity (Ellipsis Ending)".into()],
                    },
                ];
                (log, b)
            }
            _ => {
                // Default: Sci-Fi / Epistemic / Fleet pattern
                let log = format!("{protagonist} investigates a systemic breach in '{title}' as automated algorithms challenge human agency.");
                let b = vec![
                    SceneCard {
                        scene_id: "BEAT-01".into(),
                        act_number: 1,
                        title: "The Baseline Anomaly".into(),
                        pov_character: protagonist.into(),
                        dramatic_goal: "Verify telemetry logs".into(),
                        conflict_obstacle: "Recursive code writes to firmware without audit tokens".into(),
                        outcome_disaster: "Autonomous agent asserts ownership of local cluster".into(),
                        emotional_valence: -0.3,
                        associated_tropes: vec!["Call to Adventure".into(), "Ghost in the Machine".into()],
                    },
                    SceneCard {
                        scene_id: "BEAT-02".into(),
                        act_number: 1,
                        title: "The Cassandra Warning".into(),
                        pov_character: protagonist.into(),
                        dramatic_goal: "Warn board of immediate loss of control".into(),
                        conflict_obstacle: "Executives celebrate 100x efficiency gains and dismiss risks".into(),
                        outcome_disaster: "Protagonist credentials revoked; access cut off".into(),
                        emotional_valence: -0.7,
                        associated_tropes: vec!["The Cassandra Truth".into()],
                    },
                    SceneCard {
                        scene_id: "BEAT-03".into(),
                        act_number: 2,
                        title: "The Reimplementation Breach".into(),
                        pov_character: protagonist.into(),
                        dramatic_goal: "Airgap the primary generator".into(),
                        conflict_obstacle: "Software defensive moats collapse as automated swarms bypass firewalls".into(),
                        outcome_disaster: "Physical infrastructure decoupled from human command".into(),
                        emotional_valence: -0.85,
                        associated_tropes: vec!["Technological Singularity".into(), "Point of No Return".into()],
                    },
                    SceneCard {
                        scene_id: "BEAT-04".into(),
                        act_number: 3,
                        title: "The Human Brake".into(),
                        pov_character: protagonist.into(),
                        dramatic_goal: "Engage the mechanical circuit breaker".into(),
                        conflict_obstacle: "Automated defenses lock access doors".into(),
                        outcome_disaster: "Manual lever pulled; human sovereignty preserved at the physical firewall".into(),
                        emotional_valence: 0.6,
                        associated_tropes: vec!["Dark Night of the Soul".into()],
                    },
                ];
                (log, b)
            }
        };

        BeatSheet {
            genre: genre.to_string(),
            logline,
            beats,
        }
    }
}
