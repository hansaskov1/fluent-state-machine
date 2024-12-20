use fluent_state_machine::{StateMachine, StateMachineBuilder};

#[derive(Copy, Clone, PartialEq, Debug)]
enum State {
    Start,
    Select
}

#[derive(PartialEq)]
enum Event {
    Coin,
    Coke,
    Sprite,
    Refill
}


fn create_soda_machine() -> StateMachine<Event, State, (i32, i32)> {
    use Event::{Coin, Coke, Sprite, Refill};
    use State::{Start, Select};


    let max_coke = 3;
    let max_sprite = 3;

    let container = (max_coke, max_sprite);

    let soda_machine = StateMachineBuilder::new(container, Start)
        .state(Start)
            .on(Coin).go_to(Select)
            .on(Refill)
                .update(|(coke, sprite)| {println!("Pow"); *coke = 3; *sprite = 3;})
        .state(Select)
            .on(Coke).go_to(Start)
                .then(|(coke, _sprite)| if *coke > 0 {
                    *coke -= 1
                })
            .on(Sprite).go_to(Start)
            .then(|(coke, _sprite)| if *coke > 0 {
                *coke -= 1
            })
        .build();

    soda_machine
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_soda_machine() {
       let mut soda_machine = create_soda_machine();

        soda_machine.trigger(Event::Coin);
        soda_machine.trigger(Event::Coke);

        assert_eq!(soda_machine.state, State::Start);
        assert_eq!(soda_machine.store, (2, 3));

        soda_machine.trigger(Event::Coin);
        soda_machine.trigger(Event::Coke);

        assert_eq!(soda_machine.state, State::Start);
        assert_eq!(soda_machine.store, (1, 3));

        soda_machine.trigger(Event::Coin);
        soda_machine.trigger(Event::Coke);

        assert_eq!(soda_machine.state, State::Start);
        assert_eq!(soda_machine.store, (0, 3));

        soda_machine.trigger(Event::Coin);
        soda_machine.trigger(Event::Coke);

        assert_eq!(soda_machine.state, State::Start);
        assert_eq!(soda_machine.store, (0, 3));

        soda_machine.trigger(Event::Refill);

        assert_eq!(soda_machine.state, State::Start);
        assert_eq!(soda_machine.store, (3, 3));
    }
}