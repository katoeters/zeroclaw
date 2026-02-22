use zeroclaw::integrations::home_assistant::HaState;
use zeroclaw::integrations::home_assistant::analytics::{HomeHabitAnalyzer, HabitSummary};

#[test]
fn test_analyze_lights() {
    let states = vec![
        HaState {
            entity_id: "light.living_room".into(),
            state: "on".into(),
            last_changed: "2024-02-22T08:00:00Z".into(),
        },
        HaState {
            entity_id: "light.kitchen".into(),
            state: "off".into(),
            last_changed: "2024-02-22T07:00:00Z".into(),
        },
        HaState {
            entity_id: "sensor.temp_living_room".into(),
            state: "22.5".into(),
            last_changed: "2024-02-22T08:00:00Z".into(),
        },
    ];

    let analyzer = HomeHabitAnalyzer::new();
    let summary = analyzer.analyze(&states);

    assert_eq!(summary.active_lights.len(), 1);
    assert_eq!(summary.active_lights[0], "light.living_room");
    assert!(summary.message.contains("1 light is on"));
    assert!(!summary.suggestions.is_empty());
    assert!(summary.suggestions[0].contains("save energy"));
}
