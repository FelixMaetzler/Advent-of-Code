use std::fmt::Display;

use crate::Day;

pub fn run_part<I: Clone, T: Display>(func: impl Fn(I) -> Option<T>, input: I, day: Day, part: u8) {
    let erg = func(input);
    if let Some(x) = erg {
        println!("The anwser for {day} part {part} is: {x}")
    }
}
