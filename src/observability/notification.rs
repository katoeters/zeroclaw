use crate::config::Config;
use anyhow::{Context, Result};
use std::sync::Arc;

/// System-level notifier for broadcasting important events (new skills, tools, etc.)
pub struct SystemNotifier {
    config: Arc<Config>,
}

impl SystemNotifier {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// Send a notification to the primary Telegram user if configured.
    pub async fn notify_telegram(&self, message: &str) -> Result<()> {
        let Some(telegram_cfg) = &self.config.channels_config.telegram else {
            return Ok(());
        };

        if telegram_cfg.bot_token.is_empty() {
            return Ok(());
        }

        // Use the first allowed user as the primary recipient for system notifications.
        let Some(primary_user) = telegram_cfg.allowed_users.first() else {
            tracing::warn!("Telegram notifier: No allowed_users configured, nowhere to send notification");
            return Ok(());
        };

        let client = crate::config::build_runtime_proxy_client("system.notifier");
        let url = format!(
            "https://api.telegram.org/bot{}/sendMessage",
            telegram_cfg.bot_token
        );

        let body = serde_json::json!({
            "chat_id": primary_user,
            "text": message,
            "parse_mode": "Markdown"
        });

        let resp = client
            .post(&url)
            .json(&body)
            .send()
            .await
            .context("Failed to send system notification to Telegram")?;

        if !resp.status().is_success() {
            let err_text = resp.text().await.unwrap_or_default();
            tracing::error!("Telegram notification failed: {}", err_text);
        } else {
            tracing::info!("System notification sent to Telegram user {}", primary_user);
        }

        Ok(())
    }

    /// Notify about a newly learned skill.
    pub async fn notify_new_skill(&self, skill_name: &str) -> Result<()> {
        let msg = format!("🎓 *Ezra learned a new skill:* `{}`", skill_name);
        self.notify_telegram(&msg).await
    }

    /// Notify about a newly enabled tool.
    pub async fn notify_new_tool(&self, tool_name: &str) -> Result<()> {
        let msg = format!("🛠️ *New tool available:* `{}`", tool_name);
        self.notify_telegram(&msg).await
    }
}
