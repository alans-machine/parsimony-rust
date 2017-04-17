#![deny(missing_docs)]
//! Formally, a k-state Turing machine is a 7-tuple M = (Q, Gamma, b, Sigma, delta, q0, F),
//! where
//!
//! * Q is the set of k states {q0, q1, ..., qt}
//! * Gamma is the set of tape alphabet
//! * _ is the blank symbol
//! * Sigma is the set of input symbols
//! * delta : Q x Gamma -> (Q union F) x Gamma x {L, R} is the transition function
//! * q0 is the start states
//! * F : {HALT, ERROR} is the set of halting states
//!
//! A Turing machine's states make up the Turing machine's easily-accessible,
//! finite memory. The Turing machine's state is initialized to q0.
//!
//! The tape alphabet symbols correspond to the symbols that can be written on
//! the Turing machine's infinite tape.

pub mod movement;
pub mod unmodifiable;
pub mod transition;
