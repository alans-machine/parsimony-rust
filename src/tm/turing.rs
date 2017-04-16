//! The actual Turing machine
//!

use std::hash::Hash;
use super::transition::{Lookup, Transitions, TransitionKey};
use super::tape::{Tape, ConcreteTape};
use super::movement::Movement;

/// The actual Turing machine
pub struct Machine<Q, S> where Q: Clone + Eq + Hash, S: Clone + Eq + Hash  {
    state: Q,
    tape: ConcreteTape<S>,
    transitions: Transitions<Q, S>,
}

impl <Q, S> Machine<Q, S> where S: Clone + Eq + Hash, Q: Clone + Eq + Hash {
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
