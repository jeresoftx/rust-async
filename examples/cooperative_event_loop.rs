use rust_async::cooperative::{CooperativeTask, Progress};

fn main() {
    let mut first = CooperativeTask::new();
    let mut second = CooperativeTask::new();

    assert_eq!(first.poll(), Progress::Pending);
    assert_eq!(second.poll(), Progress::Pending);

    second.notify();
    assert_eq!(first.poll(), Progress::Pending);
    assert_eq!(second.poll(), Progress::Ready);
}
