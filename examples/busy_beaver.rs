extern crate parsimony;

use parsimony::tm::turing::Machine;
use parsimony::tm::tape::TapeBuilder;
use parsimony::tm::transition::{Transitions, TransitionKey, TransitionValue};
use parsimony::tm::movement::Movement;

fn main() {
    let mut machine = Some(Machine::new(
        0,
        TapeBuilder::with_blank("_").build(),
        Transitions::new()
            .insert(
                TransitionKey::new(0, "_"),
                TransitionValue::new(1, "I", Movement::Left))
            .insert(
                TransitionKey::new(0, "I"),
                TransitionValue::new(2, "I", Movement::Right))
            .insert(
                TransitionKey::new(1, "_"),
                TransitionValue::new(2, "I", Movement::Left))
            .insert(
                TransitionKey::new(1, "I"),
                TransitionValue::new(1, "I", Movement::Left))
            .insert(
                TransitionKey::new(2, "_"),
                TransitionValue::new(3, "I", Movement::Left))
            .insert(
                TransitionKey::new(2, "I"),
                TransitionValue::new(4, "_", Movement::Right))
            .insert(
                TransitionKey::new(3, "_"),
                TransitionValue::new(0, "I", Movement::Right))
            .insert(
                TransitionKey::new(3, "I"),
                TransitionValue::new(3, "I", Movement::Right))
            .insert(
                TransitionKey::new(4, "_"),
                TransitionValue::new(-1, "I", Movement::Left))
            .insert(
                TransitionKey::new(4, "I"),
                TransitionValue::new(0, "_", Movement::Right))
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
