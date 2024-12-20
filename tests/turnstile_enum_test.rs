use fluent_state_machine::{StateMachine, StateMachineBuilder};

#[derive(Debug, Clone, Copy, PartialEq)]
enum States {
    Locked,
    UnLocked
}

#[derive(PartialEq)]
enum Events {
    Coin,
    Push,
}

fn create_locked_turnstile() -> StateMachine<Events, States, ()> {

    use Events::{Coin, Push};
    use States::{Locked, UnLocked};

    StateMachineBuilder::new((), Locked)
    .state(Locked)
        .on(Coin).go_to(UnLocked)
    .state(UnLocked)
        .on(Push).go_to(Locked)
    .build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_locked() {
        let mut sm = create_locked_turnstile();
        sm.trigger(Events::Push);
        assert_eq!(sm.state, States::Locked);  
    }

    #[test]
    fn test_unlocked() {
        let mut sm = create_locked_turnstile();
            sm.trigger(Events::Coin);
            sm.trigger(Events::Push);
        assert_eq!(sm.state, States::Locked);  
    }

    #[test]
    fn test_locked_to_unlocked() {
        let mut sm = create_locked_turnstile();
            sm.trigger(Events::Coin);
        assert_eq!(sm.state, States::UnLocked);  
    }

    #[test]
    fn test_unlocked_to_locked() {
        let mut sm = create_locked_turnstile();
            sm.trigger(Events::Coin);
            sm.trigger(Events::Push);
            sm.trigger(Events::Push);
        assert_eq!(sm.state, States::Locked);  
    }

}
