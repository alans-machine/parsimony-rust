//! Transitions describes how a Turing machine behaves.
//!
//! Depending on the current state and the symbol that the read/write head is
//! reading from the tape, a transition tells what state comes next, what symbol
//! to write and which way to move the tape.

use super::movement::Movement;

/// `TransitionKey` describe the current context of the Turing machine. I.e. the
/// state the Turing machine is in and the symbol the read/write head is
/// scanning.
#[derive(Clone, PartialEq, Eq, Debug)]
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
#[derive(Clone, Debug)]
pub struct TransitionValue<Q, S> where Q: Clone, S: Clone {
    /// The state the Turing machine will be in after the transition.
    pub state: Q,
    /// The symbol to write in the cell the read/write head is scanning.
    pub symbol: S,
    /// The direction the tape is moving in after the transition.
    pub movement: Movement,
}

impl <Q, S> TransitionValue<Q, S> where Q: Clone, S: Clone {
    /// Create a `TransitionValue`
    pub fn new(state: Q, symbol: S, movement: Movement) -> TransitionValue<Q, S> {
        TransitionValue { state: state, symbol: symbol, movement: movement }
    }
}

/// Transitions are used to describe the entire operation of a Turing machine.
///
/// A Turing machine is defined by the transitions it can make.
#[derive(Clone, Debug)]
pub enum Transitions<Q, S> where Q: Clone, S: Clone {
    /// Head of the linked list
    Transition(TransitionKey<Q,S>, TransitionValue<Q,S>, Box<Transitions<Q,S>>),
    /// Stop case of the linked list
    NoTransition,
}

impl <Q, S> Transitions<Q, S> where Q: Clone, S: Clone {
    /// Create transitions
    pub fn new() -> Transitions<Q, S> {
        Transitions::NoTransition
    }

    /// Insert a transition and return the new transition
    pub fn insert(self, key: TransitionKey<Q,S>, value: TransitionValue<Q,S>) -> Transitions<Q, S> {
        Transitions::Transition(key, value, Box::new(self))
    }

    /// The number of transitions
    pub fn len(&self) -> usize {
        match *self {
            Transitions::NoTransition => 0,

            Transitions::Transition(_, _, ref next) => 1 + next.len()
        }
    }
}

/// Lookup a `TransitionKey`, returning a `TransitionValue`
///
/// This trait specifies the contract any collection of Transitions should adhere to.
pub trait Lookup<Q, S> where Q: Clone, S: Clone {
    /// Lookup a specific `TransitionKey` in self.
    fn lookup(&self, key: &TransitionKey<Q, S>) -> Option<TransitionValue<Q, S>>;
}


impl <Q, S> Lookup<Q,S> for Transitions<Q, S> where Q: Clone + Eq, S: Clone + Eq {
    /// Lookup a `TransitionKey`
    fn lookup(&self, target: &TransitionKey<Q, S>) -> Option<TransitionValue<Q, S>> {
        match *self {
            Transitions::Transition(ref key, ref value, ref other_transitions) => {
                if *key == *target {
                    Some(value.clone())
                } else {
                    other_transitions.lookup(target)
                }
            }

            Transitions::NoTransition => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::movement::Movement;

    #[test]
    fn should_create_transitions() {
        let transitions: Transitions<u32, &str> = Transitions::new()
            .insert(
                TransitionKey::new(0, "I"),
                TransitionValue::new(0, "I", Movement::Right))
            .insert(
                TransitionKey::new(0, "_"),
                TransitionValue::new(1, "I", Movement::Left))
            .insert(
                TransitionKey::new(1, "I"),
                TransitionValue::new(1, "I", Movement::Left))
            .insert(
                TransitionKey::new(1, "_"),
                TransitionValue::new(2, "_", Movement::Right));

        assert_eq!(transitions.len(), 4);
    }

    #[test]
    fn should_lookup_transition() {
        let transitions: Transitions<u32, &str> = Transitions::new()
            .insert(
                TransitionKey::new(0, "I"),
                TransitionValue::new(0, "I", Movement::Right))
            .insert(
                TransitionKey::new(0, "_"),
                TransitionValue::new(1, "I", Movement::Left))
            .insert(
                TransitionKey::new(1, "I"),
                TransitionValue::new(1, "I", Movement::Left))
            .insert(
                TransitionKey::new(1, "_"),
                TransitionValue::new(2, "_", Movement::Right));

        let value = transitions.lookup(&TransitionKey::new(0, "I"));

        match value {
            Some(_) => assert!(true),

            None => assert!(false, "value not found"),
        }
    }
}
