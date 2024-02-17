#[cfg(test)]
mod build_tests {
    use state_machine_dsl::StateMachineBuilder;

    use super::*;

    #[test]
    fn test_default_case() {
        let sm = StateMachineBuilder::new((), "State1")
            .state("State1")
                .event("switch", "State2")
            .state("State2")
                .event("switch", "State1")
            .build();

        assert!(sm.is_ok(), "Expected Ok, got Err");
    }

    #[test]
    fn test_non_unique_states() {
        let sm = StateMachineBuilder::new((), "State1")
            .state("State1")
                .event("switch", "State2")
            .state("State2")
                .event("switch", "State1")
            .state("State1")
                .event("switch", "State2")
            .build();

        assert!(sm.is_err(), "Expected Ok, got Err");
    }

    #[test]
    fn test_non_unique_states_2() {
        let sm = StateMachineBuilder::new((), "State1")
            .state("State1")
                .event("switch", "State2")
                .event("switch", "State2")
            .state("State2")
                .event("switch", "State1")                
            .build();

        assert!(sm.is_err(), "Expected Ok, got Err");
    }

}