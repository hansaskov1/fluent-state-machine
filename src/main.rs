mod lib;

struct Transition<T, S> {
    pub from: S,
    pub to: S,
    pub action: Box<dyn FnMut(&mut T) -> bool>,
}

pub struct Machine<T, S> {
    transitions: Vec<Transition<T, S>>,
    current_state: S,
    store: T,
}

impl<T, S> Machine<T, S> {
    pub fn new(store: T) -> Self
    where
        S: Default,
    {
        Self {
            transitions: Vec::default(),
            current_state: S::default(),
            store,
        }
    }

    pub fn transition<F>(&mut self, from: S, to: S, action: F) -> &mut Self
    where
        F: FnMut(&mut T) -> bool + 'static,
    {
        self.transitions.push(Transition {
            from,
            to,
            action: Box::new(action),
        });
        self
    }

    pub fn run(&mut self) 
    where
        S: PartialEq,
    {
       let filtered_transitions: Vec<_> = self.transitions
            .iter()
            .filter(|transitions| transitions.from == self.current_state)
            .filter(|transitions| (transitions.action)(&mut self.store))
            .collect();
    }
}

#[derive(Default, Debug,)]
enum State {
    #[default]
    Idle,
    Running,
    Paused,
}

fn main() {
    let mut binding = Machine::<_, State>::new(0);
    let machine = binding
        .transition(State::Idle, State::Paused, |i| {
            *i = 1;
            *i == 0
        })
        .transition(State::Paused, State::Running, |i| *i == 1)
        .transition(State::Running, State::Paused, |i| *i == 2)
        .transition(State::Paused, State::Idle, |i| *i == 3);

    println!("{:?}", machine.store);
}
