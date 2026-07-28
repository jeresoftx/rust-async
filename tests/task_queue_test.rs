use rust_async::educational_future::CountdownFuture;
use rust_async::task_queue::TaskQueue;

#[test]
fn requeues_pending_tasks_and_removes_ready_tasks() {
    let mut queue = TaskQueue::new();
    queue.push(CountdownFuture::new(1));
    queue.push(CountdownFuture::new(0));

    assert!(!queue.step());
    assert!(queue.step());
    assert!(queue.step());
    assert!(queue.is_empty());
}
