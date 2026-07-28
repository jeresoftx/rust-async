#[tokio::main]
async fn main() {
    let task = tokio::spawn(async { "ready" });
    assert_eq!(task.await.expect("task should finish"), "ready");
}
