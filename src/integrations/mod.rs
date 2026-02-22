pub mod home_assistant;
pub mod registry;

use crate::config::Config;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegrationCategory {
    Chat,
    AiModel,
    Productivity,
    MusicAudio,
    SmartHome,
    ToolsAutomation,
    MediaCreative,
    Social,
    Platform,
}

impl IntegrationCategory {
    pub fn all() -> &'static [Self] {
        &[
            Self::Chat,
            Self::AiModel,
            Self::Productivity,
            Self::MusicAudio,
            Self::SmartHome,
            Self::ToolsAutomation,
            Self::MediaCreative,
            Self::Social,
            Self::Platform,
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegrationStatus {
    Active,
    Available,
    ComingSoon,
}

pub struct IntegrationEntry {
    pub name: &'static str,
    pub description: &'static str,
    pub category: IntegrationCategory,
    pub status_fn: fn(&Config) -> IntegrationStatus,
}

pub fn handle_command(command: crate::IntegrationCommands, config: &Config) -> anyhow::Result<()> {
    match command {
        crate::IntegrationCommands::Info { name } => {
            let all = registry::all_integrations();
            let entry = all.iter().find(|e| e.name.eq_ignore_ascii_case(&name));

            if let Some(entry) = entry {
                println!("Integration: {}", entry.name);
                println!("Description: {}", entry.description);
                println!("Category: {:?}", entry.category);
                println!("Status: {:?}", (entry.status_fn)(config));
            } else {
                anyhow::bail!("Integration '{}' not found.", name);
            }
            Ok(())
        }
    }
}
