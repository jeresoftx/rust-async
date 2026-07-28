use rust_async::pinned_state::PinnedState;

#[test]
fn pinned_state_exposes_progress_without_replacing_its_value() {
    let mut state = PinnedState::new("connected");
    assert_eq!(*state.value(), "connected");
    state.advance();
    assert_eq!(state.steps(), 1);
}
