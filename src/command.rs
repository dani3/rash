//! Command representation.
//!
//! This module provides two types, [`Command`] and [`Batch`]. A command and its arguments are
//! stored in a [`Command`] struct. A list of commands are stored in a [`Batch`]. A [`Batch`] also contains
//! the paths to an input and/or output files if _stdin_ and/or _stdout_ have been redirected.
//!
//! ## Simple usage
//!
//! To parse a simple command simply send a [`str`] when creating a [`Batch`]:
//!
//! ```
//! let s = "wc -l file.txt";
//! let job = Batch::new(s);
//! ```
use core::mem::size_of;
use std::{path::PathBuf, str::FromStr};

/// Representation of a shell command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Command<'a> {
    /// Command.
    cmd: String,
    /// Command arguments.
    args: Vec<&'a str>,
}

impl<'a> Command<'a> {
    pub fn new(cmd: String, args: Vec<&'a str>) -> Self {
        Self { cmd, args }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Batch<'a> {
    cmds: Vec<Command<'a>>,
    input: Option<PathBuf>,
    output: Option<PathBuf>,
    is_async: bool,
}

impl<'a> Batch<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut commands: Vec<Command<'a>> = Vec::new();
        let mut redir_in: Option<PathBuf> = None;
        let mut redir_out: Option<PathBuf> = None;
        if !input.is_empty() {
            let limit: usize;
            let only_cmds_str: &str;
            if let Some(pos_in) = input.find('<') {
                if let Some(pos_out) = input.find('>') {
                    if pos_in > pos_out {
                        // cat | grep .txt > output.txt < input.txt
                        limit = pos_out;
                        let remainder: &str = &input[limit..];
                        let tokens: Vec<&str> = remainder.split("<").collect();
                        redir_out = Some(PathBuf::from_str(&tokens[0][1..].trim()).unwrap());
                        redir_in = Some(PathBuf::from_str(&tokens[1][1..].trim()).unwrap());
                    } else {
                        // cat | grep .txt < input.txt > output.txt
                        limit = pos_in;
                        let remainder: &str = &input[limit..];
                        let tokens: Vec<&str> = remainder.split(">").collect();
                        redir_in = Some(PathBuf::from_str(&tokens[0][1..].trim()).unwrap());
                        redir_out = Some(PathBuf::from_str(&tokens[1][1..].trim()).unwrap());
                    }
                } else {
                    limit = pos_in;
                    redir_in = Some(PathBuf::from_str(input[limit + 1..].trim()).unwrap());
                }
            } else {
                if let Some(pos_out) = input.find('>') {
                    limit = pos_out;
                    redir_out = Some(PathBuf::from_str(input[limit + 1..].trim()).unwrap());
                } else {
                    limit = input.len();
                }
            }

            only_cmds_str = &input[..limit];

            let tokens = only_cmds_str.trim().split('|');
            let count = tokens.clone().count();
            for (i, command) in tokens.enumerate() {
                // cmd_tokens = [ "cmd $1", "cmd $1 $2", "cmd" ]

                let cmd_tokens: Vec<&str> = command.trim().split_whitespace().collect();
                // If it's the last command, special care needs to be taken as there
                // could be input/output redirection.
                //
                if i == (count - 1) {}

                commands.push(Command::new(
                    cmd_tokens[0].to_string(),
                    cmd_tokens[1..].to_vec(),
                ));
            }
        }

        Self {
            cmds: commands,
            input: redir_in,
            output: redir_out,
            is_async: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_line() {
        let s = "";
        let job = Batch::new(s);
        assert_eq!(job.cmds, vec![]);
    }

    #[test]
    fn single_command_without_arguments<'a>() {
        let s = "echo";
        let args: Vec<&'a str> = Vec::new();
        let job = Batch::new(s);
        assert_eq!(job.cmds.len(), 1);
        assert_eq!(job.cmds[0].cmd, "echo");
        assert_eq!(job.cmds[0].args, args);
        assert_eq!(job.input, None);
        assert_eq!(job.output, None);
    }

    #[test]
    fn single_command_with_arguments() {
        let s = "wc -l file.txt";
        let job = Batch::new(s);
        assert_eq!(job.cmds.len(), 1);
        assert_eq!(job.cmds[0].cmd, "wc");
        assert_eq!(job.cmds[0].args, vec!["-l", "file.txt"]);
        assert_eq!(job.input, None);
        assert_eq!(job.output, None);
    }

    #[test]
    fn two_piped_commands() {
        let s = "cat file.txt | wc -l";
        let job = Batch::new(s);
        assert_eq!(job.cmds.len(), 2);
        assert_eq!(job.cmds[0].cmd, "cat");
        assert_eq!(job.cmds[0].args, vec!["file.txt"]);
        assert_eq!(job.cmds[1].cmd, "wc");
        assert_eq!(job.cmds[1].args, vec!["-l"]);
        assert_eq!(job.input, None);
        assert_eq!(job.output, None);
    }

    #[test]
    fn one_command_with_input_redirection() {
        let s = "cat < input.txt";
        let job = Batch::new(s);
        assert_eq!(job.cmds.len(), 1);
        assert_eq!(job.cmds[0].cmd, "cat");
        assert_eq!(job.input.unwrap().as_os_str(), "input.txt");
        assert!(job.cmds[0].args.is_empty());
        assert_eq!(job.output, None);
    }

    #[test]
    fn one_command_with_output_redirection() {
        let s = "cat input.txt > output.txt";
        let job = Batch::new(s);
        assert_eq!(job.cmds.len(), 1);
        assert_eq!(job.cmds[0].cmd, "cat");
        assert_eq!(job.output.unwrap().as_os_str(), "output.txt");
        assert_eq!(job.cmds[0].args, vec!["input.txt"]);
        assert_eq!(job.input, None);
    }

    #[test]
    fn one_command_with_input_and_output_redirection_1() {
        let s = "cat < input.txt > output.txt";
        let job = Batch::new(s);
        assert_eq!(job.cmds.len(), 1);
        assert_eq!(job.cmds[0].cmd, "cat");
        assert_eq!(job.input.unwrap().as_os_str(), "input.txt");
        assert_eq!(job.output.unwrap().as_os_str(), "output.txt");
        assert!(job.cmds[0].args.is_empty());
    }

    #[test]
    fn one_command_with_input_and_output_redirection_2() {
        let s = "cat > output.txt < input.txt";
        let job = Batch::new(s);
        assert_eq!(job.cmds.len(), 1);
        assert_eq!(job.cmds[0].cmd, "cat");
        assert_eq!(job.input.unwrap().as_os_str(), "input.txt");
        assert_eq!(job.output.unwrap().as_os_str(), "output.txt");
        assert!(job.cmds[0].args.is_empty());
    }
}
