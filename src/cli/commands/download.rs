use std::{fs, path::Path, process::Output};

use crate::cli::{
    aoc_cli::{AOCCommands, AOCError, build_args, call_aoc_cli},
    day::Day,
};

pub fn download(day: Day) -> Result<Output, AOCError> {
    let input_path = day.input_path();
    let puzzle_path = day.puzzle_path();
    create_path(&input_path).map_err(|_| AOCError::FailedCreateDir(input_path.clone()))?;
    create_path(&puzzle_path).map_err(|_| AOCError::FailedCreateDir(puzzle_path.clone()))?;
    let args = build_args(
        AOCCommands::Download,
        &[
            "--overwrite",
            "--input-file",
            input_path.to_str().unwrap(),
            "--puzzle-file",
            puzzle_path.to_str().unwrap(),
        ],
        day,
    );
    let output = call_aoc_cli(&args)?;
    println!("---");
    println!(
        "🎄 Successfully wrote input to \"{}\".",
        &input_path.to_str().unwrap()
    );
    println!(
        "🎄 Successfully wrote puzzle to \"{}\".",
        &puzzle_path.to_str().unwrap()
    );
    Ok(output)
}

pub fn create_path(path: &Path) -> Result<(), std::io::Error> {
    let dir = path.parent().unwrap();
    fs::create_dir_all(dir)
}
