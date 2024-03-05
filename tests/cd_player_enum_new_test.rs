use fluent_state_machine::{StateMachine, StateMachineBuilder};


#[derive(Debug, Clone, Copy, PartialEq)]
enum States {
    Stopped,
    Playing,
    Paused,
}

#[derive(PartialEq)]
enum Event {
    Play,
    Stop,
    Pause,
    Backward,
    Forward,
}


fn create_cd_player() -> StateMachine<Event, States, i32> {

    use Event::{Backward, Forward, Pause, Play, Stop};
    use States::{Playing, Stopped, Paused};

    // Create a u32 store for state machine. It could be any type you want
    let track = 0;

    // Construct state machine.
    StateMachineBuilder::new( track, Stopped)
        .state(Stopped)
            .on(Play).go_to(Playing).only_if(|track| *track > 0 )
            .on(Forward).update(|track| *track += 1 )
            .on(Backward).update(|track| *track -= 1)
        .state(Playing)
            .on(Stop).go_to(Stopped).then(|track| *track = 0)
            .on(Pause).go_to(Paused)
        .state(Paused)
            .on(Play).go_to(Playing)
            .on(Stop).go_to(Stopped).then(|track| *track = 0)
            .on(Forward).update(|track| *track += 1)
            .on(Backward).update(|track| *track -= 1)
        .build()
        .unwrap()
}



#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_press_play_with_no_track() {
        let mut cd = create_cd_player();
        cd.trigger(Event::Play);
        assert_eq!(cd.state, States::Stopped);  
        assert_eq!(cd.store, 0);          
    }

    #[test]    
    fn test_press_forward_when_playing() {
        let mut cd = create_cd_player();
        cd.trigger(Event::Forward);
        cd.trigger(Event::Play);
        cd.trigger(Event::Forward);
        assert_eq!(cd.state, States::Playing);  
        assert_eq!(cd.store, 1);          
    }

    #[test]
    fn test_press_pause_when_playing() {
        let mut cd = create_cd_player();
            cd.trigger(Event::Forward);
            cd.trigger(Event::Play);
            cd.trigger(Event::Pause);
        assert_eq!(cd.state, States::Paused);
        assert_eq!(cd.store, 1);
    }

    #[test]
    fn test_press_stop_when_paused() {
        let mut cd = create_cd_player();
            cd.trigger(Event::Forward);
            cd.trigger(Event::Play);
            cd.trigger(Event::Pause);
            cd.trigger(Event::Stop);
        assert_eq!(cd.state, States::Stopped);
        assert_eq!(cd.store, 0);
    }

    #[test]
    fn test_press_backward_when_paused() {
        let mut cd = create_cd_player();
            cd.trigger(Event::Forward);
            cd.trigger(Event::Play);
            cd.trigger(Event::Pause);
            cd.trigger(Event::Backward);
        assert_eq!(cd.state, States::Paused);
        assert_eq!(cd.store, 0);
    }


    #[test]
    fn test_press_play_when_paused() {
        let mut cd = create_cd_player();
            cd.trigger(Event::Forward);
            cd.trigger(Event::Play);
            cd.trigger(Event::Pause);
            cd.trigger(Event::Forward);
            cd.trigger(Event::Play);
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
        cd.trigger(Event::Play);
        assert_eq!(cd.state, States::Stopped);
        assert_eq!(cd.store, 0);

        // Forward to first track
        cd.trigger(Event::Forward);
        assert_eq!(cd.state, States::Stopped);
        assert_eq!(cd.store, 1);

        // Play first track
        cd.trigger(Event::Play);
        assert_eq!(cd.state, States::Playing);
        assert_eq!(cd.store, 1);

        // Pause first track
        cd.trigger(Event::Pause);
        assert_eq!(cd.state, States::Paused);
        assert_eq!(cd.store, 1);

        // Forward to second track while paused
        cd.trigger(Event::Forward);
        assert_eq!(cd.state, States::Paused);
        assert_eq!(cd.store, 2);

        // Play second track
        cd.trigger(Event::Play);
        assert_eq!(cd.state, States::Playing);
        assert_eq!(cd.store, 2);

        // Try Backward to first track while playing
        cd.trigger(Event::Backward);
        assert_eq!(cd.state, States::Playing);
        assert_eq!(cd.store, 2);

        // Stop while playing
        cd.trigger(Event::Stop);
        assert_eq!(cd.state, States::Stopped);
        assert_eq!(cd.store, 0);

        // Forward to first track
        cd.trigger(Event::Forward);
        assert_eq!(cd.state, States::Stopped);
        assert_eq!(cd.store, 1);

        // Play first track
        cd.trigger(Event::Play);
        assert_eq!(cd.state, States::Playing);
        assert_eq!(cd.store, 1);
    }

}
