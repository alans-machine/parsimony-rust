//! The actual Turing machine
//!

use super::transition::Transitions;
use super::tape::ConcreteTape;

/// The actual Turing machine
pub struct Machine<S, Q> where S: Clone {
    state: Q,
    tape: ConcreteTape<S>,
    transitions: Transitions<S, Q>,
}
