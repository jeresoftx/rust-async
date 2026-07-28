use rust_async::educational_future::CountdownFuture;
use rust_async::single_task_executor::SingleTaskExecutor;

#[test]
fn runs_a_task_until_it_completes() {
    let mut executor = SingleTaskExecutor::new(CountdownFuture::new(1));
    assert!(!executor.step());
    assert!(executor.step());
    assert!(executor.is_complete());
}
