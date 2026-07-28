use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (sender, _receiver) = mpsc::channel(1);
    sender.try_send("primer mensaje").expect("hay capacidad");
    assert!(sender.try_send("segundo mensaje").is_err());
}
