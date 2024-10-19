use std::{
    fmt::Display,
    path::PathBuf,
    process::{Command, Output, Stdio},
};
pub enum AOCCommands {
    Download,
    Submit,
}
impl From<AOCCommands> for String {
    fn from(value: AOCCommands) -> Self {
        match value {
            AOCCommands::Download => "download",
            AOCCommands::Submit => "submit",
        }
        .to_string()
    }
}
use super::day::Day;
#[derive(Debug)]
pub enum AOCError {
    CommandNotCallable,
    BadExitStatus(Output),
    FailedCreateDir(PathBuf),
    FileError(String),
    FileNotExist(PathBuf),
}
impl Display for AOCError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
pub fn call_aoc_cli(args: &[String]) -> Result<Output, AOCError> {
    let output = Command::new("aoc")
        .args(args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .map_err(|_| AOCError::CommandNotCallable)?;

    if output.status.success() {
        Ok(output)
    } else {
        Err(AOCError::BadExitStatus(output))
    }
}
pub fn build_args(command: AOCCommands, args: &[&str], day: Day) -> Vec<String> {
    let mut cmd_args = args.iter().map(|s| s.to_string()).collect::<Vec<_>>();

    cmd_args.push("--year".into());
    cmd_args.push(day.year.to_string());

    cmd_args.append(&mut vec![
        "--day".into(),
        day.day.to_string(),
        command.into(),
    ]);

    cmd_args
}
