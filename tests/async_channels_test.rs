use rust_async::async_channels::{
    bounded_channel_applies_backpressure, drain_after_senders_close, receiver_closure_rejects_send,
};

#[tokio::test]
async fn bounded_channel_reports_full_queue() {
    assert!(bounded_channel_applies_backpressure());
}

#[tokio::test]
async fn receiver_drains_messages_before_observing_sender_closure() {
    assert_eq!(drain_after_senders_close().await, vec![1, 2]);
}

#[tokio::test]
async fn closed_receiver_rejects_new_messages() {
    assert!(receiver_closure_rejects_send().await);
}
