use rust_async::actor_model::{spawn_counter, CounterMessage};
use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    let (sender, task) = spawn_counter(2);
    sender
        .send(CounterMessage::Increment(3))
        .await
        .expect("actor abierto");
    sender
        .send(CounterMessage::Increment(4))
        .await
        .expect("actor abierto");

    let (reply, response) = oneshot::channel();
    sender
        .send(CounterMessage::Get(reply))
        .await
        .expect("actor abierto");
    println!("contador: {}", response.await.expect("actor responde"));

    drop(sender);
    task.await.expect("actor termina correctamente");
}
