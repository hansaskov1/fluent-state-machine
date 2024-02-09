use state_machine_dsl::StateMachine;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_machine() {
        // Create a new state machine
        let state_machine = StateMachine::new(0, "STOP")
            .transition("STOP", "PLAYING", |track| {
                *track = 1;
                *track == 0
            })
            .transition("STOP", "STOP", |track| *track >= 10);

        // Execute state transitions
        let state_machine = state_machine.execute_state("zero");
        assert_eq!(state_machine.state, "one");

        let state_machine = state_machine.execute_state("one");
        assert_eq!(state_machine.state, "two");

        let state_machine = state_machine.execute_state("two");
        assert_eq!(state_machine.state, "two");
    }
}
