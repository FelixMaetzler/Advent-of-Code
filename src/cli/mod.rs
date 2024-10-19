pub mod aoc_cli;
pub mod commands;
pub mod day;
pub mod macros;

use core::panic;
use std::fs;

pub use day::Day;
pub mod runner;

#[must_use]
pub fn read_inputs_file(day: Day) -> String {
    let path = day.input_path();
    match fs::read_to_string(path) {
        Ok(x) => x,
        Err(_) => panic!("Input File can't be read"),
    }
}
#[must_use]
pub fn read_examples_file(day: Day) -> String {
    let path = day.examples_path();
    match fs::read_to_string(path) {
        Ok(x) => x,
        Err(_) => panic!("Example File can't be read"),
    }
}
