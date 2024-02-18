

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

    use State::{Locked, UnLocked};
    use Event::{Coin, Push};
    
    let mut turnstyle = StateMachineBuilder::new((), Locked)
        .state(Locked)
            .on(Coin).go_to(UnLocked)
        .state(UnLocked)
            .on(Push).go_to(Locked)
        .build().unwrap();

    turnstyle.trigger(Coin);
    println!("State: {:?}", turnstyle.state);

    turnstyle.trigger(Push);
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

## Implementation

### State Machine 

### State Machine builder




