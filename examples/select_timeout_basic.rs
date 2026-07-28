#[tokio::main]
async fn main() {
    assert_eq!(rust_async::coordination::first_ready().await, "immediate");
    assert_eq!(
        rust_async::coordination::immediate_with_timeout()
            .await
            .expect("operation should finish"),
        1
    );
}
