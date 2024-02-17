use std::collections::HashSet;


struct Transition<Event, State, Store> {
    event: Event,
    from_state: State,
    to_state: State,
    before_event: fn(&mut Store),
    after_event: fn(&mut Store),
    condition: fn(&Store) -> bool,
}

pub struct StateMachine<Event, State, Store> {
    transitions: Vec<Transition<Event, State, Store>>,
    pub state: State,
    pub store: Store,
}

impl<Event, State, Store> StateMachine<Event, State, Store>
where
    State: Copy + PartialEq,
    Event: PartialEq,
{
    pub fn trigger(&mut self, event: Event) {
        for transition in &mut self.transitions {

            // Filter out transitions that do not match the trigger or the current state
            if transition.event != event || self.state != transition.from_state {
                continue;
            }

            // Call the before_trigger function
            (transition.before_event)(&mut self.store);


            // Call the efter trigger and set new state if condition is met
            if (transition.condition)(&self.store) {
                (transition.after_event)(&mut self.store);
                self.state = transition.to_state;
            }
        }
    }
}

pub struct StateMachineBuilder<Event, State, Store> {
    state_machine: StateMachine<Event, State, Store>,
    last_added_state: State,
}

impl<Event, State, Store> StateMachineBuilder<Event, State, Store>
where
    State: Copy + PartialEq,
    Event: PartialEq,
{
    pub fn new(data_store: Store, initial_state: State) -> Self {
        Self {
            state_machine: StateMachine {
                transitions: Vec::new(),
                state: initial_state,
                store: data_store,
            },
            last_added_state: initial_state,
        }
    }

    #[must_use]
    pub const fn state(mut self, state: State) -> Self {
        self.last_added_state = state;
        self
    }

    #[must_use]
    pub fn event(mut self, event: Event, new_state: State) -> Self {

        self.state_machine.transitions.push(Transition {
            event,
            from_state: self.last_added_state,
            to_state: new_state,
            condition: |_| true,
            before_event: |_| {},
            after_event: |_| {},
        });
        self
    }


    #[must_use]
    pub fn before_condition(mut self, before_event: fn(&mut Store)) -> Self
    {
        let last_transition = self.state_machine.transitions.last_mut().unwrap();
        last_transition.before_event = before_event;
        self
    }

    #[must_use]
    pub fn condition(mut self, condition: fn(&Store) -> bool) -> Self
    {
        let last_transition = self.state_machine.transitions.last_mut().unwrap();
        last_transition.condition = condition;
        self
    }


    #[must_use]
    pub fn after_condition(mut self, after_event: fn(&mut Store)) -> Self
    {
        let last_transition = self.state_machine.transitions.last_mut().unwrap();
        last_transition.after_event = after_event;
        self
    }

    pub fn build(self) -> Result<StateMachine<Event, State, Store>, StateMachineError> {
        let transitions = &self.state_machine.transitions;

        for i in 0..transitions.len() {
            for j in i+1..transitions.len() {
                if transitions[i].event == transitions[j].event &&
                   transitions[i].from_state == transitions[j].from_state &&
                   transitions[i].to_state == transitions[j].to_state {
                    return Err(StateMachineError::DuplicateTransition);
                }
            }
        }

        Ok(self.state_machine)
    }
}


#[derive(Debug)]
pub enum StateMachineError {
    DuplicateTransition,
    // Add other error types as needed
}

impl std::fmt::Display for StateMachineError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            StateMachineError::DuplicateTransition => write!(f, "Duplicate transition found"),
            // Add other error types as needed
        }
    }
}

impl std::error::Error for StateMachineError {}
