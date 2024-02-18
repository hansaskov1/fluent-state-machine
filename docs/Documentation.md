
The entire library can be found in my [Github](https://github.com/hansaskov1/state_machine_dsl) including 5 extensive test.

## Internal DSL ideology

I believe that an internal DSL should utilize the language's features to enhance the developer experience over what an external DSL can achieve by being generic and extendable. For this assignment i implemented this by having the ability to choose a type for the store of the state machine, or how it's possible to choose any type for the state, including a string, a tuple, and even an integer. The usage should be consistent, or the code will not compile, preventing unexpected runtime errors. Another example is using of the languages build in syntax for comparison with higher order functions.  

This allows other developers of the internal DSL to use it with more than just integers. It also allows the developers to utilize the build in features of the language. The equals, not equals, more than, less than, is already part of the standard syntax so let's just use it. In some languages the use of higher order functions can be scary, but in Rust it's easy because when the compiler is satisfied i know there isn't a weird memory edge case where i shoot my foot clean off. 
## Turnstile Example

Here is a simple example for the classical turnstile state machine. It has two states, `Locked` and `Un-locked`. As well as two events, `Push`and `Coin`.

Inserting a coin will change the state from locked to Unlocked, where it would then be possible to walk push the turnstile over. This will in return change the state to locked. See the diagram below

![Turnstyle Diagram](Turnstyle-Diagram.png)

  The same example id implemented using the StateMachineBuilder. The first two arguments are for the internal store and the initial state for the state machine. Use the `.state` function to add a new state and use `.on` for creating a new event where `.go_to` describes which state to change to when triggered. Build will check for a valid state_machine and either return the StateMachine or an error. In this example we use `unwrap()` to stop the program if the build is not successful.

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

If you like compile time type safety for your states and events, then you can use an enum for the states and the events. 
  
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
  
The output is the same as above.

You can actually use any type for the event as long as it implements the `PartialEqual` trait. The same is true for the state as long as it implements the `Clone` and `Copy` trait. This allows for a very generic and extentible implementation where the user of the StateMachine can decide their own preffered style.

## Cd-Player Example

This next example will showcase some more complex functionalities. Here we have three states `Stopped`, `Playing` and `Paused`, with five events being. `Stop`, `Play`, `Pause`, `Forward` and `Backward`. The cd-player will start in the stopped state. To start you must first forward the tape using the forward event. When a track has been chosen you can start playing music with the play event. See the full diagram for a full understanding.  

![Cd Player Diagram](Cd-Player-Diagram.png)

For this example we set an initial value of the track to 0 and the initial state to stopped. The play event has an additional check that it will only change state if the lamda function provided is true. This is denoted as `only_if(|track| *track > 0) `. The Forward and Backward event will add or subtract 1 from the track state each time they are called. Additionally we will set the track to 0 when the stopped event is triggered. This is done using the `.then()` function.

  
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

  
## Implementation

The implementation consists of three structs: a Trigger, a StateMachine, and a StateMachineBuilder. The StateMachine holds a list of Triggers, a current state, and a store. The StateMachine struct has only one function, `.trigger("event")`, which runs all the transitions for the current state. So, what is a transition? A transition consists of six attributes: event, to_state, from_state, before_event, condition, and after_event. The before_event and after_event are both functions that mutate the internal store. The condition is a function that uses the store as the argument and must return a boolean. When the condition function returns true, the transition occurs by changing the state.

  
The StateMachineBuilder is essentially just a helper class to create the StateMachine using the builder pattern. It's syntactical sugar for creating the StateMachine. It has two attributes, one being the state_machine and the other being the last_added_state. The last added state is set using the `.state()` function. Where a transition with default values is appended to the state_machine with the `.on()` function. The `.go_to()`, `.update()`, `then()`, `only_if` functions all just update the last last value in the state_machine transitions list with their respective new values. In the end we have a build function which checks that all transitions are unique before returning the state_machine object. 

I use three generic types for my Event, State, Store, which is what makes the code so generic. The Event and State types are confined so that they must implement the PartialEq Trait. Which is another way of saying that they can be compared like `a==b`. 

Below are the structs without any of the functions. 


``` Rust
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
```

## Future improvements
Error handling of the StateMachineBuilder is handled by stopping the program, which only occur if an incorrect configuration is done. A future improvement would be to store the errors as they occur and return a Result of either the Statemachine or a list of erros when the build function is called. Then the developer of the library can get a detailed track of all the errors. 

Another way would be to implement a macro to do the error handling at compile time. I have not had time to invest time into this approach, but my initial findings indicate that it is only possible to do with a much worse API. 