use state_machine_dsl::{StateMachine, StateMachineBuilder};



#[derive(Debug, Clone, Copy, PartialEq)]
enum States {
    Locked,
    UnLocked
}

#[derive(PartialEq)]
enum Triggers {
    Coin,
    Push,
}



fn create_locked_turnstile() -> StateMachine<Triggers, States, ()> {

    StateMachineBuilder::new((), States::Locked)
    .state(States::Locked)
        .event(Triggers::Coin, States::UnLocked)
        .event(Triggers::Push, States::Locked)
    .state(States::UnLocked)
        .event(Triggers::Coin, States::UnLocked)
        .event(Triggers::Push, States::Locked)
    .build()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_locked() {
        let mut sm = create_locked_turnstile();
        sm.trigger(Triggers::Push);
        assert_eq!(sm.state, States::Locked);  
    }

    #[test]
    fn test_unlocked() {
        let mut sm = create_locked_turnstile();
            sm.trigger(Triggers::Coin);
            sm.trigger(Triggers::Push);
        assert_eq!(sm.state, States::Locked);  
    }

    #[test]
    fn test_locked_to_unlocked() {
        let mut sm = create_locked_turnstile();
            sm.trigger(Triggers::Coin);
        assert_eq!(sm.state, States::UnLocked);  
    }

    #[test]
    fn test_unlocked_to_locked() {
        let mut sm = create_locked_turnstile();
            sm.trigger(Triggers::Coin);
            sm.trigger(Triggers::Push);
            sm.trigger(Triggers::Push);
        assert_eq!(sm.state, States::Locked);  
    }

}
