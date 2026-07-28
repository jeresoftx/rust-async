use rust_async::educational_future::CountdownFuture;
use rust_async::single_task_executor::SingleTaskExecutor;

fn main() {
    let mut executor = SingleTaskExecutor::new(CountdownFuture::new(1));
    assert!(!executor.step());
    assert!(executor.step());
}
