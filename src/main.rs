use lib::StateMachineBuilder;
mod lib;


#[derive(Copy, Clone, PartialEq, Debug)]
enum State {
    Start,
    Select
}

#[derive(PartialEq)]
enum Trigger {
    Coin,
    Coke,
    Sprite,
    Refill
}


fn main() {

    let max_coke = 3;
    let max_sprite = 3;

    let container = (max_coke, max_sprite);

    let mut soda_machine = StateMachineBuilder::new(container, State::Start)
    .state(State::Start)
        .event(Trigger::Coin, State::Select)
        .event(Trigger::Refill, State::Start)
            .before_condition(|(coke, sprite)| {println!("Pow"); *coke = 3; *sprite = 3;})
    .state(State::Select)
        .event(Trigger::Coke, State::Start)
            .after_condition(|(coke, sprite)| if *coke > 0 {
                *coke -= 1
            })
        .event(Trigger::Sprite, State::Start)
        .after_condition(|(coke, sprite)| if *coke > 0 {
            *coke -= 1
        })
    .build();


    soda_machine.trigger(Trigger::Coin);
    soda_machine.trigger(Trigger::Coke);

    println!("State: {:?}", soda_machine.state);
    println!("Store: {:?}", soda_machine.store);

    soda_machine.trigger(Trigger::Coin);
    soda_machine.trigger(Trigger::Coke);

    println!("State: {:?}", soda_machine.state);
    println!("Store: {:?}", soda_machine.store);

    soda_machine.trigger(Trigger::Coin);
    soda_machine.trigger(Trigger::Coke);

    println!("State: {:?}", soda_machine.state);
    println!("Store: {:?}", soda_machine.store);

    soda_machine.trigger(Trigger::Coin);
    soda_machine.trigger(Trigger::Coke);

    println!("State: {:?}", soda_machine.state);
    println!("Store: {:?}", soda_machine.store);

    soda_machine.trigger(Trigger::Refill);

    println!("State: {:?}", soda_machine.state);
    println!("Store: {:?}", soda_machine.store);


}
