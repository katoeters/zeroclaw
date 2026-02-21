use zeroclaw::channels::matrix::MatrixChannel;
use zeroclaw::channels::traits::Channel;

#[test]
fn test_matrix_channel_construction() {
    let channel = MatrixChannel::new(
        "https://matrix.org".to_string(),
        "test_token".to_string(),
        "!room:matrix.org".to_string(),
        vec!["@husband:matrix.org".to_string(), "@wife:matrix.org".to_string()],
    );
    assert_eq!(channel.name(), "matrix");
}

#[tokio::test]
async fn test_matrix_channel_health_check_failure_with_invalid_config() {
    let channel = MatrixChannel::new(
        "https://invalid-homeserver".to_string(),
        "test_token".to_string(),
        "invalid_room".to_string(),
        vec![],
    );
    // Should fail because room ID must start with ! or #
    assert!(!channel.health_check().await);
}
