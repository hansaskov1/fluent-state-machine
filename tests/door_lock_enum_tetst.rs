use std::time::Duration;
use fluent_state_machine::{StateMachine, StateMachineBuilder};


#[derive(Debug, Clone, Copy, PartialEq)]
enum States {
    Locked,
    Unlocked,
    Locking,
    Unlocking
}

#[derive(PartialEq)]
enum Event {
    OpenDoor,
    Step
}
#[derive(PartialEq)]
enum LockSensor {
    Locked,
    Unlocked
}

#[derive(PartialEq)]
enum DoorSensor {
    Open,
    Closed
}

struct Store {
    lock_sensor: LockSensor,
    door_sensor: DoorSensor,
    duration_in_state: Duration,
}




fn create_lock() -> StateMachine<Event, States, Store> {
    // Create a u32 store for state machine. It could be any type you want
    let store = Store {
        lock_sensor: LockSensor::Locked,
        door_sensor: DoorSensor::Closed,
        duration_in_state: Duration::from_secs(0),
    };


    // Construct state machine.
    StateMachineBuilder::new( store, States::Locked)
    .set_global_action(|store| store.duration_in_state = Duration::from_secs(0))
        .state(States::Locked)
            .on(Event::OpenDoor)
                .go_to(States::Unlocking)
        .state(States::Unlocking)
            .on(Event::Step)
                .go_to(States::Unlocked)
                .only_if(|store| store.lock_sensor == LockSensor::Unlocked)
            .on(Event::Step)
                .go_to(States::Locked)
                .only_if(|store| store.duration_in_state > Duration::from_secs(10))
        .state(States::Unlocked)
            .on(Event::Step)
                .go_to(States::Locking)
                .only_if(|store| store.door_sensor == DoorSensor::Closed && store.duration_in_state > Duration::from_secs(5))
        .state(States::Locking)
            .on(Event::Step)
                .go_to(States::Locked)
                .only_if(|store| store.lock_sensor == LockSensor::Locked)
        .build()
        .unwrap()
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_door() {
        let mut door_lock = create_lock();

        door_lock.trigger(Event::OpenDoor);
        assert_eq!(door_lock.state, States::Unlocking);

        door_lock.trigger(Event::Step);
        assert_eq!(door_lock.state, States::Unlocking);

        door_lock.store.lock_sensor = LockSensor::Unlocked;
        door_lock.trigger(Event::Step);
        assert_eq!(door_lock.state, States::Unlocked);

        door_lock.trigger(Event::Step);
        assert_eq!(door_lock.state, States::Unlocked);

        door_lock.store.door_sensor = DoorSensor::Open;
        door_lock.trigger(Event::Step);
        assert_eq!(door_lock.state, States::Unlocked);

        door_lock.store.duration_in_state = Duration::from_secs(6);
        door_lock.trigger(Event::Step);
        assert_eq!(door_lock.state, States::Unlocked);

        door_lock.store.door_sensor = DoorSensor::Closed;
        door_lock.trigger(Event::Step);
        assert_eq!(door_lock.state, States::Locking);

        door_lock.trigger(Event::Step);
        assert_eq!(door_lock.state, States::Locking);

        door_lock.store.lock_sensor = LockSensor::Locked;
        door_lock.trigger(Event::Step);
        assert_eq!(door_lock.state, States::Locked);

    }

    #[test]
    fn test_unlocking_to_locked_due_to_timeout() {
        let mut door_lock = create_lock();

        // Transition to Unlocking state
        door_lock.trigger(Event::OpenDoor);
        assert_eq!(door_lock.state, States::Unlocking);

        // Simulate time passing without the lock sensor detecting an unlocked state
        door_lock.store.duration_in_state = Duration::from_secs(11);

        // Trigger a step event
        door_lock.trigger(Event::Step);

        // Check that the state has transitioned to Locked due to timeout
        assert_eq!(door_lock.state, States::Locked);
    }



}
