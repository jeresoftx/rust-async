use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (sender, mut receiver) = mpsc::channel(2);
    sender.send(1).await.expect("receptor abierto");
    sender.send(2).await.expect("receptor abierto");
    drop(sender);

    let mut received = Vec::new();
    while let Some(message) = receiver.recv().await {
        received.push(message);
    }

    assert_eq!(received, vec![1, 2]);
}
