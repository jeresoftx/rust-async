use std::time::Duration;

#[tokio::main]
async fn main() {
    let result = tokio::time::timeout(Duration::from_millis(10), async { 7_u8 }).await;
    assert_eq!(result.expect("operation should finish"), 7);
}
