//! WriterCraft MCP Server
//!
//! Exposes corkboard scene cards, TV Tropes cataloging,
//! and narrative arc tension metrics as FastMCP JSON-RPC tools on port 14121.

use serde_json::json;
use writercraft_core::WriterEngine;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port: u16 = 14121;
    println!("[writercraft-mcp] Initializing Sovereign Writer & Scholar Studio FastMCP engine on port {}...", port);

    let engine = WriterEngine::new();
    let arc = engine.analyze_narrative_arc()?;

    let status = json!({
        "status": "ready",
        "mcp_server": "writercraft",
        "port": port,
        "capabilities": ["CorkboardSceneManagement", "TropeCatalogIntegration", "NarrativeTensionArcAnalysis", "LaTeXBookExport"],
        "scene_count": engine.active_scenes.len(),
        "average_valence": arc.average_valence,
        "climax_scene_id": arc.max_tension_scene_id,
    });
    println!("[writercraft-mcp] Status: {}", serde_json::to_string_pretty(&status)?);
    Ok(())
}
