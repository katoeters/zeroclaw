pub mod home_assistant;
pub mod registry;

use crate::config::Config;

<<<<<<< HEAD
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
=======
/// Integration status
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum IntegrationStatus {
    /// Fully implemented and ready to use
    Available,
    /// Configured and active
    Active,
    /// Planned but not yet implemented
    ComingSoon,
}

/// Integration category
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
>>>>>>> upstream/main
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
<<<<<<< HEAD
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
=======
        crate::IntegrationCommands::List { category, status } => {
            list_integrations(config, category.as_deref(), status.as_deref())
        }
        crate::IntegrationCommands::Search { query } => search_integrations(config, &query),
        crate::IntegrationCommands::Info { name } => show_integration_info(config, &name),
    }
}

fn status_icon(status: IntegrationStatus) -> &'static str {
    match status {
        IntegrationStatus::Active => "✅",
        IntegrationStatus::Available => "⚪",
        IntegrationStatus::ComingSoon => "🔜",
    }
}

fn parse_category_filter(input: &str) -> Option<IntegrationCategory> {
    match input.to_lowercase().as_str() {
        "chat" => Some(IntegrationCategory::Chat),
        "ai" | "model" | "models" | "ai-model" | "ai-models" => Some(IntegrationCategory::AiModel),
        "productivity" => Some(IntegrationCategory::Productivity),
        "music" | "audio" | "music-audio" => Some(IntegrationCategory::MusicAudio),
        "smart-home" | "smarthome" | "home" => Some(IntegrationCategory::SmartHome),
        "tools" | "automation" | "tools-automation" => Some(IntegrationCategory::ToolsAutomation),
        "media" | "creative" | "media-creative" => Some(IntegrationCategory::MediaCreative),
        "social" => Some(IntegrationCategory::Social),
        "platform" | "platforms" => Some(IntegrationCategory::Platform),
        _ => None,
    }
}

fn parse_status_filter(input: &str) -> Option<IntegrationStatus> {
    match input.to_lowercase().as_str() {
        "active" => Some(IntegrationStatus::Active),
        "available" => Some(IntegrationStatus::Available),
        "coming-soon" | "comingsoon" | "soon" => Some(IntegrationStatus::ComingSoon),
        _ => None,
    }
}

fn list_integrations(
    config: &Config,
    category_filter: Option<&str>,
    status_filter: Option<&str>,
) -> Result<()> {
    let entries = registry::all_integrations();

    let cat_filter = category_filter.map(parse_category_filter);
    if let Some(None) = cat_filter.as_ref() {
        anyhow::bail!(
            "Unknown category: '{}'. Valid: chat, ai, productivity, music, smart-home, tools, media, social, platform",
            category_filter.unwrap_or_default()
        );
    }
    let cat_filter = cat_filter.flatten();

    let stat_filter = status_filter.map(parse_status_filter);
    if let Some(None) = stat_filter.as_ref() {
        anyhow::bail!(
            "Unknown status: '{}'. Valid: active, available, coming-soon",
            status_filter.unwrap_or_default()
        );
    }
    let stat_filter = stat_filter.flatten();

    let mut count = 0usize;
    for cat in IntegrationCategory::all() {
        if let Some(ref cf) = cat_filter {
            if *cf != *cat {
                continue;
            }
        }

        let cat_entries: Vec<_> = entries
            .iter()
            .filter(|e| e.category == *cat)
            .filter(|e| {
                if let Some(ref sf) = stat_filter {
                    (e.status_fn)(config) == *sf
                } else {
                    true
                }
            })
            .collect();

        if cat_entries.is_empty() {
            continue;
        }

        println!();
        println!("  {}", console::style(cat.label()).bold().underlined());
        for entry in &cat_entries {
            let status = (entry.status_fn)(config);
            println!(
                "    {} {:<20} {}",
                status_icon(status),
                entry.name,
                console::style(entry.description).dim()
            );
            count += 1;
        }
    }

    println!();
    println!("  {} integration(s) shown.", count);
    println!();
    Ok(())
}

fn search_integrations(config: &Config, query: &str) -> Result<()> {
    let entries = registry::all_integrations();
    let query_lower = query.to_lowercase();

    let matches: Vec<_> = entries
        .iter()
        .filter(|e| {
            e.name.to_lowercase().contains(&query_lower)
                || e.description.to_lowercase().contains(&query_lower)
        })
        .collect();

    if matches.is_empty() {
        println!();
        println!("  No integrations matching '{query}'.");
        println!();
        return Ok(());
    }

    println!();
    for entry in &matches {
        let status = (entry.status_fn)(config);
        println!(
            "    {} {:<20} {} — {}",
            status_icon(status),
            entry.name,
            console::style(entry.category.label()).dim(),
            entry.description,
        );
    }
    println!();
    println!("  {} result(s) for '{query}'.", matches.len());
    println!();
    Ok(())
}

