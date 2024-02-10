// Explanation--Trigger: T, From: E, To: E, F: FnMut(&mut S) -> bool
struct Transition<T, E, S> {
    trigger: T,
    from: E,
    to: E,
    before_trigger: Box<dyn FnMut(&mut S)>,
    condition: Box<dyn Fn(&S) -> bool>,
    // after_trigger: Box<dyn FnMut(&mut S)>,
}

pub struct StateMachine<T, E, S> {
    transitions: Vec<Transition<T, E, S>>,
    pub state: E,
    pub store: S,
}

impl<T, E, S> StateMachine<T, E, S>
where
    E: Copy + PartialEq,
{
    pub fn trigger(mut self, in_trigger: T) -> Self
    where
        T: PartialEq,
    {
        for transition in self.transitions.iter_mut() {
            if transition.trigger != in_trigger || self.state != transition.from {
                continue;
            }

            // Do upate before trigger
            (transition.before_trigger)(&mut self.store);

            // Check if the condition is met
            let transition_condition_met = (transition.condition)(&self.store);
            if transition_condition_met {
                self.state = transition.to;
            }
        }

        self
    }
}

pub struct StateMachineBuilder<T, E, S> {
    transitions: Vec<Transition<T, E, S>>,
    pub state: E,
    pub store: S,
    pub recent_state: E,
}

impl<T, E, S> StateMachineBuilder<T, E, S>
where
    E: Copy + PartialEq,
{
    pub fn new(store: S, state: E) -> Self {
        Self {
            transitions: Vec::new(),
            state,
            store,
            recent_state: state,
        }
    }

    pub fn state(mut self, state: E) -> Self {
        Self {
            recent_state: state,
            ..self
        }
    }

    pub fn trigger(mut self, trigger: T, state: E) -> Self {
        self.transitions.push(Transition {
            trigger,
            from: self.recent_state,
            to: state,
            condition: Box::new(|_| true),
            before_trigger: Box::new(|_| {}),
        });
        self
    }

    pub fn only_if<C>(mut self, condition: C) -> Self
    where
        C: Fn(&S) -> bool + 'static,
    {
        let last_transition = self.transitions.last_mut().unwrap();
        last_transition.condition = Box::new(condition);
        self
    }

    pub fn update<F>(mut self, before: F) -> Self
    where
        F: FnMut(&mut S) + 'static,
    {
        let last_transition = self.transitions.last_mut().unwrap();
        last_transition.before_trigger = Box::new(before);
        self
    }

    pub fn build(self) -> StateMachine<T, E, S> {
        StateMachine {
            transitions: self.transitions,
            state: self.state,
            store: self.store,
        }
    }
}
