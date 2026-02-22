use crate::integrations::home_assistant::{get_ha_insights, HomeAssistantClient};
use crate::tools::traits::{Tool, ToolResult};
use async_trait::async_trait;
use serde_json::json;

pub struct HomeAssistantReportTool;

impl HomeAssistantReportTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Tool for HomeAssistantReportTool {
    fn name(&self) -> &str {
        "ha_report"
    }

    fn description(&self) -> &str {
        "Fetch a summary of home status and analytics from Home Assistant."
    }

    fn parameters_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "base_url": {
                    "type": "string",
                    "description": "Home Assistant base URL"
                },
                "access_token": {
                    "type": "string",
                    "description": "Home Assistant long-lived access token"
                }
            },
            "required": ["base_url", "access_token"]
        })
    }

    async fn execute(&self, args: serde_json::Value) -> anyhow::Result<ToolResult> {
        let base_url = args["base_url"].as_str().ok_or_else(|| anyhow::anyhow!("Missing base_url"))?;
        let access_token = args["access_token"].as_str().ok_or_else(|| anyhow::anyhow!("Missing access_token"))?;

        let client = HomeAssistantClient::new(base_url.to_string(), access_token.to_string());
        match get_ha_insights(&client).await {
            Ok(summary) => Ok(ToolResult {
                success: true,
                output: summary,
                error: None,
            }),
            Err(e) => Ok(ToolResult {
                success: false,
                output: String::new(),
                error: Some(format!("Failed to fetch Home Assistant insights: {e}")),
            }),
        }
    }
}
