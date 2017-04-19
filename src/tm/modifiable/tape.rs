//! Tape is a representation of the Turing machine infinite tape.
//!
//! You can read from it, write to it and move it left and right.

/// Contract a concrete Tape should adhere to
pub trait Tape<S: Clone> {
    /// Reads the symbol at the location of the head of the Turing machine.
    fn read(&self) -> S;

    /// Writes symbol at the location of the head of the Turing machine.
    fn write(&mut self, symbol: S);

    /// The blank symbol for this tape.
    fn blank(&self) -> S;

    /// Move the head of the Turing machine to the left on the tape.
    fn left(&mut self);

    /// Move the head of the Turing machine to the right on the tape.
    fn right(&mut self);
}

/// A type implementing the `Tape` trait
#[derive(Debug)]
pub struc ConcreteTape<S> {
    blank: S,
    left: HalfTape<S>,
    current: S,
    right: HalfTape<S>,
}

#[derive(Debug)]
enum HalfTape<S> {
    Cell(S, Box<HalfTape<S>>),
    Empty,
}

impl<S> HalfTape<S> {
    fn empty() -> HalfTape<S> {
        HalfTape::Empty
    }

}
