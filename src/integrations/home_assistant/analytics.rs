use super::HaState;

pub struct HomeHabitAnalyzer;

pub struct HabitSummary {
    pub active_lights: Vec<String>,
    pub message: String,
}

impl HomeHabitAnalyzer {
    pub fn new() -> Self {
        Self
    }

    pub fn analyze(&self, states: &[HaState]) -> HabitSummary {
        let active_lights: Vec<String> = states
            .iter()
            .filter(|s| s.entity_id.starts_with("light.") && s.state == "on")
            .map(|s| s.entity_id.clone())
            .collect();

        let message = if active_lights.is_empty() {
            "All lights are off.".to_string()
        } else if active_lights.len() == 1 {
            format!("1 light is on: {}.", active_lights[0])
        } else {
            format!("{} lights are on.", active_lights.len())
        };

        HabitSummary {
            active_lights,
            message,
        }
    }
}
