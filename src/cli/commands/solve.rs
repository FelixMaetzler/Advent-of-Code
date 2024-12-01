use std::process::{Command, Stdio};

use crate::Day;

pub fn solve(day: Day, release: bool, submit: Option<u8>, time: bool) {
    let mut args = vec!["run".to_string()];
    if release {
        args.push("--release".to_string());
    }

    args.push("--bin".to_string());
    args.push(day.bin_name());
    args.push("--".to_string());
    if time {
        args.push("--time".to_string());
    }
    let output = Command::new("cargo")
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .output()
        .unwrap();

    let output = String::from_utf8_lossy(&output.stdout);
    println!("{output}");
    if let Some(x) = submit {
        for line in output.lines() {
            if line.contains(&format!("part {x} is:")) {
                let answer = line.split_once(": ").unwrap().1;
                let mut args = day.as_args();
                args.push("submit".to_string());
                args.push(x.to_string());
                args.push(answer.to_string());

                Command::new("aoc")
                    .args(args)
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .output()
                    .unwrap();
            }
        }
    }
}
