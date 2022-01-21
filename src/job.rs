use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Command<'a> {
    pub cmd: String,
    pub args: Vec<&'a str>,
}

impl<'a> Command<'a> {
    pub fn new(cmd: String, args: Vec<&'a str>) -> Self {
        Self { cmd, args }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Job<'a> {
    pub cmds: Vec<Command<'a>>,
    pub input: Option<PathBuf>,
    pub output: Option<PathBuf>,
}

impl<'a> Job<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut commands: Vec<Command<'a>> = Vec::new();
        if !input.is_empty() {
            for command in input.trim().split('|') {
                let tokens: Vec<&str> = command.trim().split_whitespace().collect();
                commands.push(Command::new(tokens[0].to_string(), tokens[1..].to_vec()));
            }
        }

        Self {
            cmds: commands,
            input: None,
            output: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_line() {
        let s = "";
        let job = Job::new(s);
        assert_eq!(job.cmds, vec![]);
    }

    #[test]
    fn single_command_without_arguments<'a>() {
        let s = "echo";
        let args: Vec<&'a str> = Vec::new();
        let job = Job::new(s);
        assert_eq!(job.cmds.len(), 1);
        assert_eq!(job.cmds[0].cmd, "echo");
        assert_eq!(job.cmds[0].args, args);
    }

    #[test]
    fn single_command_with_arguments() {
        let s = "wc -l file.txt";
        let job = Job::new(s);
        assert_eq!(job.cmds.len(), 1);
        assert_eq!(job.cmds[0].cmd, "wc");
        assert_eq!(job.cmds[0].args, vec!["-l", "file.txt"]);
    }

    #[test]
    fn two_piped_commands() {
        let s = "cat file.txt | wc -l";
        let job = Job::new(s);
        assert_eq!(job.cmds.len(), 2);
        assert_eq!(job.cmds[0].cmd, "cat");
        assert_eq!(job.cmds[0].args, vec!["file.txt"]);
        assert_eq!(job.cmds[1].cmd, "wc");
        assert_eq!(job.cmds[1].args, vec!["-l"]);
    }
}
