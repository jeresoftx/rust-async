#[tokio::test]
async fn runtime_completes_two_independent_tasks() {
    let first = tokio::spawn(async { 1_u8 });
    let second = tokio::spawn(async { 2_u8 });

    assert_eq!(first.await.expect("first task should finish"), 1);
    assert_eq!(second.await.expect("second task should finish"), 2);
}
