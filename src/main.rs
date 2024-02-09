use crate::lib::StateMachine;

mod lib;


#[derive(Default, Debug, Clone, Copy, PartialEq)]
enum State {
    #[default]
    Idle,
    Running,
    Paused,
}

fn main() {
    let store = 0;

    let state_machine = StateMachine::new(store, State::Idle)
        .transition(State::Idle, State::Paused, |store| {
            *store += 1;
            *store == 2
        })
        .execute_state(State::Idle);

    println!("{:?}", state_machine.store);
    println!("{:?}", state_machine.state);
}
