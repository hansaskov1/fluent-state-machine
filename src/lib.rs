// From: E, To: E, F: FnMut(&mut S) -> bool
type Transition<T, E, S> = (T, E, E, Box<dyn FnMut(&mut S) -> bool>);

pub struct StateMachine<T, E, S> {
    pub transitions: Vec<Transition<T, E, S>>,
    pub state: E,
    pub store: S,
}

impl<T, E, S> StateMachine<T, E, S>
where
    E: Copy + PartialEq,
{
    pub fn new(store: S, state: E) -> Self {
        Self {
            transitions: Vec::new(),
            state,
            store,
        }
    }

    pub fn transition<F>(mut self, trigger: T, from: E, to: E, f: F) -> Self
    where
        F: FnMut(&mut S) -> bool + 'static,
    {
        self.transitions.push((trigger, from, to, Box::new(f)));
        self
    }

    pub fn trigger(mut self, in_trigger: T) -> Self
    where
        T: PartialEq,
    {
        for (trigger, from, to, boolean_function) in self.transitions.iter_mut() {
            if trigger != &in_trigger || self.state != *from {
                continue;
            }

            if boolean_function(&mut self.store) {
                self.state = *to;
            }
        }

        self
    }
}



