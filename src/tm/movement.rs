//! Movement describes the movement of the read/write head of a Turing machine.

/// The ways the read/write head can move is restricted to Left and Right. The movement is performed after a state
/// change.
pub enum Movement {
    /// Moves the tape left.
    Left,
    /// Moves the tape right.
    Right,
}
