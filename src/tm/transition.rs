//! Transitions describes how a Turing machine behaves.
//!
//! Depending on the current state and the symbol that the read/write head is
//! reading from the tape, a transition tells what state comes next, what symbol
//! to write and which way to move the tape.

use std::collections::HashMap;
use super::movement::Movement;

/// `TransitionKey` describe the current context of the Turing machine. I.e. the
/// state the Turing machine is in and the symbol the read/write head is
/// scanning.
#[derive(Hash,PartialEq,Eq)]
pub struct TransitionKey<Q, S> {
    /// The state the Turing machine is in.
    pub state: Q,
    /// The symbol the read/write head is scanning.
    pub symbol: S,
}

impl <Q, S> TransitionKey<Q, S> {
    /// Create a `TransitionKey`
    pub fn new(state: Q, symbol: S) -> TransitionKey<Q, S> {
        TransitionKey { state: state, symbol: symbol }
    }
}

/// `TransitionValue` describes the next context of the Turing machine, I.e. the
/// state is should transition in, the symbol that is should write and the
/// movement of the tape.
pub struct TransitionValue<Q, S> {
    /// The state the Turing machine will be in after the transition.
    pub state: Q,
    /// The symbol to write in the cell the read/write head is scanning.
    pub symbol: S,
    /// The direction the tape is moving in after the transition.
    pub movement: Movement
}

impl <Q, S> TransitionValue<Q, S> {
    /// Create a `TransitionValue`
    pub fn new(state: Q, symbol: S, movement: Movement) -> TransitionValue<Q, S> {
        TransitionValue { state: state, symbol: symbol, movement: movement }
    }
}

/// Transitions are used to describe the entire operation of a Turing machine.
///
/// A Turing machine is defined by the transitions it can make.
pub type Transitions<Q, S> = HashMap<TransitionKey<Q, S>, TransitionValue<Q, S>>;

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::movement::Movement;

    #[test]
    fn should_create_transitions() {
        let mut transitions: Transitions<u32, &str> = Transitions::new();
        transitions.insert(
            TransitionKey::new(0, "I"),
            TransitionValue::new(0, "I", Movement::Right));
        transitions.insert(
            TransitionKey::new(0, "_"),
            TransitionValue::new(1, "I", Movement::Left));
        transitions.insert(
            TransitionKey::new(1, "I"),
            TransitionValue::new(1, "I", Movement::Left));
        transitions.insert(
            TransitionKey::new(1, "_"),
            TransitionValue::new(2, "_", Movement::Right));
    }
}
