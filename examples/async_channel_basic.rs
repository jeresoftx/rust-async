use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (sender, mut receiver) = mpsc::channel(2);
    sender
        .send("primer mensaje")
        .await
        .expect("receptor abierto");
    sender
        .send("segundo mensaje")
        .await
        .expect("receptor abierto");
    drop(sender);

    while let Some(message) = receiver.recv().await {
        println!("{message}");
    }
}
