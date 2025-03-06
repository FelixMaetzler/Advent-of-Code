use std::{
    cmp,
    fmt::Display,
    process::{self, Command, Stdio},
    str::FromStr,
    time::{Duration, Instant},
};

use crate::Day;
pub const ANSI_BOLD: &str = "\u{1b}[1m";
pub const ANSI_ITALIC: &str = "\u{1b}[3m";
pub const ANSI_RESET: &str = "\u{1b}[0m";
pub struct PartDayResult<T> {
    pub day: Day,
    pub part: u8,
    pub result: Option<T>,
    pub durations: Vec<Duration>,
}

impl<T> PartDayResult<T>
where
    T: ToString + FromStr,
{
    fn serialize(&self) -> String {
        let day = self.day;
        let part = self.part;
        let result = self
            .result
            .as_ref()
            .map(|r| r.to_string())
            .unwrap_or_else(|| "None".to_string());
        let durations = self
            .durations
            .iter()
            .map(|d| d.as_nanos().to_string())
            .collect::<Vec<_>>()
            .join(";");

        format!("{};;{};;{};;{}%%%%", day, part, result, durations)
    }

    pub fn deserialize(input: &str) -> Result<Self, String> {
        let parts: Vec<&str> = input.split(";;").collect();
        if parts.len() != 4 {
            return Err(format!(
                "Invalid input format. Got {} but expected 4. Parts: {:?}",
                parts.len(),
                &parts
            ));
        }

        let day = Day::from_str(parts[0]).map_err(|_| "Invalid day value".to_string())?;
        let part = parts[1]
            .parse::<u8>()
            .map_err(|_| "Invalid part value".to_string())?;
        let result = if parts[2] == "None" {
            None
        } else {
            Some(
                parts[2]
                    .parse::<T>()
                    .map_err(|_| "Invalid result value".to_string())?,
            )
        };
        let durations = parts[3]
            .split(';')
            .map(|d| d.parse::<u64>().map(Duration::from_nanos))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| "Invalid duration value".to_string())?;

        Ok(PartDayResult {
            day,
            part,
            result,
            durations,
        })
    }
}
impl<T> PartDayResult<T>
where
    T: Display,
{
    pub fn pretty_print(&self) -> String {
        let avg = self.average_duration();
        if let Some(x) = &self.result {
            match self.durations.len() {
                0 => unreachable!(),
                1 => format!(
                    "Part {}: {ANSI_BOLD}{}{ANSI_RESET} ({:.02?})",
                    self.part, x, avg,
                ),
                2..25 => format!(
                    "Part {}: {ANSI_BOLD}{}{ANSI_RESET} ({:.02?} @ {} samples)",
                    self.part,
                    x,
                    avg,
                    self.durations.len()
                ),
                25.. => format!(
                    "Part {}: {ANSI_BOLD}{}{ANSI_RESET} ({:.02?} @ {} samples  with {:?} std, {:?}..{:?})",
                    self.part,
                    x,
                    avg,
                    self.durations.len(),
                    self.standard_deviation(),
                    self.durations.iter().min().unwrap(),
                    self.durations.iter().max().unwrap(),
                ),
            }
        } else {
            format!("Part {ANSI_BOLD}{}{ANSI_RESET}: ✖", self.part)
        }
    }
}
impl<T> PartDayResult<T> {
    pub fn average_duration(&self) -> Duration {
        self.durations.iter().sum::<Duration>() / (self.durations.len() as u32)
    }
    fn standard_deviation(&self) -> Duration {
        let average = self.average_duration().as_nanos();
        let mut erg = 0;
        for n in &self.durations {
            erg += (n.as_nanos().abs_diff(average)).pow(2);
        }
        let erg = erg as f64 / (self.durations.len() - 1) as f64;
        Duration::from_nanos(erg.sqrt() as u64)
    }
}
pub fn run_part<I: Clone, T: FromStr + Display>(
    func: impl Fn(I) -> Option<T>,
    input: I,
    day: Day,
    part: u8,
) {
    let (result, durations) = run_timed(func, input);
    let res = PartDayResult {
        day,
        part,
        result,
        durations,
    };
    if std::env::args().any(|x| x == "--machine-readable") {
        println!("{}", res.serialize());
    } else {
        println!("{}", res.pretty_print());
    }

    if let Some(result) = res.result {
        submit_result(result, day, part);
    }
}

fn run_timed<I: Clone, T>(func: impl Fn(I) -> T, input: I) -> (T, Vec<Duration>) {
    let timer = Instant::now();
    let result = func(input.clone());
    let base_time = timer.elapsed();

    let run = if std::env::args().any(|x| x == "--time") {
        bench(func, input, &base_time)
    } else {
        vec![base_time]
    };

    (result, run)
}
fn bench<I: Clone, T>(func: impl Fn(I) -> T, input: I, base_time: &Duration) -> Vec<Duration> {
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

    timers
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
        eprintln!(
            "command \"aoc\" not found or not callable. Try running \"cargo install aoc-cli\" to install it."
        );
        process::exit(1);
    }
    //println!("Submitting result via aoc-cli...");
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
