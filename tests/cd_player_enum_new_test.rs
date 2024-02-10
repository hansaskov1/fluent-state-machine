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
        .trigger(Triggers::Play, States::Playing).only_if(|track| *track > 0 )
        .trigger(Triggers::Forward, States::Stopped).update(|track| *track += 1 )
        .trigger(Triggers::Backward, States::Stopped).update(|track| *track -= 1)
    .state(States::Playing)
        .trigger(Triggers::Stop, States::Stopped).update(|track| *track = 0)
        .trigger(Triggers::Pause, States::Paused)
    .state(States::Paused)
        .trigger(Triggers::Play, States::Playing)
        .trigger(Triggers::Stop, States::Stopped).update(|track| *track = 0)
        .trigger(Triggers::Forward, States::Paused).update(|track| *track += 1)
        .trigger(Triggers::Backward, States::Paused).update(|track| *track -= 1)
    .build()
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_press_play_with_no_track() {
        let st = create_cd_player()
        .trigger(Triggers::Play);
        assert_eq!(st.state, States::Stopped);  
        assert_eq!(st.store, 0);          
    }

    #[test]    
    fn test_press_forward_when_playing() {
        let st = create_cd_player()
        .trigger(Triggers::Forward)
        .trigger(Triggers::Play)
        .trigger(Triggers::Forward);
        assert_eq!(st.state, States::Playing);  
        assert_eq!(st.store, 1);          
    }

    #[test]
    fn test_press_pause_when_playing() {
        let st = create_cd_player()
            .trigger(Triggers::Forward)
            .trigger(Triggers::Play)
            .trigger(Triggers::Pause);
        assert_eq!(st.state, States::Paused);
        assert_eq!(st.store, 1);
    }

    #[test]
    fn test_press_stop_when_paused() {
        let st = create_cd_player()
            .trigger(Triggers::Forward)
            .trigger(Triggers::Play)
            .trigger(Triggers::Pause)
            .trigger(Triggers::Stop);
        assert_eq!(st.state, States::Stopped);
        assert_eq!(st.store, 0);
    }

    #[test]
    fn test_press_backward_when_paused() {
        let st = create_cd_player()
            .trigger(Triggers::Forward)
            .trigger(Triggers::Play)
            .trigger(Triggers::Pause)
            .trigger(Triggers::Backward);
        assert_eq!(st.state, States::Paused);
        assert_eq!(st.store, 0);
    }


    #[test]
    fn test_press_play_when_paused() {
        let st = create_cd_player()
            .trigger(Triggers::Forward)
            .trigger(Triggers::Play)
            .trigger(Triggers::Pause)
            .trigger(Triggers::Forward)
            .trigger(Triggers::Play);
        assert_eq!(st.state, States::Playing);
        assert_eq!(st.store, 2);
    }

        #[test]
    fn test_complex_sequence() {
        let mut st = create_cd_player();

        // Start with no track
        assert_eq!(st.state, States::Stopped);
        assert_eq!(st.store, 0);

        // Try to play with no track
        st = st.trigger(Triggers::Play);
        assert_eq!(st.state, States::Stopped);
        assert_eq!(st.store, 0);

        // Forward to first track
        st = st.trigger(Triggers::Forward);
        assert_eq!(st.state, States::Stopped);
        assert_eq!(st.store, 1);

        // Play first track
        st = st.trigger(Triggers::Play);
        assert_eq!(st.state, States::Playing);
        assert_eq!(st.store, 1);

        // Pause first track
        st = st.trigger(Triggers::Pause);
        assert_eq!(st.state, States::Paused);
        assert_eq!(st.store, 1);

        // Forward to second track while paused
        st = st.trigger(Triggers::Forward);
        assert_eq!(st.state, States::Paused);
        assert_eq!(st.store, 2);

        // Play second track
        st = st.trigger(Triggers::Play);
        assert_eq!(st.state, States::Playing);
        assert_eq!(st.store, 2);

        // Try Backward to first track while playing
        st = st.trigger(Triggers::Backward);
        assert_eq!(st.state, States::Playing);
        assert_eq!(st.store, 2);

        // Stop while playing
        st = st.trigger(Triggers::Stop);
        assert_eq!(st.state, States::Stopped);
        assert_eq!(st.store, 0);

        // Forward to first track
        st = st.trigger(Triggers::Forward);
        assert_eq!(st.state, States::Stopped);
        assert_eq!(st.store, 1);

        // Play first track
        st = st.trigger(Triggers::Play);
        assert_eq!(st.state, States::Playing);
        assert_eq!(st.store, 1);
    }

}
