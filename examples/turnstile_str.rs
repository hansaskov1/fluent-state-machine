use fluent_state_machine::StateMachineBuilder;

fn main() {
    let mut turnstyle = StateMachineBuilder::new((), "Locked")
        .state("Locked")
            .on("Coin").go_to("Unlocked")
        .state("Unlocked")
            .on("Push").go_to("Locked")
        .build();

    turnstyle.trigger("Coin");
    println!("State: {}", turnstyle.state);

    turnstyle.trigger("Push");
    println!("State: {}", turnstyle.state);
}