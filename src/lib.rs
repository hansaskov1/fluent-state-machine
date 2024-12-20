// Marker types remain the same
pub struct Initial;
pub struct Configuring;
pub struct Ready;

pub struct Transition<Event, State, Store> {
    event: Event,
    from_state: State,
    to_state: State,
    before_event: fn(&mut Store),
    after_event: fn(&mut Store),
    condition: fn(&Store) -> bool,
}

pub struct StateMachine<Event, State, Store> {
    global_function_after_transition: fn(&mut Store, &State, &Event),
    transitions: Vec<Transition<Event, State, Store>>,
    pub state: State,
    pub store: Store,
}

// Builder state traits
pub trait BuilderState {}
impl BuilderState for Initial {}
impl BuilderState for Configuring {}
impl BuilderState for Ready {}

// Main builder struct
pub struct StateMachineBuilder<Event, State, Store, BuilderStateType: BuilderState> {
    state_machine: StateMachine<Event, State, Store>,
    last_added_state: Option<State>,
    _builder_state: std::marker::PhantomData<BuilderStateType>,
}

// Implementation for the initial builder state
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
            last_added_state: None,
            _builder_state: std::marker::PhantomData,
        }
    }

    pub fn set_global_action(
        mut self,
        global_action: fn(&mut Store, &State, &Event)
    ) -> StateMachineBuilder<Event, State, Store, Initial> {
        self.state_machine.global_function_after_transition = global_action;
        self
    }

    pub fn state(self, state: State) -> StateMachineBuilder<Event, State, Store, Configuring> {
        StateMachineBuilder {
            state_machine: self.state_machine,
            last_added_state: Some(state),
            _builder_state: std::marker::PhantomData,
        }
    }
}

// Implementation for the configuring state
impl<Event, State, Store> StateMachineBuilder<Event, State, Store, Configuring>
where
    State: Copy + PartialEq,
    Event: PartialEq,
{
    pub fn state(mut self, state: State) -> Self {
        self.last_added_state = Some(state);
        self
    }

    pub fn on(mut self, event: Event) -> TransitionBuilder<Event, State, Store> {
        let current_state = self.last_added_state.unwrap();
        let transition = Transition {
            event,
            from_state: current_state,
            to_state: current_state,
            condition: |_| true,
            before_event: |_| {},
            after_event: |_| {},
        };
        
        self.state_machine.transitions.push(transition);
        let transition_index = self.state_machine.transitions.len() - 1;
        
        TransitionBuilder {
            builder: self,
            transition_index,
            current_state,
        }
    }

    pub fn build(self) -> StateMachine<Event, State, Store> {
        self.state_machine
    }
}

// Builder for configuring a specific transition
pub struct TransitionBuilder<Event, State, Store> {
    builder: StateMachineBuilder<Event, State, Store, Configuring>,
    transition_index: usize,
    current_state: State,
}

impl<Event, State, Store> TransitionBuilder<Event, State, Store>
where
    State: Copy + PartialEq,
    Event: PartialEq,
{
    pub fn go_to(mut self, target: State) -> Self {
        let transition = &mut self.builder.state_machine.transitions[self.transition_index];
        transition.to_state = target;
        self
    }

    pub fn update(mut self, before_event: fn(&mut Store)) -> Self {
        let transition = &mut self.builder.state_machine.transitions[self.transition_index];
        transition.before_event = before_event;
        self
    }

    pub fn only_if(mut self, condition: fn(&Store) -> bool) -> Self {
        let transition = &mut self.builder.state_machine.transitions[self.transition_index];
        transition.condition = condition;
        self
    }

    pub fn then(mut self, after_event: fn(&mut Store)) -> Self {
        let transition = &mut self.builder.state_machine.transitions[self.transition_index];
        transition.after_event = after_event;
        self
    }

    pub fn on(mut self, event: Event) -> TransitionBuilder<Event, State, Store> {
        let transition = Transition {
            event,
            from_state: self.current_state,
            to_state: self.current_state,
            condition: |_| true,
            before_event: |_| {},
            after_event: |_| {},
        };
        
        self.builder.state_machine.transitions.push(transition);
        let transition_index = self.builder.state_machine.transitions.len() - 1;
        
        TransitionBuilder {
            builder: self.builder,
            transition_index,
            current_state: self.current_state,
        }
    }

    pub fn state(self, state: State) -> StateMachineBuilder<Event, State, Store, Configuring> {
        self.builder.state(state)
    }

    // Add build method to TransitionBuilder
    pub fn build(self) -> StateMachine<Event, State, Store> {
        self.builder.build()
    }
}

// Implementation for the StateMachine itself
impl<Event, State, Store> StateMachine<Event, State, Store>
where
    State: Copy + PartialEq,
    Event: PartialEq,
{
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