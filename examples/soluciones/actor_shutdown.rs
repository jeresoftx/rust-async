use rust_async::actor_model::{spawn_counter, CounterMessage};

#[tokio::main]
async fn main() {
    let (sender, task) = spawn_counter(1);
    sender
        .send(CounterMessage::Increment(1))
        .await
        .expect("actor abierto");

    drop(sender);
    task.await.expect("actor termina al cerrar el buzón");
}
