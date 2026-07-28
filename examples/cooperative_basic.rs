use rust_async::cooperative::{CooperativeTask, Progress};

fn main() {
    let mut task = CooperativeTask::new();
    assert_eq!(task.poll(), Progress::Pending);
    task.notify();
    assert_eq!(task.poll(), Progress::Ready);
}
