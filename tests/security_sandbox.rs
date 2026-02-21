use zeroclaw::security::policy::SandboxPolicy;

#[test]
fn test_sandbox_default_policy() {
    let policy = SandboxPolicy::default();
    assert!(policy.read_only_filesystem, "Default policy should have read-only filesystem enabled");
    assert!(!policy.network_access, "Default policy should have network access disabled");
}

#[test]
fn test_sandbox_custom_policy() {
    let policy = SandboxPolicy::new().allow_network(true);
    assert!(policy.network_access, "Custom policy should have network access enabled");
}