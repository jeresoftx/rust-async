use rust_async::coordination::first_ready;

#[tokio::test]
async fn returns_the_first_ready_branch() {
    assert_eq!(first_ready().await, "immediate");
}
