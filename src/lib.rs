struct Transition<Trigger, State, Store> {
    trigger: Trigger,
    from_state: State,
    to_state: State,
    before_trigger: fn(&mut Store),
    after_trigger: fn(&mut Store),
    condition: fn(&Store) -> bool,
}

pub struct StateMachine<Trigger, State, Store> {
    transitions: Vec<Transition<Trigger, State, Store>>,
    pub state: State,
    pub store: Store,
}

impl<Trigger, State, Store> StateMachine<Trigger, State, Store>
where
    State: Copy + PartialEq,
    Trigger: PartialEq,
{
    pub fn trigger(&mut self, trigger: Trigger) {
        for transition in &mut self.transitions {

            // Filter out transitions that do not match the trigger or the current state
            if transition.trigger != trigger || self.state != transition.from_state {
                continue;
            }

            // Call the before_trigger function
            (transition.before_trigger)(&mut self.store);


            // Call the efter trigger and set new state if condition is met
            if (transition.condition)(&self.store) {
                (transition.after_trigger)(&mut self.store);
                self.state = transition.to_state;
            }
        }

    }
}

pub struct StateMachineBuilder<Trigger, State, Store> {
    state_machine: StateMachine<Trigger, State, Store>,
    last_added_state: State,
}

impl<Event, State, Store> StateMachineBuilder<Event, State, Store>
where
    State: Copy + PartialEq,
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

    pub const fn state(mut self, state: State) -> Self {
        self.last_added_state = state;
        self
    }

    pub fn event(mut self, trigger: Event, new_state: State) -> Self {
        self.state_machine.transitions.push(Transition {
            trigger,
            from_state: self.last_added_state,
            to_state: new_state,
            condition: |_| true,
            before_trigger: |_| {},
            after_trigger: |_| {},
        });
        self
    }


    pub fn before_condition(mut self, trigger: fn(&mut Store)) -> Self
    {
        let last_transition = self.state_machine.transitions.last_mut().unwrap();
        last_transition.before_trigger = trigger;
        self
    }

    pub fn condition(mut self, condition: fn(&Store) -> bool) -> Self
    {
        let last_transition = self.state_machine.transitions.last_mut().unwrap();
        last_transition.condition = condition;
        self
    }



    pub fn after_condition(mut self, trigger: fn(&mut Store)) -> Self
    {
        let last_transition = self.state_machine.transitions.last_mut().unwrap();
        last_transition.after_trigger = trigger;
        self
    }

    pub fn build(self) -> StateMachine<Event, State, Store> {
        self.state_machine
    }
}