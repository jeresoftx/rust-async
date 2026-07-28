use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (sender, receiver) = mpsc::channel(1);
    drop(receiver);

    assert!(sender.send("mensaje").await.is_err());
}
