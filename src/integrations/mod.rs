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
                let status = (entry.status_fn)(config);
                let (icon, label) = match status {
                    IntegrationStatus::Active => ("✅", "Active"),
                    IntegrationStatus::Available => ("⚪", "Available"),
                    IntegrationStatus::ComingSoon => ("🔜", "Coming Soon"),
                };

                println!();
                println!(
                    "  {} {} — {}",
                    icon,
                    console::style(entry.name).white().bold(),
                    entry.description
                );
                println!("  Category: {:?}", entry.category);
                println!("  Status:   {label}");
                println!();

                // Show setup hints based on integration
                match entry.name {
                    "Telegram" => {
                        println!("  Setup:");
                        println!("    1. Message @BotFather on Telegram");
                        println!("    2. Create a bot and copy the token");
                        println!("    3. Run: zeroclaw channel add telegram '{{\"bot_token\":\"...\"}}'");
                    }
                    "Discord" => {
                        println!("  Setup:");
                        println!("    1. Go to https://discord.com/developers/applications");
                        println!("    2. Create app → Bot → Copy token");
                        println!("    3. Enable MESSAGE CONTENT intent");
                        println!("    4. Run: zeroclaw channel add discord '{{\"bot_token\":\"...\"}}'");
                    }
                    "Home Assistant" => {
                        println!("  Details:");
                        println!("    ZeroClaw connects to Home Assistant for read-only analytics.");
                        println!("    Status is currently '{label}'.");
                    }
                    "Matrix" => {
                        println!("  Setup:");
                        println!("    1. Create a Matrix account for the bot");
                        println!("    2. Get homeserver URL and access token");
                        println!("    3. Run: zeroclaw channel add matrix '{{\"homeserver\":\"...\",\"access_token\":\"...\",\"room_id\":\"...\"}}'");
                    }
                    _ => {
                        if status == IntegrationStatus::ComingSoon {
                            println!("  This integration is planned. Stay tuned!");
                        }
                    }
                }
                println!();
            } else {
                anyhow::bail!("Integration '{}' not found.", name);
            }
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    #[test]
    fn handle_command_info_is_case_insensitive() {
        let config = Config::default();
        let result = handle_command(
            crate::IntegrationCommands::Info { name: "TELEGRAM".into() },
            &config,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn handle_command_info_returns_error_for_unknown() {
        let config = Config::default();
        let result = handle_command(
            crate::IntegrationCommands::Info {
                name: "nonexistent".into(),
            },
            &config,
        );
        assert!(result.is_err());
    }
}
