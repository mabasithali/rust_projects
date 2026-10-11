use rust_project::compute_status;

#[test]
fn test_integration() {
    assert!(!compute_status().is_empty());
}
