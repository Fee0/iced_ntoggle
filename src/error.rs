use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum NtoggleError {
    #[error("ntoggle needs at least {min} states, got {actual}")]
    TooFewStates { min: usize, actual: usize },
    #[error("selected index {index} is out of bounds for {len} states")]
    SelectionOutOfBounds { index: usize, len: usize },
    #[error("selected index {index} appears more than once")]
    DuplicateSelection { index: usize },
    #[error("max_selected must be greater than zero")]
    MaxSelectedZero,
    #[error("selected {actual} states, but max_selected is {max}")]
    TooManySelected { max: usize, actual: usize },
}
