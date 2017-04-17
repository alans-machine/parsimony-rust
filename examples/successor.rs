extern crate parsimony;

use parsimony::tm::turing::Machine;
use parsimony::tm::tape::TapeBuilder;
use parsimony::tm::transition::{Transitions, TransitionKey, TransitionValue};
use parsimony::tm::movement::Movement;

fn main() {
    let mut machine = Some(Machine::new(
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
                TransitionValue::new(-1, "_", Movement::Right))
    ));

    loop {
        machine = match machine {
            Some(m) => {
                println!("{:?}", m);
                m.step()
            },
            None => break,
        }
    }
}
