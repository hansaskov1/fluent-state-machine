use fluent_state_machine::{StateMachine, StateMachineBuilder};

#[derive(Debug, Clone, Copy, PartialEq)]
enum LightStates {
    Off,
    On,
    Dimmed,
    Broken,
}

#[derive(PartialEq, Debug)]
enum LightEvents {
    TurnOn,
    TurnOff,
    Dim,
    Break,
    Repair,
}

#[derive(Debug, Default)]
struct LightStore {
    brightness: u8,
    is_functional: bool,
    power_cycles: u32,
}

fn create_smart_light() -> StateMachine<LightEvents, LightStates, LightStore> {
    use LightEvents::*;
    use LightStates::*;

    StateMachineBuilder::new(
        LightStore { 
            brightness: 0, 
            is_functional: true, 
            power_cycles: 0 
        }, 
        Off
    )
    .set_global_action(|store, state, event| {
        println!("Transitioned to {:?} via {:?}", state, event);
        if matches!(event, TurnOn | TurnOff) {
            store.power_cycles += 1;
        }
    })
    .state(Off)
        .on(TurnOn)
            .go_to(On)
            .update(|store| store.brightness = 100)
            .only_if(|store| store.is_functional)
        .on(Break)
            .go_to(Broken)
            .then(|store| store.is_functional = false)
    .state(On)
        .on(TurnOff)
            .go_to(Off)
            .update(|store| store.brightness = 0)
        .on(Dim)
            .go_to(Dimmed)
            .update(|store| store.brightness = 30)
        .on(Break)
            .go_to(Broken)
            .then(|store| store.is_functional = false)
    .state(Dimmed)
        .on(TurnOff)
            .go_to(Off)
            .update(|store| store.brightness = 0)
        .on(TurnOn)
            .go_to(On)
            .update(|store| store.brightness = 100)
        .on(Break)
            .go_to(Broken)
            .then(|store| store.is_functional = false)
    .state(Broken)
        .on(Repair)
            .go_to(Off)
            .then(|store| {
                store.is_functional = true;
                store.brightness = 0;
            })
    .build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_safe_builder_basic_flow() {
        let mut sm = create_smart_light();
        
        // Initially off with 0 brightness
        assert_eq!(sm.state, LightStates::Off);
        assert_eq!(sm.store.brightness, 0);
        assert_eq!(sm.store.power_cycles, 0);
        
        // Turn on
        sm.trigger(LightEvents::TurnOn);
        assert_eq!(sm.state, LightStates::On);
        assert_eq!(sm.store.brightness, 100);
        assert_eq!(sm.store.power_cycles, 1);
        
        // Dim
        sm.trigger(LightEvents::Dim);
        assert_eq!(sm.state, LightStates::Dimmed);
        assert_eq!(sm.store.brightness, 30);
        
        // Turn off from dimmed
        sm.trigger(LightEvents::TurnOff);
        assert_eq!(sm.state, LightStates::Off);
        assert_eq!(sm.store.brightness, 0);
        assert_eq!(sm.store.power_cycles, 2);
    }

    #[test]
    fn test_conditional_transitions() {
        let mut sm = create_smart_light();
        
        // Break the light first
        sm.trigger(LightEvents::Break);
        assert_eq!(sm.state, LightStates::Broken);
        assert!(!sm.store.is_functional);
        
        // Try to turn on broken light - should fail condition
        sm.trigger(LightEvents::TurnOn);
        assert_eq!(sm.state, LightStates::Broken); // Should stay broken
        assert_eq!(sm.store.brightness, 0);
        
        // Repair the light
        sm.trigger(LightEvents::Repair);
        assert_eq!(sm.state, LightStates::Off);
        assert!(sm.store.is_functional);
        
        // Now turning on should work
        sm.trigger(LightEvents::TurnOn);
        assert_eq!(sm.state, LightStates::On);
        assert_eq!(sm.store.brightness, 100);
    }

    #[test]
    fn test_multiple_state_transitions() {
        let mut sm = create_smart_light();
        
        // Test a complex sequence
        sm.trigger(LightEvents::TurnOn);    // Off -> On
        sm.trigger(LightEvents::Dim);       // On -> Dimmed
        sm.trigger(LightEvents::TurnOn);    // Dimmed -> On
        sm.trigger(LightEvents::Break);     // On -> Broken
        sm.trigger(LightEvents::Repair);    // Broken -> Off
        
        assert_eq!(sm.state, LightStates::Off);
        assert_eq!(sm.store.brightness, 0);
        assert!(sm.store.is_functional);
        // Should have 3 power cycles: initial TurnOn, Dimmed->On, none for Break/Repair
        assert_eq!(sm.store.power_cycles, 2);
    }

    #[test]
    fn test_no_valid_transitions() {
        let mut sm = create_smart_light();
        
        // Try invalid transitions - should stay in same state
        sm.trigger(LightEvents::Dim);       // Can't dim when off
        assert_eq!(sm.state, LightStates::Off);
        
        sm.trigger(LightEvents::Repair);    // Can't repair when not broken
        assert_eq!(sm.state, LightStates::Off);
        
        sm.trigger(LightEvents::TurnOn);
        sm.trigger(LightEvents::TurnOn);    // Already on
        assert_eq!(sm.state, LightStates::On);
        assert_eq!(sm.store.power_cycles, 1); // Should only increment once
    }

    #[test]
    fn test_builder_prevents_unsafe_usage() {
        // This test demonstrates that the type system now prevents
        // the unsafe unwrap() that existed before
        
        // The following would not compile (commented out to show intent):
        // let broken_builder = StateMachineBuilder::new(LightStore::default(), LightStates::Off)
        //     .on(LightEvents::TurnOn); // ERROR: Can't call on() without state()!
        
        // This is the correct, safe way:
        let sm = StateMachineBuilder::new(LightStore::default(), LightStates::Off)
            .state(LightStates::Off)  // Must call state() first
            .on(LightEvents::TurnOn)  // Now safe to call on()
            .go_to(LightStates::On)
            .build();
            
        assert_eq!(sm.state, LightStates::Off);
    }

    #[test] 
    fn test_chaining_multiple_events_same_state() {
        let sm = StateMachineBuilder::new(LightStore::default(), LightStates::On)
            .state(LightStates::On)
                .on(LightEvents::TurnOff).go_to(LightStates::Off)
                .on(LightEvents::Dim).go_to(LightStates::Dimmed)
                .on(LightEvents::Break).go_to(LightStates::Broken)
            .build();
            
        assert_eq!(sm.state, LightStates::On);
    }
}