//! The actual Turing machine
//!

use std::hash::Hash;
use super::super::transition::{Lookup, Transitions, TransitionKey};
use super::tape::{Tape, ConcreteTape};
use super::super::movement::Movement;

/// The actual Turing machine
#[derive(Debug)]
pub struct Machine<Q, S> where Q: Clone + Eq + Hash, S: Clone + Eq + Hash  {
    state: Q,
    tape: ConcreteTape<S>,
    transitions: Transitions<Q, S>,
}

impl <Q, S> Machine<Q, S> where S: Clone + Eq + Hash, Q: Clone + Eq + Hash {
    /// Create a Turing machine from a initial state, a tape and transistions
    pub fn new(q: Q, tape: ConcreteTape<S>, transitions: Transitions<Q, S>) -> Machine<Q, S> {
        Machine {
            state: q,
            tape: tape,
            transitions: transitions,
        }
    }

    /// Step the machine through one transition.
    pub fn step(&self) -> Option<Machine<Q, S>> {
        let symbol = self.tape.read();
        let key = TransitionKey::new(self.state.clone(), symbol);
        match self.transitions.lookup(&key) {
            Some(next) => {
                Some(Machine {
                    state : next.state.clone(),
                    tape : match next.movement {
                        Movement::Left => self.tape.write(next.symbol.clone()).left(),

                        Movement::Right => self.tape.write(next.symbol.clone()).right(),
                    },
                    transitions: self.transitions.clone(),
                })
            }
            None => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::super::movement::Movement;
    use super::super::tape::TapeBuilder;
    use super::super::super::transition::{Transitions, TransitionKey, TransitionValue};

    #[test]
    fn should_step_through_a_program() {
        let m = Machine::new(
            0,
            TapeBuilder::with_blank("_")
                .with_current("I")
                .with_right_tape(vec!["I", "I"])
                .build(),
            Transitions::new()
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
                    TransitionValue::new(2, "_", Movement::Right))
        );

        match m.step() {
            Some(_) => assert!(true),
            None => assert!(false),
        }
    }
}
