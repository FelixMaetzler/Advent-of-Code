use std::{
    fmt::Display,
    path::PathBuf,
    process::{Command, Stdio},
    str::FromStr,
};

use super::runner::PartDayResult;

#[derive(Debug, Clone, Copy)]
pub struct Day {
    pub day: u8,
    pub year: u16,
}
impl Day {
    pub fn input_path(self) -> PathBuf {
        PathBuf::from(format!("data/inputs/{}/{:02}.txt", self.year, self.day))
    }
    pub fn puzzle_path(self) -> PathBuf {
        PathBuf::from(format!("data/puzzles/{}/{:02}.md", self.year, self.day))
    }
    pub fn bin_path(self) -> PathBuf {
        PathBuf::from(format!("src/bin/{}_{:02}.rs", self.year, self.day))
    }
    pub fn examples_path(self) -> PathBuf {
        PathBuf::from(format!("data/examples/{}/{:02}.txt", self.year, self.day))
    }
    pub fn bin_name(self) -> String {
        format!("{}_{:02}", self.year, self.day)
    }
    pub fn as_args(self) -> Vec<String> {
        vec![
            "--year".to_owned(),
            self.year.to_string(),
            "--day".to_owned(),
            self.day.to_string(),
        ]
    }
    pub fn exists(self) -> bool {
        self.bin_path().exists()
    }
    pub fn execute(
        self,
        release: bool,
        time: bool,
        submit: Option<u8>,
    ) -> (PartDayResult<String>, PartDayResult<String>) {
        let mut args = vec!["run".to_owned()];
        if release {
            args.push("--release".to_owned());
        }

        args.push("--bin".to_owned());
        args.push(self.bin_name());
        args.push("--quiet".to_owned());
        args.push("--".to_owned());
        if time {
            args.push("--time".to_owned());
        }
        if let Some(x) = submit {
            args.push("--submit".to_owned());
            args.push(x.to_string());
        }
        args.push("--machine-readable".to_owned());

        let output = Command::new("cargo")
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .output()
            .unwrap();

        let output = String::from_utf8_lossy(&output.stdout);
        (
            PartDayResult::deserialize(output.split("%%%%\n").nth(0).unwrap()).unwrap(),
            PartDayResult::deserialize(output.split("%%%%\n").nth(1).unwrap()).unwrap(),
        )
    }
}
impl Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}.12.{}", self.day, self.year)
    }
}
impl FromStr for Day {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split('.');
        let day = it
            .next()
            .expect("There has to be a Day")
            .parse()
            .expect("Is not an Integer");
        if !(1..=25).contains(&day) {
            return Err(format!("Day is out iof range: {day}"));
        }
        let month = it.next().expect("There has to be a month");
        if month != "12" {
            return Err(format!("Month is not 12: {month}"));
        }
        let year = it
            .next()
            .expect("There has to be a Year")
            .parse()
            .expect("Is not an integer");
        if year < 2015 {
            return Err(format!("Year has to be greater or equal to 2015: {year}"));
        }
        Ok(Self { day, year })
    }
}
