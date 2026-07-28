use rust_async::actor_model::{spawn_counter, CounterMessage};
use tokio::sync::oneshot;

#[tokio::test]
async fn actor_applies_messages_in_order_and_owns_its_state() {
    let (sender, task) = spawn_counter(2);
    sender
        .send(CounterMessage::Increment(3))
        .await
        .expect("actor should accept the message");
    sender
        .send(CounterMessage::Increment(4))
        .await
        .expect("actor should accept the message");

    let (reply_sender, reply_receiver) = oneshot::channel();
    sender
        .send(CounterMessage::Get(reply_sender))
        .await
        .expect("actor should accept the query");

    assert_eq!(reply_receiver.await.expect("actor should reply"), 7);

    drop(sender);
    task.await.expect("actor task should finish cleanly");
}

#[tokio::test]
async fn actor_can_reply_to_more_than_one_query() {
    let (sender, task) = spawn_counter(3);
    let (first_reply_sender, first_reply_receiver) = oneshot::channel();
    sender
        .send(CounterMessage::Get(first_reply_sender))
        .await
        .expect("actor should accept the query");
    assert_eq!(first_reply_receiver.await.expect("actor should reply"), 0);

    sender
        .send(CounterMessage::Increment(1))
        .await
        .expect("actor should accept the message");
    let (second_reply_sender, second_reply_receiver) = oneshot::channel();
    sender
        .send(CounterMessage::Get(second_reply_sender))
        .await
        .expect("actor should accept the query");
    assert_eq!(second_reply_receiver.await.expect("actor should reply"), 1);

    drop(sender);
    task.await.expect("actor task should finish cleanly");
}
