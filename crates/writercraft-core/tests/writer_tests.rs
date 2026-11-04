use writercraft_core::WriterEngine;

#[test]
fn test_writercraft_scenes_and_arc() {
    let engine = WriterEngine::new();
    assert_eq!(engine.active_scenes.len(), 2);

    let arc = engine.analyze_narrative_arc().expect("arc analysis");
    assert_eq!(arc.total_scenes, 2);
    assert_eq!(arc.max_tension_scene_id, "SCENE-02");
    assert!(arc.tropes_utilized.contains(&"Call to Adventure".to_string()));
}
