

# Rust State Machine

This project is part of an assignment in Model Driven Development which is a coarce for my Software Engineering Master degree. 


## Turnstyle Example

Here is a simple example for the classical turnstyle state machine. It has two states, `Locked` and `Un-locked`. As well as two events, `Push`and `Coin`.

Inserting a coin will change the state from locked to Unlocked, where it would then be possible to walk push the turnstyle over. This will in return change the state to locked. See the diagram below 

![Turnstyle Diagram](Turnstyle-Diagram.png)

The same example can be implemented using the StateMachineBuilder. The first two arguments are for the internal store and the initial state for the state machine. Use the `.state` function to add a new state and use `.on` for creating a new event where `.go_to` describes which state to change to when triggered. Build will check for a valid state_machine and either return the StateMachine or an error. In this example we use `unwrap()` to stop the program if the build is not suceccfull.   
``` Rust
fn main() {
    let mut turnstyle = StateMachineBuilder::new((), "Locked")
        .state("Locked")
            .on("Coin").go_to("Unlocked")
        .state("Unlocked")
            .on("Push").go_to("Locked")
        .build().unwrap();

    turnstyle.trigger("Coin");
    println!("State: {}", turnstyle.state);

    turnstyle.trigger("Push");
    println!("State: {}", turnstyle.state);
}
```
This program will then print out the following
```yaml
State: UnLocked
State: Locked
```


If you would like compile time type safety for your states and events, then you can use an enum instead. Below is a more turse example of using an enum for the state and event. The output is the same as above.

``` Rust
#[derive(Debug, Clone, Copy, PartialEq)]
enum State {
    Locked,
    UnLocked
}

#[derive(PartialEq)]
enum Event {
    Coin,
    Push,
}

fn main() {
    
    let mut turnstyle = StateMachineBuilder::new((), State::Locked)
        .state(State::Locked)
            .on(Event::Coin).go_to(State::UnLocked)
        .state(State::UnLocked)
            .on(Event::Push).go_to(State::Locked)
        .build().unwrap();

    turnstyle.trigger(Event::Coin);
    println!("State: {:?}", turnstyle.state);

    turnstyle.trigger(Event::Push);
    println!("State: {:?}", turnstyle.state);
}
```

You can actually use any type for the event as long as it implements the `PartialEqual` trait. The same is true for the state as long as it implements the `Clone` and `Copy` trait. This allows for a very generic and extentible implementation where the user of the StateMachine can decide their own preffered style. 

## Cd-Player Example

This next example will showcase some more complex functionalities. Here we have three states `Stopped`, `Playing` and `Paused`, with five events being. `Stop`, `Play`, `Pause`, `Forward` and `Backward`. The cd-player will start in the stopped state. To start you must first forward the tape using the forward event. When a track has been chosen you can start playing music with the play event. See the full diagram for a full understanding. 

![Cd Player Diagram](Cd-Player-Diagram.png)

For this example we set an initial value of the track to 0 and the initial state to stopped. The play event has an additional check that it will only change state if the lamda function provided is true. The function we provice will only be true if the track is above 0. The Forward and Backward event will add or subtract 1 from the track state each time they are called. Additionally we will set the track to 0 when the stopped event is triggered. This is done using the `.then()` function. 

```Rust 
fn main() {

    // Create store for state machine. In this case it is an integer
    let track = 0;
    
    let mut cd_player = StateMachineBuilder::new( track, "Stopped")
        .state("Stopped")
            .on("Play").go_to("Playing").only_if(|track| *track > 0 )
            .on("Forward").update(|track| *track += 1 )
            .on("Backward").update(|track| *track -= 1)
        .state("Playing")
            .on("Stop").go_to("Stopped").then(|track| *track = 0)
            .on("Pause").go_to("Paused")
        .state("Paused")
            .on("Play").go_to("Playing")
            .on("Stop").go_to("Stopped").then(|track| *track = 0)
            .on("Forward").update(|track| *track += 1)
            .on("Backward").update(|track| *track -= 1)
        .build()
        .unwrap();

    println!("Track: {}, State: {}", cd_player.store, cd_player.state);

    cd_player.trigger("Forward");
    println!("Track: {}, State: {}", cd_player.store, cd_player.state);

    cd_player.trigger("Play");
    println!("Track: {}, State: {}", cd_player.store, cd_player.state);
}
```

Running this example gives us the following output: 

``` text
Track: 0, State: Stopped
Track: 1, State: Stopped
Track: 1, State: Playing
```

## Internal DSL idealogy
I believe that an internal DSL should utilize the language's features to enhance the developer experience over what an external DSL can achieve by being generic and extendable. An example is the ability to choose a type for the store of the state machine, or how it's possible to choose any type for their state, including a string, a tuple, and even an integer. The usage should just be consistent, or the code will not compile, preventing unexpected runtime errors.

## Implementation
The implementation consists of three structs: a Trigger, a StateMachine, and a StateMachineBuilder. The StateMachine holds a list of Triggers, a current state, and a store. The StateMachine struct has only one function, `.trigger("event")`, which runs all the transitions for the current state. So, what is a transition? A transition consists of six attributes: event, to_state, from_state, before_event, condition, and after_event. The before_event and after_event are both functions that mutate the internal store. The condition is a function that uses the store as the argument and must return a boolean. When the condition function returns true, the transition occurs by changing the state.

The StateMachineBuilder is essentially just a helper class to create the StateMachine using the builder pattern. It's syntactical sugar for creating the StateMachine. It has two attributes, one being the state_machine and the other being the last_added_state. The last added state is set using the `.state()` function. Where a transition with default values is appeneded to the state_machine with the `.on()` function. The `.go_to()`, `.update()`, `then()`, `only_if` functions all just update the last last value in the state_machine transitions list with their respective new values. In the end we have a builder function which checks that all tranitions are unique before returning the state_machine object. The entire library is expressed below in 120 lines of code.  


```rust
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
    pub fn then(mut self, after_event: fn(&mut Store)) -> Self {
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
```