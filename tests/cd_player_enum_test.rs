use state_machine_dsl::StateMachine;


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
    StateMachine::new(0, States::Stopped)
    // Stopped
    .transition(Triggers::Play ,States::Stopped, States::Playing, |track| *track > 0 )
    .transition(Triggers::Forward ,States::Stopped, States::Stopped, |track| {*track += 1; false })
    .transition(Triggers::Backward ,States::Stopped, States::Stopped, |track| {*track -= 1; false })
    // Playing
    .transition(Triggers::Stop ,States::Playing, States::Stopped, |track| {*track = 0; true })
    .transition(Triggers::Pause ,States::Playing, States::Paused, |_| true )
    // Paused
    .transition(Triggers::Play ,States::Paused, States::Playing, |_| true )
    .transition(Triggers::Stop ,States::Paused, States::Stopped, |track| {*track = 0; true })
    .transition(Triggers::Forward ,States::Paused, States::Paused, |track| {*track += 1; false })
    .transition(Triggers::Backward ,States::Paused, States::Paused, |track| {*track -= 1; false })
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
