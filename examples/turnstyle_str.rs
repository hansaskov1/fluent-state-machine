use state_machine_dsl::StateMachineBuilder;

fn main() {
    let mut turnstyle = StateMachineBuilder::new((), "Locked")
        .state("Locked")
            .when("Coin").to("Unlocked")
        .state("Unlocked")
            .when("Push").to("Locked")
        .build().unwrap();

    turnstyle.trigger("Coin");
    println!("State: {}", turnstyle.state);

    turnstyle.trigger("Push");
    println!("State: {}", turnstyle.state);
}