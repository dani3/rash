extern crate termion;

use std::{io::Write, path::PathBuf};
use termion::input::TermRead;

fn prompt(stdout: &mut std::io::StdoutLock) {
    stdout.write_all(b"> ").unwrap();
    stdout.flush().unwrap();
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Command<'a> {
    cmd: String,
    args: Vec<&'a str>,
}

impl<'a> Command<'a> {
    fn new(cmd: String, args: Vec<&'a str>) -> Self {
        Self { cmd, args }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Job<'a> {
    cmds: Vec<Command<'a>>,
    input: Option<PathBuf>,
    output: Option<PathBuf>,
}

impl<'a> Job<'a> {
    fn new(input: &'a str) -> Self {
        let mut commands: Vec<Command<'a>> = Vec::new();
        for command in input.trim().split('|') {
            let tokens: Vec<&str> = command.trim().split_whitespace().collect();
            commands.push(Command::new(tokens[0].to_string(), tokens[1..].to_vec()));
        }

        Self {
            cmds: commands,
            input: None,
            output: None,
        }
    }

    fn execute(&self) {
        // let output = std::process::Command::new(line).output();
        // match output {
        //     Ok(output) => {
        //         stdout.write_all(&output.stdout).expect("Error writing");
        //         stdout.flush().unwrap();
        //     }
        //     Err(msg) => {
        //         panic!("{}", msg);
        //     }
        // }
    }
}

fn main() {
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();

    loop {
        prompt(&mut stdout);
        let line = stdin.read_line();
        if let Ok(Some(line)) = line {
            if !line.is_empty() {
                let job = Job::new(&line);
                dbg!("{}", job);
            }
        }
    }
}
