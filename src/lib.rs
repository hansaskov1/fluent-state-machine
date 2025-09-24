pub struct Transition<Event, State, Store> {
    event: Event,
    from_state: State,
    to_state: State,
    before_event: fn(&mut Store),
    after_event: fn(&mut Store),
    condition: fn(&Store) -> bool,
}

impl<Event, State, Store> Transition<Event, State, Store> {
    fn new(event: Event, from_state: State) -> Self 
    where
        State: Copy,
    {
        Self {
            event,
            from_state,
            to_state: from_state, // Default to same state
            before_event: |_| {}, // Default to no-op
            after_event: |_| {},  // Default to no-op
            condition: |_| true,  // Default to always true
        }
    }
}

pub struct StateMachine<Event, State, Store> {
    global_function_after_transition: fn(&mut Store, &State, &Event),
    transitions: Vec<Transition<Event, State, Store>>,
    pub state: State,
    pub store: Store,
}

impl<Event, State, Store> StateMachine<Event, State, Store>
where
    State: Copy + PartialEq,
    Event: PartialEq,
{
    #[allow(clippy::needless_pass_by_value)]
    pub fn trigger(&mut self, event: Event) {
        for transition in &mut self.transitions {
            if transition.event != event || self.state != transition.from_state {
                continue;
            }

            (transition.before_event)(&mut self.store);

            if (transition.condition)(&self.store) {
                (transition.after_event)(&mut self.store);
                self.state = transition.to_state;
                (self.global_function_after_transition)(&mut self.store, &self.state, &event);
                break;
            }
        }
    }
}


pub struct Initial;
pub struct StateSelected;
pub struct OnSelected;

pub trait BuilderState {}
impl BuilderState for Initial {}
impl BuilderState for StateSelected {}
impl BuilderState for OnSelected {}

#[must_use]
pub struct StateMachineBuilder<Event, State, Store, BuilderStateType: BuilderState> {
    state_machine: StateMachine<Event, State, Store>,
    current_state: State,
    _builder_state: std::marker::PhantomData<BuilderStateType>,
}

impl<Event, State, Store> StateMachineBuilder<Event, State, Store, Initial> 
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
                global_function_after_transition: |_,_,_| {},
            },
            current_state: initial_state,
            _builder_state: std::marker::PhantomData,
        }
    }

    pub fn set_global_action(
        mut self,
        global_action: fn(&mut Store, &State, &Event)
    ) -> Self {
        self.state_machine.global_function_after_transition = global_action;
        self
    }

    pub fn state(mut self, state: State) -> StateMachineBuilder<Event, State, Store, StateSelected> {
        self.current_state = state;
        StateMachineBuilder {
            state_machine: self.state_machine,
            current_state: state,
            _builder_state: std::marker::PhantomData,
        }
    }
}

impl<Event, State, Store> StateMachineBuilder<Event, State, Store, StateSelected>
where
    State: Copy + PartialEq,
    Event: PartialEq,
{
    pub fn on(mut self, event: Event) -> StateMachineBuilder<Event, State, Store, OnSelected> {
        let transition = Transition::new(event, self.current_state);
        self.state_machine.transitions.push(transition);
        
        StateMachineBuilder {
            state_machine: self.state_machine,
            current_state: self.current_state,
            _builder_state: std::marker::PhantomData,
        }
    }
}

impl<Event, State, Store> StateMachineBuilder<Event, State, Store, OnSelected>
where
    State: Copy + PartialEq,
    Event: PartialEq,
{
    pub fn state(mut self, state: State) -> StateMachineBuilder<Event, State, Store, StateSelected> {
        self.current_state = state;
        StateMachineBuilder {
            state_machine: self.state_machine,
            current_state: state,
            _builder_state: std::marker::PhantomData,
        }
    }

    pub fn on(mut self, event: Event) -> Self {
        let transition = Transition::new(event, self.current_state);
        self.state_machine.transitions.push(transition);
        self
    }

    pub fn go_to(mut self, target: State) -> Self {
        self.state_machine.transitions.last_mut().unwrap().to_state = target;
        self
    }

    pub fn update(mut self, before_event: fn(&mut Store)) -> Self {
        self.state_machine.transitions.last_mut().unwrap().before_event = before_event;
        self
    }

    pub fn only_if(mut self, condition: fn(&Store) -> bool) -> Self {
        self.state_machine.transitions.last_mut().unwrap().condition = condition;
        self
    }

    pub fn then(mut self, after_event: fn(&mut Store)) -> Self {
        self.state_machine.transitions.last_mut().unwrap().after_event = after_event;
        self
    }

    pub fn build(self) -> StateMachine<Event, State, Store> {
        self.state_machine
    }
}