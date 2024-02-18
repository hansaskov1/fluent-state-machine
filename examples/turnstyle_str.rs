use state_machine_dsl::StateMachineBuilder;

fn main() {
    let mut turnstyle = StateMachineBuilder::new((), "Locked")
        .state("Locked")
            .on("Coin").go_to("Unlocked")
        .state("Unlocked")
            .on("Push").go_to("Locked")
        .build().unwrap();

    turnstyle.trigger("Coin");
    println!("State: {}", turnstyle.state);

    turnstyle.trigger("Push");
    println!("State: {}", turnstyle.state);
}