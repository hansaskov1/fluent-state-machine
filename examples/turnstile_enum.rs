use state_machine_dsl::StateMachineBuilder;

#[derive(Debug, Clone, Copy, PartialEq)]
enum State {
    Locked,
    UnLocked
}

#[derive(PartialEq)]
enum Event {
    Coin,
    Push,
}

fn main() {
    
    let mut turnstyle = StateMachineBuilder::new((), State::Locked)
        .state(State::Locked)
            .on(Event::Coin).go_to(State::UnLocked)
        .state(State::UnLocked)
            .on(Event::Push).go_to(State::Locked)
        .build().unwrap();

    turnstyle.trigger(Event::Coin);
    println!("State: {:?}", turnstyle.state);

    turnstyle.trigger(Event::Push);
    println!("State: {:?}", turnstyle.state);
}