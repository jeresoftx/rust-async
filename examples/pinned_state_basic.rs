use rust_async::pinned_state::PinnedState;

fn main() {
    let mut state = PinnedState::new("connected");
    state.advance();
    assert_eq!(state.steps(), 1);
}
