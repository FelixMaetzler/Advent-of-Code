use std::{fs, fs::OpenOptions, io::Write};

use crate::cli::{aoc_cli::AOCError, commands::download::create_path, day::Day};

use super::download::download;

pub fn prepare(day: Day) -> Result<(), AOCError> {
    download(day)?;
    let bin_path = day.bin_path();
    if !bin_path.exists() {
        create_path(&bin_path).map_err(|_| AOCError::FailedCreateDir(bin_path.clone()))?;
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(bin_path)
            .map_err(|e| AOCError::FileError(e.to_string()))?;
        file.write_all(
            MODULE_TEMPLATE
                .replace("DAY_NUMBER", &format!("{}, {}", day.day, day.year))
                .as_bytes(),
        )
        .map_err(|e| AOCError::FileError(e.to_string()))?;
    }
    let examples_path = day.examples_path();
    if !examples_path.exists() {
        create_path(&examples_path)
            .map_err(|_| AOCError::FailedCreateDir(examples_path.clone()))?;
        let _ = fs::File::create_new(examples_path);
    }
    Ok(())
}

const MODULE_TEMPLATE: &str = "
all_aoc::solution!(DAY_NUMBER);

pub fn part_one(input: &str) -> Option<u32> {
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, None);
    }
}

";
