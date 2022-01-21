extern crate termion;

mod job;

use crate::job::Job;
use std::io::Write;
use termion::input::TermRead;

fn prompt(stdout: &mut std::io::StdoutLock) {
    stdout.write_all(b"> ").unwrap();
    stdout.flush().unwrap();
}

fn execute() {
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
