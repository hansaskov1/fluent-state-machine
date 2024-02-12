use state_machine_dsl::{StateMachine, StateMachineBuilder};


#[derive(Debug, Clone, Copy, PartialEq)]
enum States {
    Stopped,
    Playing,
    Paused,
}

#[derive(PartialEq)]
enum Triggers {
    Play,
    Stop,
    Pause,
    Backward,
    Forward,
}

fn create_cd_player() -> StateMachine<Triggers, States, i32> {

    // Create a new state machine
    StateMachineBuilder::new(0, States::Stopped)
    .state(States::Stopped)
        .event(Triggers::Play, States::Playing).condition(|track| *track > 0 )
        .event(Triggers::Forward, States::Stopped).before_trigger(|track| *track += 1 )
        .event(Triggers::Backward, States::Stopped).before_trigger(|track| *track -= 1)
    .state(States::Playing)
        .event(Triggers::Stop, States::Stopped).before_trigger(|track| *track = 0)
        .event(Triggers::Pause, States::Paused)
    .state(States::Paused)
        .event(Triggers::Play, States::Playing)
        .event(Triggers::Stop, States::Stopped).before_trigger(|track| *track = 0)
        .event(Triggers::Forward, States::Paused).before_trigger(|track| *track += 1)
        .event(Triggers::Backward, States::Paused).before_trigger(|track| *track -= 1)
    .build()
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_press_play_with_no_track() {
        let mut cd = create_cd_player();
        cd.trigger(Triggers::Play);
        assert_eq!(cd.state, States::Stopped);  
        assert_eq!(cd.store, 0);          
    }

    #[test]    
    fn test_press_forward_when_playing() {
        let mut cd = create_cd_player();
        cd.trigger(Triggers::Forward);
        cd.trigger(Triggers::Play);
        cd.trigger(Triggers::Forward);
        assert_eq!(cd.state, States::Playing);  
        assert_eq!(cd.store, 1);          
    }

    #[test]
    fn test_press_pause_when_playing() {
        let mut cd = create_cd_player();
            cd.trigger(Triggers::Forward);
            cd.trigger(Triggers::Play);
            cd.trigger(Triggers::Pause);
        assert_eq!(cd.state, States::Paused);
        assert_eq!(cd.store, 1);
    }

    #[test]
    fn test_press_stop_when_paused() {
        let mut cd = create_cd_player();
            cd.trigger(Triggers::Forward);
            cd.trigger(Triggers::Play);
            cd.trigger(Triggers::Pause);
            cd.trigger(Triggers::Stop);
        assert_eq!(cd.state, States::Stopped);
        assert_eq!(cd.store, 0);
    }

    #[test]
    fn test_press_backward_when_paused() {
        let mut cd = create_cd_player();
            cd.trigger(Triggers::Forward);
            cd.trigger(Triggers::Play);
            cd.trigger(Triggers::Pause);
            cd.trigger(Triggers::Backward);
        assert_eq!(cd.state, States::Paused);
        assert_eq!(cd.store, 0);
    }


    #[test]
    fn test_press_play_when_paused() {
        let mut cd = create_cd_player();
            cd.trigger(Triggers::Forward);
            cd.trigger(Triggers::Play);
            cd.trigger(Triggers::Pause);
            cd.trigger(Triggers::Forward);
            cd.trigger(Triggers::Play);
        assert_eq!(cd.state, States::Playing);
        assert_eq!(cd.store, 2);
    }

    #[test]
    fn test_complex_sequence() {
        let mut cd = create_cd_player();

        // Start with no track
        assert_eq!(cd.state, States::Stopped);
        assert_eq!(cd.store, 0);

        // Try to play with no track
        cd.trigger(Triggers::Play);
        assert_eq!(cd.state, States::Stopped);
        assert_eq!(cd.store, 0);

        // Forward to first track
        cd.trigger(Triggers::Forward);
        assert_eq!(cd.state, States::Stopped);
        assert_eq!(cd.store, 1);

        // Play first track
        cd.trigger(Triggers::Play);
        assert_eq!(cd.state, States::Playing);
        assert_eq!(cd.store, 1);

        // Pause first track
        cd.trigger(Triggers::Pause);
        assert_eq!(cd.state, States::Paused);
        assert_eq!(cd.store, 1);

        // Forward to second track while paused
        cd.trigger(Triggers::Forward);
        assert_eq!(cd.state, States::Paused);
        assert_eq!(cd.store, 2);

        // Play second track
        cd.trigger(Triggers::Play);
        assert_eq!(cd.state, States::Playing);
        assert_eq!(cd.store, 2);

        // Try Backward to first track while playing
        cd.trigger(Triggers::Backward);
        assert_eq!(cd.state, States::Playing);
        assert_eq!(cd.store, 2);

        // Stop while playing
        cd.trigger(Triggers::Stop);
        assert_eq!(cd.state, States::Stopped);
        assert_eq!(cd.store, 0);

        // Forward to first track
        cd.trigger(Triggers::Forward);
        assert_eq!(cd.state, States::Stopped);
        assert_eq!(cd.store, 1);

        // Play first track
        cd.trigger(Triggers::Play);
        assert_eq!(cd.state, States::Playing);
        assert_eq!(cd.store, 1);
    }

}
