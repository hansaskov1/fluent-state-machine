use state_machine_dsl::{StateMachine, StateMachineBuilder};



fn create_cd_player() -> StateMachine<&'static str , &'static str, i32> {

    // Create store for state machine. In this case it is an integer
    let track = 0;
    
    // Construct state machine and return the result. 
    StateMachineBuilder::new( track, "Stopped")
        .state("Stopped")
            .on("Play").go_to("Playing").only_if(|track| *track > 0 )
            .on("Forward").update(|track| *track += 1 )
            .on("Backward").update(|track| *track -= 1)
        .state("Playing")
            .on("Stop").go_to("Stopped").update_after(|track| *track = 0)
            .on("Pause").go_to("Paused")
        .state("Paused")
            .on("Play").go_to("Playing")
            .on("Stop").go_to("Stopped").update_after(|track| *track = 0)
            .on("Forward").update(|track| *track += 1)
            .on("Backward").update(|track| *track -= 1)
        .build()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_press_play_with_no_track() {
        let mut cd = create_cd_player();
        cd.trigger("Play");
        assert_eq!(cd.state, "Stopped");  
        assert_eq!(cd.store, 0);          
    }

    #[test]    
    fn test_press_forward_when_playing() {
        let mut cd = create_cd_player();
        cd.trigger("Forward");
        cd.trigger("Play");
        cd.trigger("Forward");
        assert_eq!(cd.state, "Playing");  
        assert_eq!(cd.store, 1);          
    }

    #[test]
    fn test_press_pause_when_playing() {
        let mut cd = create_cd_player();
            cd.trigger("Forward");
            cd.trigger("Play");
            cd.trigger("Pause");
        assert_eq!(cd.state, "Paused");
        assert_eq!(cd.store, 1);
    }

    #[test]
    fn test_press_stop_when_paused() {
        let mut cd = create_cd_player();
            cd.trigger("Forward");
            cd.trigger("Play");
            cd.trigger("Pause");
            cd.trigger("Stop");
        assert_eq!(cd.state, "Stopped");
        assert_eq!(cd.store, 0);
    }

    #[test]
    fn test_press_backward_when_paused() {
        let mut cd = create_cd_player();
            cd.trigger("Forward");
            cd.trigger("Play");
            cd.trigger("Pause");
            cd.trigger("Backward");
        assert_eq!(cd.state, "Paused");
        assert_eq!(cd.store, 0);
    }

    #[test]
    fn test_press_play_when_paused() {
        let mut cd = create_cd_player();
            cd.trigger("Forward");
            cd.trigger("Play");
            cd.trigger("Pause");
            cd.trigger("Forward");
            cd.trigger("Play");
        assert_eq!(cd.state, "Playing");
        assert_eq!(cd.store, 2);
    }

    #[test]
    fn test_complex_sequence() {
        let mut cd = create_cd_player();

        // Start with no track
        assert_eq!(cd.state, "Stopped");
        assert_eq!(cd.store, 0);

        // Try to play with no track
        cd.trigger("Play");
        assert_eq!(cd.state, "Stopped");
        assert_eq!(cd.store, 0);

        // Forward to first track
        cd.trigger("Forward");
        assert_eq!(cd.state, "Stopped");
        assert_eq!(cd.store, 1);

        // Play first track
        cd.trigger("Play");
        assert_eq!(cd.state, "Playing");
        assert_eq!(cd.store, 1);

        // Pause first track
        cd.trigger("Pause");
        assert_eq!(cd.state, "Paused");
        assert_eq!(cd.store, 1);

        // Forward to second track while paused
        cd.trigger("Forward");
        assert_eq!(cd.state, "Paused");
        assert_eq!(cd.store, 2);

        // Play second track
        cd.trigger("Play");
        assert_eq!(cd.state, "Playing");
        assert_eq!(cd.store, 2);
    }
}