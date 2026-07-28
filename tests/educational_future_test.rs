use rust_async::educational_future::CountdownFuture;
use std::task::Poll;

#[test]
fn returns_pending_until_its_final_poll() {
    let mut future = CountdownFuture::new(2);
    assert_eq!(future.poll(), Poll::Pending);
    assert_eq!(future.poll(), Poll::Pending);
    assert_eq!(future.poll(), Poll::Ready(0));
}
