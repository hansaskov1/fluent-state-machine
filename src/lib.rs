// From: E, To: E, F: FnMut(&mut S) -> bool
type Transition<E, S> = (E, E, Box<dyn FnMut(&mut S) -> bool>);

pub struct StateMachine<E, S> {
    pub transitions: Vec<Transition<E, S>>,
    pub state: E,
    pub store: S,
}

impl<E, S> StateMachine<E, S>
where
    E: Copy,
{
    pub fn new(store: S, state: E) -> Self {
        Self {
            transitions: Vec::new(),
            state,
            store,
        }
    }

    pub fn transition<F>(mut self, from: E, to: E, f: F) -> Self
    where
        F: FnMut(&mut S) -> bool + 'static,
    {
        self.transitions.push((from, to, Box::new(f)));
        self
    }

    pub fn execute_state(mut self, state: E) -> Self
    where
        E: PartialEq,
    {
        for (from, to, boolean_function) in self.transitions.iter_mut() {
            if from != &state {
                continue;
            }

            if boolean_function(&mut self.store) {
                self.state = *to;
            }
        }

        self
    }
}



