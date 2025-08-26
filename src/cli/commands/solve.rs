use core::time::Duration;
use std::io::{self, Write as _};

use crate::cli::day::Day;

pub fn single_day(day: Day, release: bool, submit: Option<u8>, time: bool) {
    let (e1, e2) = day.execute(release, time, submit);
    println!("{}\n{}", e1.pretty_print(), e2.pretty_print());
}
pub fn year(days: Vec<Day>, release: bool, time: bool) {
    let mut part_1 = vec![];
    let mut part_2 = vec![];
    let year = days[0].year;
    let days = days
        .into_iter()
        .filter(|days| days.bin_path().exists())
        .collect::<Vec<_>>();

    for (i, day) in days.iter().enumerate() {
        let i = i + 1;
        let (p1, p2) = day.execute(release, time, None);
        part_1.push(p1);
        part_2.push(p2);

        let bar = "=".repeat(i) + &" ".repeat(days.len() - i);
        print!("\r{bar} {}/{}", i, days.len());
        io::stdout().flush().unwrap();
    }
    println!();
    part_1.retain_mut(|p| p.result.is_some());
    part_2.retain_mut(|p| p.result.is_some());
    println!(
        "In the Year {year} you solved {} Part Ones and {} Part Twos.",
        part_1.len(),
        part_2.len()
    );
    let min = part_1
        .iter()
        .chain(part_2.iter())
        .map(|p| p.durations.iter().min().unwrap())
        .min()
        .unwrap();
    let max = part_1
        .iter()
        .chain(part_2.iter())
        .map(|p| p.durations.iter().max().unwrap())
        .max()
        .unwrap();
    let over_1_ms = part_1
        .iter()
        .chain(part_2.iter())
        .filter(|p| p.average_duration() >= Duration::from_millis(1))
        .count();
    let s = format!(
        "The minimum Duration of a Part is {min:.2?} and the max is {max:.2?}. {over_1_ms}/{} are over 1 ms.",
        part_1.len() + part_2.len()
    );
    println!("{s}");
}
