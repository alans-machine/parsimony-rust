//! Movement for a Turing machine is restricted to Left and Right. This tells
//! which way the Turing machines read/write head should move after a state
//! transition.

pub enum Movement {
    Left,
    Right,
}
