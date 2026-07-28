use rust_async::educational_waker::WakeSignal;

#[test]
fn multiple_wakes_coalesce_into_one_pending_signal() {
    let mut signal = WakeSignal::new();
    signal.wake();
    signal.wake();
    assert!(signal.take());
    assert!(!signal.take());
}
