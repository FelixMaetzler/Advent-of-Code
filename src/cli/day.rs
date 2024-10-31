use std::{fmt::Display, path::PathBuf};

#[derive(Debug, Clone, Copy)]
pub struct Day {
    pub day: u8,
    pub year: u16,
}
impl Day {
    pub fn input_path(&self) -> PathBuf {
        PathBuf::from(format!("data/inputs/{}/{:02}.txt", self.year, self.day))
    }
    pub fn puzzle_path(&self) -> PathBuf {
        PathBuf::from(format!("data/puzzles/{}/{:02}.md", self.year, self.day))
    }
    pub fn bin_path(&self) -> PathBuf {
        PathBuf::from(format!("src/bin/{}_{:02}.rs", self.year, self.day))
    }
    pub fn examples_path(&self) -> PathBuf {
        PathBuf::from(format!("data/examples/{}/{:02}.txt", self.year, self.day))
    }
    pub fn bin_name(&self) -> String {
        format!("{}_{}", self.year, self.day)
    }
    pub fn as_args(&self) -> Vec<String> {
        vec![
            "--year".to_string(),
            self.year.to_string(),
            "--day".to_string(),
            self.day.to_string(),
        ]
    }
}
impl Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}.12.{}", self.day, self.year)
    }
}
