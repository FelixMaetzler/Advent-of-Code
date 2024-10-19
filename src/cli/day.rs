use std::path::PathBuf;

#[derive(Debug, Clone, Copy)]
pub struct Day {
    pub day: u8,
    pub year: u16,
}
impl Day {
    pub fn input_path(&self) -> PathBuf {
        PathBuf::from(format!("data/inputs/{}/{}.txt", self.year, self.day))
    }
    pub fn puzzle_path(&self) -> PathBuf {
        PathBuf::from(format!("data/puzzles/{}/{}.txt", self.year, self.day))
    }
    pub fn bin_path(&self) -> PathBuf {
        PathBuf::from(format!("src/bin/{}_{}.rs", self.year, self.day))
    }
    pub fn examples_path(&self) -> PathBuf {
        PathBuf::from(format!("data/examples/{}/{}.txt", self.year, self.day))
    }
}
