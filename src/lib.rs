mod errors;
use errors::StateMachineError;

pub struct Transition<Event, State, Store> {
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

pub struct StateMachineBuilder<Event, State, Store> {
    state_machine: StateMachine<Event, State, Store>,
    last_added_state: State,
}

impl<Event, State, Store> StateMachine<Event, State, Store>
where
    State: Copy + PartialEq,
    Event: PartialEq,
{
    // Trigger an event, this will result in the state machine changing state if the condition is met. By default the condition is always true.
    pub fn trigger(&mut self, event: Event) {
        for transition in &mut self.transitions {
            // Filter out transitions that do not match the trigger or the current state
            if transition.event != event || self.state != transition.from_state {
                continue;
            }

            // Call the before_event function
            (transition.before_event)(&mut self.store);

            // If condition is met call the after trigger and change internal state
            if (transition.condition)(&self.store) {
                (transition.after_event)(&mut self.store);
                self.state = transition.to_state;
            }
        }
    }
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
    pub fn on(mut self, event: Event) -> Self {
        self.state_machine.transitions.push(Transition {
            event,
            from_state: self.last_added_state,
            to_state: self.last_added_state,
            condition: |_| true,
            before_event: |_| {},
            after_event: |_| {},
        });
        self
    }

    pub fn go_to(mut self, target: State) -> Self {
        let last_transition = self.state_machine.transitions.last_mut().unwrap();
        last_transition.to_state = target;
        self
    }

    #[must_use]
    pub fn update(mut self, before_event: fn(&mut Store)) -> Self {
        let last_transition = self.state_machine.transitions.last_mut().unwrap();
        last_transition.before_event = before_event;
        self
    }

    #[must_use]
    pub fn only_if(mut self, condition: fn(&Store) -> bool) -> Self {
        let last_transition = self.state_machine.transitions.last_mut().unwrap();
        last_transition.condition = condition;
        self
    }

    #[must_use]
    pub fn update_after(mut self, after_event: fn(&mut Store)) -> Self {
        let last_transition = self.state_machine.transitions.last_mut().unwrap();
        last_transition.after_event = after_event;
        self
    }

    // Build the state machine and return the result. If there are any duplicate transitions an error will be returned.
    pub fn build(self) -> Result<StateMachine<Event, State, Store>, StateMachineError> {
        let transitions = &self.state_machine.transitions;

        for i in 0..transitions.len() {
            for j in i + 1..transitions.len() {
                if transitions[i].event == transitions[j].event
                    && transitions[i].from_state == transitions[j].from_state
                    && transitions[i].to_state == transitions[j].to_state
                {
                    return Err(StateMachineError::DuplicateTransition);
                }
            }
        }
        Ok(self.state_machine)
    }
}
