use clap::{Parser, Subcommand};
use writercraft_core::WriterEngine;

#[derive(Parser)]
#[command(name = "writercraft-cli", about = "WriterCraft Local Corkboard & Narrative Arc CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all scene cards in the active manuscript
    Scenes,
    /// Analyze narrative tension and emotional valence arc
    Arc,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let engine = WriterEngine::new();

    match cli.command {
        Commands::Scenes => {
            println!("{}", serde_json::to_string_pretty(&engine.active_scenes)?);
        }
        Commands::Arc => {
            let arc = engine.analyze_narrative_arc()?;
            println!("{}", serde_json::to_string_pretty(&arc)?);
        }
    }
    Ok(())
}
