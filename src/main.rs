use lib::StateMachineBuilder;
mod lib;


// Example. 
fn main() {

    let mut turnstile = StateMachineBuilder::new((), "Locked")
    .state("Locked")
        .event("Coin", "UnLocked")
    .state("UnLocked")
        .event("Push", "Locked")
    .build();

    turnstile.trigger("Coin");

    println!("State: {}", turnstile.state);
}
