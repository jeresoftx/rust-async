use rust_async::cooperative::{CooperativeTask, Progress};

fn main() {
    let mut waiting = CooperativeTask::new();
    let mut ready = CooperativeTask::new();
    ready.notify();

    assert_eq!(waiting.poll(), Progress::Pending);
    assert_eq!(ready.poll(), Progress::Ready);
}
