use std::env;

use all_aoc::cli::{
    commands::{download::download, prepare::prepare, solve::solve},
    day::Day,
};
#[derive(Debug)]
enum Command {
    Download { days: Vec<Day> },
    Prepare { days: Vec<Day> },
    Solve { days: Vec<Day>, submit: Option<u8> },
}
impl Command {
    fn execute(&self) -> Result<(), String> {
        match self {
            Command::Download { days } => {
                for day in days {
                    if let Err(e) = download(*day) {
                        return Err(e.to_string());
                    }
                }
                Ok(())
            }
            Command::Prepare { days } => {
                for day in days {
                    if let Err(e) = prepare(*day) {
                        return Err(e.to_string());
                    }
                }
                Ok(())
            }
            Command::Solve { days, submit } => {
                for day in days {
                    solve(*day, false, *submit)
                }
                Ok(())
            }
        }
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = parse(&args);
    match command {
        Ok(c) => match c.execute() {
            Ok(_) => {}
            Err(e) => eprintln!("{e}"),
        },
        Err(e) => eprintln!("{e}"),
    }
}

fn parse(args: &[String]) -> Result<Command, String> {
    let subcommand = args.get(1).ok_or("Missing Command".to_string())?;
    match subcommand.as_str() {
        "download" => {
            let day = args.get(2).ok_or("Missing Day".to_string())?;
            Ok(Command::Download {
                days: parse_day(day)?,
            })
        }
        "prepare" => {
            let day = args.get(2).ok_or("Missing Day".to_string())?;
            Ok(Command::Prepare {
                days: parse_day(day)?,
            })
        }
        "solve" => {
            let day = args.get(2).ok_or("Missing Day".to_string())?;
            let submit = args.get(3).is_some_and(|x| x == "--submit").then(|| {
                args.get(4)
                    .expect("if --submit flag is set, there has to be a next argument")
                    .parse()
                    .expect("Has to be a number")
            });
            Ok(Command::Solve {
                days: parse_day(day)?,
                submit,
            })
        }

        c => Err(format!("Unknown Subcommand {c}")),
    }
}
fn parse_day(arg: &str) -> Result<Vec<Day>, String> {
    // possible Ways:
    // 17           needs year from env
    // 17.2023      specified year
    // 2023         whole year
    if arg.contains('.') {
        let (day, year) = arg.split_once('.').expect("checked above");
        let day = day
            .parse::<u8>()
            .map_err(|_| format!("Day is not a number between 1 and 25: {}", day))?;
        let mut year = year
            .parse::<u16>()
            .map_err(|_| format!("Year is not a number greater than 2015: {}", year))?;
        if !(1..=25).contains(&day) {
            return Err(format!("Day is not a number between 1 and 25: {}", day));
        }
        if year < 2015 {
            if year < 15 {
                return Err(format!("Year is not a number greater than 2015: {}", year));
            } else {
                year += 2000;
            }
        }
        Ok(vec![Day { day, year }])
    } else {
        let n = arg.parse::<u32>().map_err(|e| e.to_string())?;
        if (1..=25).contains(&n) {
            Ok(vec![Day {
                day: n as u8,
                year: match std::env::var("AOC_YEAR") {
                    Ok(x) => x.parse::<u16>().map_err(|e| e.to_string())?,
                    Err(_) => return Err(
                        "The year is not specified. Please have a look in the .cargo/config.toml"
                            .to_string(),
                    ),
                },
            }])
        } else if n >= 2015 {
            Ok((1..=25)
                .map(|day| Day {
                    day,
                    year: n as u16,
                })
                .collect())
        } else {
            Err(format!("'{n}' is not a day"))
        }
    }
}
