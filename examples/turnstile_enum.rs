use fluent_state_machine::StateMachineBuilder;

#[derive(Debug, Clone, Copy, PartialEq)]
enum States {
    Locked,
    UnLocked
}

#[derive(PartialEq)]
enum Event {
    Coin,
    Push,
}

fn main() {

    use Event::{Coin, Push};
    use States::{Locked,  UnLocked};
    
    let mut turnstyle = StateMachineBuilder::new((), Locked)
        .state(Locked)
            .on(Coin).go_to(UnLocked)
        .state(UnLocked)
            .on(Push).go_to(Locked)
        .build();

    turnstyle.trigger(Coin);
    println!("State: {:?}", turnstyle.state);

    turnstyle.trigger(Push);
    println!("State: {:?}", turnstyle.state);
}