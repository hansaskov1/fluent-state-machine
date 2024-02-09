mod lib;

// From: E, To: E, F: FnMut(&mut S) -> bool
type Transition<E, S> = (E, E, Box<dyn FnMut(&mut S) -> bool>);

struct StateMachine<E, S> {
    pub transitions: Vec<Transition<E, S>>,
    pub state: E,
    pub store: S,
}

impl<E, S> StateMachine<E, S>
where
    S: Copy,
    E: Default,
{
    fn new(store: S) -> Self {
        Self {
            transitions: Vec::new(),
            state: E::default(),
            store,
        }
    }

    fn transition<F>(mut self, from: E, to: E, f: F) -> Self
    where
        F: FnMut(&mut S) -> bool + 'static,
    {
        self.transitions.push((from, to, Box::new(f)));
        self
    }

    fn run(mut self) -> Self {
        self.transitions.iter_mut().for_each(|(from, to, f)| {
            f(&mut self.store);
        });

        self
    }
}

#[derive(Default, Debug)]
enum State {
    #[default]
    Idle,
    Running,
    Paused,
}

fn main() {
    let storage = 0;

    let state_machine = StateMachine::<State, i32>::new(storage)
        .transition(State::Idle, State::Paused, |store| store == &1)
        .run();

    println!("{:?}", state_machine.store);
}
