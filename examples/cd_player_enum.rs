use fluent_state_machine::StateMachineBuilder;

#[derive(Debug, Clone, Copy, PartialEq)]
enum State {
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

fn main() {
    // Using the Event and State namespace
    use Event::{Backward, Forward, Pause, Play, Stop};
    use State::{Playing, Stopped, Paused};

    // Create store for state machine. In this case it is an integer
    let track = 0;
    
    let mut cd_player =  StateMachineBuilder::new( track, Stopped)
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
        .build();

    println!("Track: {}, State: {:?}", cd_player.store, cd_player.state);

    cd_player.trigger(Forward);
    println!("Track: {}, State: {:?}", cd_player.store, cd_player.state);

    cd_player.trigger(Play);
    println!("Track: {}, State: {:?}", cd_player.store, cd_player.state);
}