struct Transition<Trigger, State, Store> {
    trigger: Trigger,
    from_state: State,
    to_state: State,
    before_trigger: Box<dyn FnMut(&mut Store)>,
    condition: Box<dyn Fn(&Store) -> bool>,
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
            if transition.trigger != trigger || self.state != transition.from_state {
                continue;
            }

            (transition.before_trigger)(&mut self.store);

            if (transition.condition)(&self.store) {
                self.state = transition.to_state;
            }
        }

    }
}

pub struct StateMachineBuilder<Trigger, State, Store> {
    state_machine: StateMachine<Trigger, State, Store>,
    last_added_state: State,
}

impl<Trigger, State, Store> StateMachineBuilder<Trigger, State, Store>
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

    pub fn event(mut self, trigger: Trigger, target_state: State) -> Self {
        self.state_machine.transitions.push(Transition {
            trigger,
            from_state: self.last_added_state,
            to_state: target_state,
            condition: Box::new(|_| true),
            before_trigger: Box::new(|_| {}),
        });
        self
    }

    pub fn condition<C>(mut self, condition: C) -> Self
    where
        C: Fn(&Store) -> bool + 'static,
    {
        let last_transition = self.state_machine.transitions.last_mut().unwrap();
        last_transition.condition = Box::new(condition);
        self
    }

    pub fn before_trigger<F>(mut self, before_trigger: F) -> Self
    where
        F: FnMut(&mut Store) + 'static
    {
        let last_transition = self.state_machine.transitions.last_mut().unwrap();
        last_transition.before_trigger = Box::new(before_trigger);
        self
    }

    pub fn build(self) -> StateMachine<Trigger, State, Store> {
        self.state_machine
    }
}