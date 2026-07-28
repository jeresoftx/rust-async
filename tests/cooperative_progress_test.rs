use rust_async::cooperative::{CooperativeTask, Progress};

#[test]
fn pending_task_needs_an_explicit_notification() {
    let mut task = CooperativeTask::new();

    assert_eq!(task.poll(), Progress::Pending);
    assert_eq!(task.poll(), Progress::Pending);

    task.notify();

    assert_eq!(task.poll(), Progress::Ready);
}

#[test]
fn completed_task_remains_ready() {
    let mut task = CooperativeTask::new();
    task.notify();

    assert_eq!(task.poll(), Progress::Ready);
    assert_eq!(task.poll(), Progress::Ready);
}
