use test_log::test;
use tracing::info;

#[test]
fn test_info () {
    info!("❌ info in integration test");
    assert!(false);
}
