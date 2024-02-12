use lib::StateMachineBuilder;

mod lib;


fn main() {


    let mut turnstile = StateMachineBuilder::new((), "Locked")
    .state("Locked")
        .event("Coin", "UnLocked")
        .event("Push", "Locked")
    .state("UnLocked")
        .event("Coin", "UnLocked")
        .event("Push", "Locked")
    .build();

    turnstile.trigger("Coin");
    turnstile.trigger("Push");

    println!("State: {}", turnstile.state);
    println!("Store: {:?}", turnstile.store);

}
