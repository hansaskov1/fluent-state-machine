# State Machine DSL

This is a Rust project that provides a domain-specific language (DSL) for creating state machines.

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

### Prerequisites

You need to have Rust and Cargo installed on your machine. If you don't have them installed, you can get them from [here](https://www.rust-lang.org/tools/install).

### Cloning the Repository

To clone the repository, run the following command:

```sh
git clone https://github.com/hansaskov/state_machine_dsl.git
cd state_machine_dsl
```

### Running the Project

Navigate to the project directory and run the project with:

```sh
cargo run
```

### Running the Tests

You can run the tests with:

```sh
cargo test
```

## Code Examples

### Creating a Turnstile State Machine with String Literals

```rs
let turnstile = StateMachineBuilder::new((), "Locked")
    .state("Locked")
        .event("Coin", "UnLocked")
        .event("Push", "Locked")
    .state("UnLocked")
        .event("Coin", "UnLocked")
        .event("Push", "Locked")
    .build();
```

### Creating a Turnstile State Machine with Enums

```rs
#[derive(Debug, Clone, Copy, PartialEq)]
enum States {
    Locked,
    UnLocked
}

#[derive(PartialEq)]
enum Triggers {
    Coin,
    Push,
}

let turnstile = StateMachineBuilder::new((), States::Locked)
    .state(States::Locked)
        .event(Triggers::Coin, States::UnLocked)
        .event(Triggers::Push, States::Locked)
    .state(States::UnLocked)
        .event(Triggers::Coin, States::UnLocked)
        .event(Triggers::Push, States::Locked)
    .build();
```

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct, and the process for submitting pull requests to us.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details
