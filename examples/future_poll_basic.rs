use rust_async::educational_future::CountdownFuture;
use std::task::Poll;

fn main() {
    let mut future = CountdownFuture::new(1);
    assert_eq!(future.poll(), Poll::Pending);
    assert_eq!(future.poll(), Poll::Ready(0));
}
