pub mod analytics;

use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct HaState {
    pub entity_id: String,
    pub state: String,
    pub last_changed: String,
}

pub struct HomeAssistantClient {
    base_url: String,
    access_token: String,
    http_client: Client,
}

impl HomeAssistantClient {
    pub fn new(base_url: String, access_token: String) -> Self {
        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            access_token,
            http_client: Client::new(),
        }
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub async fn fetch_states(&self) -> anyhow::Result<Vec<HaState>> {
        let url = format!("{}/api/states", self.base_url);
        let resp = self
            .http_client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .send()
            .await?;

        if !resp.status().is_success() {
            let err = resp.text().await?;
            anyhow::bail!("Home Assistant API failed: {err}");
        }

        Ok(resp.json().await?)
    }

    pub async fn fetch_state(&self, entity_id: &str) -> anyhow::Result<HaState> {
        let url = format!("{}/api/states/{}", self.base_url, entity_id);
        let resp = self
            .http_client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .send()
            .await?;

        if !resp.status().is_success() {
            let err = resp.text().await?;
            anyhow::bail!("Home Assistant API failed for {entity_id}: {err}");
        }

        Ok(resp.json().await?)
    }
}

pub async fn get_ha_insights(client: &HomeAssistantClient) -> anyhow::Result<String> {
    let states = client.fetch_states().await?;
    let analyzer = analytics::HomeHabitAnalyzer::new();
    let summary = analyzer.analyze(&states);
    
    let mut output = summary.message;
    if !summary.suggestions.is_empty() {
        output.push_str("\n\nSuggestions:\n");
        for suggestion in summary.suggestions {
            output.push_str(&format!("- {suggestion}\n"));
        }
    }
    
    Ok(output)
}
