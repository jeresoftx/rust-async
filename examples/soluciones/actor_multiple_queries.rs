use rust_async::actor_model::{spawn_counter, CounterMessage};
use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    let (sender, task) = spawn_counter(3);
    let (first_reply, first_response) = oneshot::channel();
    sender
        .send(CounterMessage::Get(first_reply))
        .await
        .expect("actor abierto");
    assert_eq!(first_response.await.expect("actor responde"), 0);

    sender
        .send(CounterMessage::Increment(5))
        .await
        .expect("actor abierto");
    let (second_reply, second_response) = oneshot::channel();
    sender
        .send(CounterMessage::Get(second_reply))
        .await
        .expect("actor abierto");
    assert_eq!(second_response.await.expect("actor responde"), 5);

    drop(sender);
    task.await.expect("actor termina correctamente");
}
