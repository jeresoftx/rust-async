use rust_async::educational_waker::WakeSignal;

fn main() {
    let mut signal = WakeSignal::new();
    signal.wake();
    assert!(signal.take());
    assert!(!signal.take());
}
