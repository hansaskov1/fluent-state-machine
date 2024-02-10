use state_machine_dsl::{StateMachine, StateMachineBuilder};

fn create_cd_player() -> StateMachine<&'static str , &'static str, i32> {

    // Create a new state machine
    StateMachineBuilder::new(0, "Stopped")
    .state("Stopped")
        .trigger("Play", "Playing").only_if(|track| *track > 0 )
        .trigger("Forward", "Stopped").update(|track| *track += 1 )
        .trigger("Backward", "Stopped").update(|track| *track -= 1)
    .state("Playing")
        .trigger("Stop", "Stopped").update(|track| *track = 0)
        .trigger("Pause", "Paused")
    .state("Paused")
        .trigger("Play", "Playing")
        .trigger("Stop", "Stopped").update(|track| *track = 0)
        .trigger("Forward", "Paused").update(|track| *track += 1)
        .trigger("Backward", "Paused").update(|track| *track -= 1)
    .build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_press_play_with_no_track() {
        let st = create_cd_player()
        .trigger("Play");
        assert_eq!(st.state, "Stopped");  
        assert_eq!(st.store, 0);          
    }

    #[test]    
    fn test_press_forward_when_playing() {
        let st = create_cd_player()
        .trigger("Forward")
        .trigger("Play")
        .trigger("Forward");
        assert_eq!(st.state, "Playing");  
        assert_eq!(st.store, 1);          
    }

    #[test]
    fn test_press_pause_when_playing() {
        let st = create_cd_player()
            .trigger("Forward")
            .trigger("Play")
            .trigger("Pause");
        assert_eq!(st.state, "Paused");
        assert_eq!(st.store, 1);
    }

    #[test]
    fn test_press_stop_when_paused() {
        let st = create_cd_player()
            .trigger("Forward")
            .trigger("Play")
            .trigger("Pause")
            .trigger("Stop");
        assert_eq!(st.state, "Stopped");
        assert_eq!(st.store, 0);
    }

    #[test]
    fn test_press_backward_when_paused() {
        let st = create_cd_player()
            .trigger("Forward")
            .trigger("Play")
            .trigger("Pause")
            .trigger("Backward");
        assert_eq!(st.state, "Paused");
        assert_eq!(st.store, 0);
    }

    #[test]
    fn test_press_play_when_paused() {
        let st = create_cd_player()
            .trigger("Forward")
            .trigger("Play")
            .trigger("Pause")
            .trigger("Forward")
            .trigger("Play");
        assert_eq!(st.state, "Playing");
        assert_eq!(st.store, 2);
    }

    #[test]
    fn test_complex_sequence() {
        let mut st = create_cd_player();

        // Start with no track
        assert_eq!(st.state, "Stopped");
        assert_eq!(st.store, 0);

        // Try to play with no track
        st = st.trigger("Play");
        assert_eq!(st.state, "Stopped");
        assert_eq!(st.store, 0);

        // Forward to first track
        st = st.trigger("Forward");
        assert_eq!(st.state, "Stopped");
        assert_eq!(st.store, 1);

        // Play first track
        st = st.trigger("Play");
        assert_eq!(st.state, "Playing");
        assert_eq!(st.store, 1);

        // Pause first track
        st = st.trigger("Pause");
        assert_eq!(st.state, "Paused");
        assert_eq!(st.store, 1);

        // Forward to second track while paused
        st = st.trigger("Forward");
        assert_eq!(st.state, "Paused");
        assert_eq!(st.store, 2);

        // Play second track
        st = st.trigger("Play");
        assert_eq!(st.state, "Playing");
        assert_eq!(st.store, 2);
    }
}