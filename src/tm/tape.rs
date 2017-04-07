//! Tape is a representation of the Turing machine infinite tape.

pub trait Tape<S: Clone> {
    /// Reads the symbol at the location of the head of the Turing machine
    fn read(&self) -> S;

    // Writes symbol at the location of the head of the Turing machine
    fn write(&self, symbol: S) -> Self;

    // The blank symbol for this tape
    fn blank(&self) -> S;

    // Move the head of the Turing machine to the left on this tape
    fn left(&self) -> Self;

    // Move the head of the Turing machine to the right on this tape
    fn right(&self) -> Self;
}

#[derive(Clone)]
struct ConcreteTape<S: Clone> {
    blank: S,
    left: HalfTape<S>,
    current: S,
    right: HalfTape<S>,
}

#[derive(Clone)]
enum HalfTape<S: Clone> {
    Cell(S, Box<HalfTape<S>>),
    Empty,
}

impl<S: Clone> HalfTape<S> {
    fn new() -> HalfTape<S> {
        HalfTape::Empty
    }

    fn pop(&self) -> (Option<S>, HalfTape<S>) {
        match *self {
            HalfTape::Cell(ref symbol, ref boxed_tail) => {
                (Some(symbol.clone()), *boxed_tail.clone())
            },
            HalfTape::Empty => (None, HalfTape::Empty)
        }
    }

    fn push(self, symbol: S) -> HalfTape<S> {
        HalfTape::Cell(symbol, Box::new(self))
    }
}


impl<S: Clone> Tape<S> for ConcreteTape<S> {
    fn read(&self) -> S {
        self.current.clone()
    }

    fn write(&self, symbol: S) -> ConcreteTape<S> {
        ConcreteTape {
            current: symbol, .. self.clone()
        }
    }

    fn blank(&self) -> S {
        self.blank.clone()
    }

    fn left(&self) -> ConcreteTape<S> {
        let (option, left_tail) = self.left.pop();
        match option {
            Some(symbol) => ConcreteTape {
                left: left_tail,
                current: symbol,
                right: self.right.clone().push(self.current.clone()),
                .. self.clone()
            },
            None => ConcreteTape {
                left: left_tail,
                current: self.blank(),
                right: self.right.clone().push(self.current.clone()),
                .. self.clone()
            }
        }
    }

    fn right(&self) -> ConcreteTape<S> {
        let (option, right_tail) = self.right.pop();
        match option {
            Some(symbol) => ConcreteTape {
                left: self.right.clone().push(self.current.clone()),
                current: symbol,
                right: right_tail,
                .. self.clone()
            },
            None => ConcreteTape {
                left: self.left.clone().push(self.current.clone()),
                current: self.blank(),
                right: right_tail,
                .. self.clone()
            }
        }
    }
}

