use zeroclaw::integrations::home_assistant::HomeAssistantClient;
use zeroclaw::tools::{HomeAssistantReportTool, Tool};
use axum::{routing::get, Json, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use serde_json::json;

#[tokio::test]
async fn test_ha_report_tool_execution() {
    // 1. Setup mock server
    let app = Router::new().route(
        "/api/states",
        get(|| async {
            Json(vec![
                serde_json::json!({
                    "entity_id": "light.kitchen",
                    "state": "on",
                    "last_changed": "2024-02-22T10:00:00Z"
                })
            ])
        }),
    );

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    // 2. Test tool
    let tool = HomeAssistantReportTool::new();
    let args = serde_json::json!({
        "base_url": format!("http://{}", addr),
        "access_token": "test_token"
    });
    let result = tool.execute(args).await.expect("Should execute tool");

    assert!(result.success);
    assert!(result.output.contains("1 light is on"));
}

#[tokio::test]
async fn test_ha_client_construction() {
    let client = HomeAssistantClient::new("http://localhost:8123".to_string(), "test_token".to_string());
    assert_eq!(client.base_url(), "http://localhost:8123");
}

#[tokio::test]
async fn test_ha_client_fetch_states_failure() {
    let client = HomeAssistantClient::new("http://invalid-ha".to_string(), "test_token".to_string());
    let result = client.fetch_states().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_ha_client_fetch_single_state() {
    // 1. Setup mock server
    let app = Router::new().route(
        "/api/states/light.living_room",
        get(|| async {
            Json(serde_json::json!({
                "entity_id": "light.living_room",
                "state": "on",
                "last_changed": "2024-02-22T10:00:00Z"
            }))
        }),
    );

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    // 2. Test client
    let client = HomeAssistantClient::new(format!("http://{}", addr), "test_token".to_string());
    let state = client.fetch_state("light.living_room").await.expect("Should fetch state");

    assert_eq!(state.entity_id, "light.living_room");
    assert_eq!(state.state, "on");
}
