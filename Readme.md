# State Machine DSL

This is a Rust project that provides a domain-specific language (DSL) for creating state machines.

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

### Prerequisites

You need to have Rust and Cargo installed on your machine. If you don't have them installed, you can get them from [here](https://www.rust-lang.org/tools/install).

### Cloning the Repository

To clone the repository, run the following command:

```sh
git clone https://github.com/hansaskov1/state_machine_dsl.git
cd state_machine_dsl
```

### Running the Project
You can run one of the three available examples [cd_player str](examples/cd_player_str.rs),  [turnstile_enum](examples/cd_player_enum.rs), [turnstile_str](examples/turnstile_str.rs), [turnstile_str](examples/turnstile_enum.rs). Here is how to run the cd_player example

```sh
cargo run --example cd_player_str 
```

### Running the Tests

You can run the tests with:

```sh
cargo test
```

## Code Examples

### Creating a Turnstile State Machine with String Literals

```rs
use fluent_state_machine::StateMachineBuilder;

fn main() {
    let mut turnstyle = StateMachineBuilder::new((), "Locked")
        .state("Locked")
            .on("Coin").go_to("Unlocked")
        .state("Unlocked")
            .on("Push").go_to("Locked")
        .build();

    turnstyle.trigger("Coin");
    println!("State: {}", turnstyle.state);

    turnstyle.trigger("Push");
    println!("State: {}", turnstyle.state);
}
```

This code will print out 
```bash
State: Unlocked
State: Locked
```
### Creating a Turnstile State Machine with Enums

```rs
#[derive(Debug, Clone, Copy, PartialEq)]
enum States {
    Locked,
    UnLocked
}

#[derive(PartialEq)]
enum Event {
    Coin,
    Push,
}

fn main() {

    use Event::{Coin, Push};
    use States::{Locked,  UnLocked};
    
    let mut turnstyle = StateMachineBuilder::new((), Locked)
        .state(Locked)
            .on(Coin).go_to(UnLocked)
        .state(UnLocked)
            .on(Push).go_to(Locked)
        .build();

    turnstyle.trigger(Coin);
    println!("State: {:?}", turnstyle.state);

    turnstyle.trigger(Push);
    println!("State: {:?}", turnstyle.state);
}
```
This code will also print out 
```bash
State: Unlocked
State: Locked
```


And here is a more complex example for a Cd-Player

```Rust 
fn main() {
    // Using the Event and State namespace
    use Event::{Backward, Forward, Pause, Play, Stop};
    use State::{Playing, Stopped, Paused};

    // Create store for state machine. In this case it is an integer
    let track = 0;
    
    let mut cd_player =  StateMachineBuilder::new( track, Stopped)
        .state(Stopped)
            .on(Play).go_to(Playing).only_if(|track| *track > 0 )
            .on(Forward).update(|track| *track += 1 )
            .on(Backward).update(|track| *track -= 1)
        .state(Playing)
            .on(Stop).go_to(Stopped).then(|track| *track = 0)
            .on(Pause).go_to(Paused)
        .state(Paused)
            .on(Play).go_to(Playing)
            .on(Stop).go_to(Stopped).then(|track| *track = 0)
            .on(Forward).update(|track| *track += 1)
            .on(Backward).update(|track| *track -= 1)
        .build();

    println!("Track: {}, State: {:?}", cd_player.store, cd_player.state);

    cd_player.trigger(Forward);
    println!("Track: {}, State: {:?}", cd_player.store, cd_player.state);

    cd_player.trigger(Play);
    println!("Track: {}, State: {:?}", cd_player.store, cd_player.state);
}
```

Running this example gives us the following output: 
"
Track: 0, State: Stopped
Track: 1, State: Stopped
Track: 1, State: Playing
"

