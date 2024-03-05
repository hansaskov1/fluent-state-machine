
#[derive(Debug, Clone)]
pub enum StateMachineError {
    DuplicateTransition,
    MissingState,
    // Add other error types as needed
}

impl std::fmt::Display for StateMachineError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::DuplicateTransition => write!(f, "Duplicate transition found"),
            Self::MissingState => write!(f, "Missing state"),
            // Add other error types as needed
        }
    }
}

impl std::error::Error for StateMachineError {}

