use rust_async::educational_future::CountdownFuture;
use rust_async::task_queue::TaskQueue;

fn main() {
    let mut queue = TaskQueue::new();
    queue.push(CountdownFuture::new(0));
    assert!(queue.step());
    assert!(queue.is_empty());
}