fn show_integration_info(config: &Config, name: &str) -> Result<()> {
    let entries = registry::all_integrations();
    let name_lower = name.to_lowercase();
>>>>>>> upstream/main

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

<<<<<<< HEAD
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
=======
    let status = (entry.status_fn)(config);
    let icon = status_icon(status);
    let label = match status {
        IntegrationStatus::Active => "Active",
        IntegrationStatus::Available => "Available",
        IntegrationStatus::ComingSoon => "Coming Soon",
    };

    println!();
    println!(
        "  {} {} — {}",
        icon,
        console::style(entry.name).white().bold(),
        entry.description
    );
    println!("  Category: {}", entry.category.label());
    println!("  Status:   {label}");
    println!();

    // Show setup hints based on integration
    match entry.name {
        "Telegram" => {
            println!("  Setup:");
            println!("    1. Message @BotFather on Telegram");
            println!("    2. Create a bot and copy the token");
            println!("    3. Run: zeroclaw onboard --channels-only");
            println!("    4. Start: zeroclaw channel start");
        }
        "Discord" => {
            println!("  Setup:");
            println!("    1. Go to https://discord.com/developers/applications");
            println!("    2. Create app → Bot → Copy token");
            println!("    3. Enable MESSAGE CONTENT intent");
            println!("    4. Run: zeroclaw onboard --channels-only");
        }
        "Slack" => {
            println!("  Setup:");
            println!("    1. Go to https://api.slack.com/apps");
            println!("    2. Create app → Bot Token Scopes → Install");
            println!("    3. Run: zeroclaw onboard --channels-only");
        }
        "OpenRouter" => {
            println!("  Setup:");
            println!("    1. Get API key at https://openrouter.ai/keys");
            println!("    2. Run: zeroclaw onboard");
            println!("    Access 200+ models with one key.");
        }
        "Ollama" => {
            println!("  Setup:");
            println!("    1. Install: brew install ollama");
            println!("    2. Pull a model: ollama pull llama3");
            println!("    3. Set provider to 'ollama' in config.toml");
        }
        "iMessage" => {
            println!("  Setup (macOS only):");
            println!("    Uses AppleScript bridge to send/receive iMessages.");
            println!("    Requires Full Disk Access in System Settings → Privacy.");
        }
        "GitHub" => {
            println!("  Setup:");
            println!("    1. Create a personal access token at https://github.com/settings/tokens");
            println!("    2. Add to config: [integrations.github] token = \"ghp_...\"");
        }
        "Browser" => {
            println!("  Built-in:");
            println!("    ZeroClaw can control Chrome/Chromium for web tasks.");
            println!("    Uses headless browser automation.");
        }
        "Cron" => {
            println!("  Built-in:");
            println!("    Schedule tasks in ~/.zeroclaw/workspace/cron/");
            println!("    Run: zeroclaw cron list");
        }
        "Webhooks" => {
            println!("  Built-in:");
            println!("    HTTP endpoint for external triggers.");
            println!("    Run: zeroclaw gateway");
        }
        _ => {
            if status == IntegrationStatus::ComingSoon {
                println!("  This integration is planned. Stay tuned!");
                println!("  Track progress: https://github.com/theonlyhennygod/zeroclaw");
>>>>>>> upstream/main
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

    #[test]
    fn list_all_integrations_succeeds() {
        let config = Config::default();
        let result = handle_command(
            crate::IntegrationCommands::List {
                category: None,
                status: None,
            },
            &config,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn list_with_category_filter_succeeds() {
        let config = Config::default();
        let result = handle_command(
            crate::IntegrationCommands::List {
                category: Some("chat".into()),
                status: None,
            },
            &config,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn list_with_status_filter_succeeds() {
        let config = Config::default();
        let result = handle_command(
            crate::IntegrationCommands::List {
                category: None,
                status: Some("available".into()),
            },
            &config,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn list_with_invalid_category_fails() {
        let config = Config::default();
        let result = handle_command(
            crate::IntegrationCommands::List {
                category: Some("nonexistent".into()),
                status: None,
            },
            &config,
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Unknown category"));
    }

    #[test]
    fn list_with_invalid_status_fails() {
        let config = Config::default();
        let result = handle_command(
            crate::IntegrationCommands::List {
                category: None,
                status: Some("bogus".into()),
            },
            &config,
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Unknown status"));
    }

    #[test]
    fn search_finds_matching_integrations() {
        let config = Config::default();
        let result = handle_command(
            crate::IntegrationCommands::Search {
                query: "telegram".into(),
            },
            &config,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn search_no_match_succeeds() {
        let config = Config::default();
        let result = handle_command(
            crate::IntegrationCommands::Search {
                query: "zzz-no-match-zzz".into(),
            },
            &config,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn parse_category_filter_covers_all_aliases() {
        assert!(parse_category_filter("chat").is_some());
        assert!(parse_category_filter("ai").is_some());
        assert!(parse_category_filter("models").is_some());
        assert!(parse_category_filter("productivity").is_some());
        assert!(parse_category_filter("music").is_some());
        assert!(parse_category_filter("smart-home").is_some());
        assert!(parse_category_filter("tools").is_some());
        assert!(parse_category_filter("media").is_some());
        assert!(parse_category_filter("social").is_some());
        assert!(parse_category_filter("platform").is_some());
        assert!(parse_category_filter("bogus").is_none());
    }

    #[test]
    fn parse_status_filter_covers_all_aliases() {
        assert!(parse_status_filter("active").is_some());
        assert!(parse_status_filter("available").is_some());
        assert!(parse_status_filter("coming-soon").is_some());
        assert!(parse_status_filter("soon").is_some());
        assert!(parse_status_filter("bogus").is_none());
    }
}
