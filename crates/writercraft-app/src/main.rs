use clap::Parser;
use serde_json::json;
use writercraft_core::WriterEngine;

#[derive(Parser, Debug)]
#[command(name = "writercraft", about = "WriterCraft Sovereign Narrative & Corkboard Studio")]
struct Args {
    #[arg(long)]
    smoke: bool,
}

pub struct WriterCraftApp {
    engine: WriterEngine,
    status_msg: String,
}

impl WriterCraftApp {
    pub fn new() -> Self {
        Self {
            engine: WriterEngine::new(),
            status_msg: "Writer & Corkboard Studio Ready (Port 14121)".to_string(),
        }
    }
}

impl eframe::App for WriterCraftApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::top("top_panel").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading("✍️ WriterCraft — Narrative Corkboard & Epistemic Studio");
                ui.separator();
                ui.label(&self.status_msg);
            });
        });

        egui::Panel::left("corkboard_nav").show(ui, |ui| {
            ui.heading("Act I Corkboard");
            for scene in &self.engine.active_scenes {
                ui.group(|ui| {
                    ui.label(egui::RichText::new(&scene.title).strong());
                    ui.label(format!("POV: {}", scene.pov_character));
                    ui.label(format!("Goal: {}", scene.dramatic_goal));
                    ui.label(format!("Conflict: {}", scene.conflict_obstacle));
                    ui.label(format!("Disaster: {}", scene.outcome_disaster));
                    ui.label(format!("Valence: {:.1}", scene.emotional_valence));
                    ui.label(format!("Tropes: {}", scene.associated_tropes.join(", ")));
                });
                ui.add_space(4.0);
            }
        });

        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Narrative Arc Tension & Climax Analytics");
            if let Ok(arc) = self.engine.analyze_narrative_arc() {
                ui.group(|ui| {
                    ui.label(format!("Total Outlined Scenes: {}", arc.total_scenes));
                    ui.label(format!("Mean Emotional Valence: {:.2}", arc.average_valence));
                    ui.label(format!("Peak Narrative Tension: Scene [{}]", arc.max_tension_scene_id));
                    ui.separator();
                    ui.label("Integrated TV Tropes Taxonomy:");
                    for trope in arc.tropes_utilized {
                        ui.label(format!("• {}", trope));
                    }
                });
            }
        });
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    if args.smoke {
        println!("[writercraft] Running headless smoke verification...");
        let engine = WriterEngine::new();
        let arc = engine.analyze_narrative_arc()?;
        let smoke_payload = json!({
            "app": "writercraft",
            "version": "0.1.0",
            "status": "ready",
            "control_port": 14121,
            "success": true,
            "scene_count": engine.active_scenes.len(),
            "climax_scene_id": arc.max_tension_scene_id,
            "capabilities": ["CorkboardSceneManagement", "TropeCatalogIntegration", "NarrativeTensionArcAnalysis", "LaTeXBookExport"]
        });
        println!("[writercraft] Smoke OK: {}", smoke_payload);
        return Ok(());
    }

    let app = WriterCraftApp::new();
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 800.0])
            .with_title("WriterCraft — Narrative & Corkboard Studio"),
        ..Default::default()
    };

    eframe::run_native(
        "WriterCraft",
        native_options,
        Box::new(|_cc| Ok(Box::new(app))),
    )?;
    Ok(())
}
