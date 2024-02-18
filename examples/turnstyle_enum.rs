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

    use State::{Locked, UnLocked};
    use Event::{Coin, Push};
    
    let mut turnstyle = StateMachineBuilder::new((), Locked)
        .state(Locked)
            .on(Coin).go_to(UnLocked)
        .state(UnLocked)
            .on(Push).go_to(Locked)
        .build().unwrap();

    turnstyle.trigger(Coin);
    println!("State: {:?}", turnstyle.state);

    turnstyle.trigger(Push);
    println!("State: {:?}", turnstyle.state);
}