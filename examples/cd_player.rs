use state_machine_dsl::StateMachineBuilder;

fn main() {

    // Create store for state machine. In this case it is an integer
    let track = 0;
    
    let mut cd_player = StateMachineBuilder::new( track, "Stopped")
    .state("Stopped")
        .when("Play").to("Playing").condition(|track| *track > 0 )
        .when("Forward").before_condition(|track| *track += 1 )
        .when("Backward").before_condition(|track| *track -= 1)
    .state("Playing")
        .when("Stop").to("Stopped").after_condition(|track| *track = 0)
        .when("Pause").to("Paused")
    .state("Paused")
        .when("Play").to("Playing")
        .when("Stop").to("Stopped").after_condition(|track| *track = 0)
        .when("Forward").before_condition(|track| *track += 1)
        .when("Backward").before_condition(|track| *track -= 1)
    .build()
    .unwrap();

    println!("Track: {}, State: {}", cd_player.store, cd_player.state);

    cd_player.trigger("Forward");
    println!("Track: {}, State: {}", cd_player.store, cd_player.state);

    cd_player.trigger("Play");
    println!("Track: {}, State: {}", cd_player.store, cd_player.state);
}