use std::{
    cmp,
    fmt::Display,
    io::{stdout, Write},
    process::{self, Command, Stdio},
    time::{Duration, Instant},
};

use crate::Day;
pub const ANSI_BOLD: &str = "\u{1b}[1m";
pub const ANSI_ITALIC: &str = "\u{1b}[3m";
pub const ANSI_RESET: &str = "\u{1b}[0m";
pub fn run_part<I: Clone, T: Display>(func: impl Fn(I) -> Option<T>, input: I, day: Day, part: u8) {
    let part_str = format!("Part {part}");

    let (result, duration, samples) =
        run_timed(func, input, |result| print_result(result, &part_str, ""));

    print_result(&result, &part_str, &format_duration(&duration, samples));

    if let Some(result) = result {
        submit_result(result, day, part);
    }
}
/// Run a solution part. The behavior differs depending on whether we are running a release or debug build:
///  1. in debug, the function is executed once.
///  2. in release, the function is benched (approx. 1 second of execution time or 10 samples, whatever take longer.)
fn run_timed<I: Clone, T>(
    func: impl Fn(I) -> T,
    input: I,
    hook: impl Fn(&T),
) -> (T, Duration, u128) {
    let timer = Instant::now();
    let result = func(input.clone());
    let base_time = timer.elapsed();

    hook(&result);

    let run = if std::env::args().any(|x| x == "--time") {
        bench(func, input, &base_time)
    } else {
        (base_time, 1)
    };

    (result, run.0, run.1)
}
fn bench<I: Clone, T>(func: impl Fn(I) -> T, input: I, base_time: &Duration) -> (Duration, u128) {
    let mut stdout = stdout();

    print!(" > {ANSI_ITALIC}benching{ANSI_RESET}");
    let _ = stdout.flush();

    let bench_iterations =
        (Duration::from_secs(1).as_nanos() / cmp::max(base_time.as_nanos(), 10)).clamp(10, 10000);

    let mut timers: Vec<Duration> = vec![];

    for _ in 0..bench_iterations {
        // need a clone here to make the borrow checker happy.
        let cloned = input.clone();
        let timer = Instant::now();
        func(cloned);
        timers.push(timer.elapsed());
    }

    (
        #[allow(clippy::cast_possible_truncation)]
        Duration::from_nanos(average_duration(&timers) as u64),
        bench_iterations,
    )
}
fn average_duration(numbers: &[Duration]) -> u128 {
    numbers
        .iter()
        .map(std::time::Duration::as_nanos)
        .sum::<u128>()
        / numbers.len() as u128
}
fn standard_deviation(numbers: &[Duration]) -> u128 {
    let average = average_duration(numbers);
    let mut erg = 0;
    for n in numbers {
        erg += (n.as_nanos() - average).pow(2);
    }
    let erg = erg as f64 / (numbers.len() - 1) as f64;
    erg.sqrt() as u128
}
fn print_result<T: Display>(result: &Option<T>, part: &str, duration_str: &str) {
    let is_intermediate_result = duration_str.is_empty();

    match result {
        Some(result) => {
            if result.to_string().contains('\n') {
                let str = format!("{part}: ▼ {duration_str}");
                if is_intermediate_result {
                    print!("{str}");
                } else {
                    print!("\r");
                    println!("{str}");
                    println!("{result}");
                }
            } else {
                let str = format!("{part}: {ANSI_BOLD}{result}{ANSI_RESET}{duration_str}");
                if is_intermediate_result {
                    print!("{str}");
                } else {
                    print!("\r");
                    println!("{str}");
                }
            }
        }
        None => {
            if is_intermediate_result {
                print!("{part}: ✖");
            } else {
                print!("\r");
                println!("{part}: ✖             ");
            }
        }
    }
}
fn format_duration(duration: &Duration, samples: u128) -> String {
    if samples == 1 {
        format!(" ({duration:.1?})")
    } else {
        format!(" ({duration:.1?} @ {samples} samples)")
    }
}

/// Parse the arguments passed to `solve` and try to submit one part of the solution if:
///  1. we are in `--release` mode.
///  2. aoc-cli is installed.
fn submit_result<T: Display>(result: T, day: Day, part: u8) {
    let args: Vec<String> = std::env::args().collect();

    if !args.contains(&"--submit".into()) {
        return;
    }

    if args.len() < 3 {
        eprintln!("Unexpected command-line input. Format: cargo solve 1 --submit 1");
        process::exit(1);
    }

    let part_index = args.iter().position(|x| x == "--submit").unwrap() + 1;

    let Ok(part_submit) = args[part_index].parse::<u8>() else {
        eprintln!("Unexpected command-line input. Format: cargo solve 1 --submit 1");
        process::exit(1);
    };

    if part_submit != part {
        return;
    }

    submit(day, part, result.to_string())
}
fn submit(day: Day, part: u8, answer: String) {
    if !check() {
        eprintln!("command \"aoc\" not found or not callable. Try running \"cargo install aoc-cli\" to install it.");
        process::exit(1);
    }
    println!("Submitting result via aoc-cli...");
    let mut args = day.as_args();
    args.push("submit".to_string());
    args.push(part.to_string());
    args.push(answer.to_string());

    Command::new("aoc")
        .args(args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .unwrap();
}
fn check() -> bool {
    Command::new("aoc").arg("-V").output().is_ok()
}
