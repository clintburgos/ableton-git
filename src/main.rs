#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::process;
use std::process::Command;
use std::string::String;
use std::{env, io};

lazy_static! {
    static ref GIT_CONFIG: String = String::from(include_str!("text/config"));
    static ref GIT_ATTRIBUTES: String = String::from(include_str!("text/.gitattributes"));
    static ref GIT_IGNORE: String = String::from(include_str!("text/.gitignore"));
    static ref GIT_README: String = String::from(include_str!("text/README.md"));
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Pass args to git first.
    // For the most part, Ableton projects can be version controlled with unwrapped git.
    let git_output = Command::new("git")
        .args(&args[1..])
        .output()
        .expect("Error occurred calling git. Is git installed?");

    // print the output from git
    io::stdout().write_all(&git_output.stdout).unwrap();
    io::stderr().write_all(&git_output.stderr).unwrap();

    // Do not proceed if git had a failure.
    if !git_output.status.success() {
        return;
    }

    // We're only wrapping init and clone to ensure the repo is set up correctly.
    let command = match args.get(1) {
        Some(command) => command,
        None => return,
    };

    if !(command == "init" || command == "clone") {
        return;
    }

    let repo_directory = if command == "init" {
        get_repo_directory_for_init(git_output.stdout)
    } else {
        get_repo_directory_for_clone(git_output.stderr)
    };

    if command == "init" {
        File::create(format!("{}/.gitattributes", repo_directory))
            .and_then(|mut f| f.write_all(GIT_ATTRIBUTES.as_bytes()))
            .expect("Could not create or write to .gitattributes");

        File::create(format!("{}/.gitignore", repo_directory))
            .and_then(|mut f| f.write_all(GIT_IGNORE.as_bytes()))
            .expect("Could not create or write to .gitignore");

        File::create(format!("{}/README.md", repo_directory))
            .and_then(|mut f| f.write_all(GIT_README.as_bytes()))
            .expect("Could not create or write to README.md");
    }

    // write to .gitconfig on both init and clone
    OpenOptions::new()
        .append(true)
        .open(format!("{}/.git/config", repo_directory))
        .and_then(|mut f| f.write_all(GIT_CONFIG.as_bytes()))
        .expect("Could not open or write to .git/config");
}

fn get_repo_directory_for_init(git_stdout: Vec<u8>) -> String {
    let git_stdout = String::from_utf8(git_stdout)
        .expect("Failed to convert stdout output from git into a string.");

    let repo_directory_regex =
        Regex::new("(Initialized empty |Reinitialized existing )(Git repository in )(?P<repo_directory>.*)(/.git/\n)")
            .expect("Could not construct repo directory regex");

    capture_repo_directory_with_regex(&git_stdout, repo_directory_regex)
}

fn get_repo_directory_for_clone(git_stderr: Vec<u8>) -> String {
    let git_stderr = String::from_utf8(git_stderr)
        .expect("Failed to convert stderr output from git into a string.");

    let repo_directory_regex = Regex::new("(Cloning into \')(?P<repo_directory>.*)(\'...\n)")
        .expect("Could not construct repo directory regex");

    capture_repo_directory_with_regex(&git_stderr, repo_directory_regex)
}

fn capture_repo_directory_with_regex(string: &str, repo_directory_regex: Regex) -> String {
    match repo_directory_regex.captures(string) {
        Some(captures) => String::from(
            captures
                .name("repo_directory")
                .expect("Could not capture repo directory from git output.")
                .as_str(),
        ),
        None => {
            println!("Ableton-git received unexpected output from git.");
            process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::get_repo_directory_for_clone;
    use crate::get_repo_directory_for_init;

    #[test]
    fn test_get_repo_directory_for_init() {
        let git_stdout = Vec::from("Initialized empty Git repository in /Users/clintonburgos/Documents/Projects/2019/test Project/.git/\n");
        let repo_directory = get_repo_directory_for_init(git_stdout);
        assert_eq!(
            repo_directory,
            String::from("/Users/clintonburgos/Documents/Projects/2019/test Project")
        );
    }

    #[test]
    fn test_get_repo_directory_for_reinit() {
        let git_stdout = Vec::from("Reinitialized existing Git repository in /Users/clintonburgos/Documents/Projects/2019/test Project/.git/\n");
        let repo_directory = get_repo_directory_for_init(git_stdout);
        assert_eq!(
            repo_directory,
            String::from("/Users/clintonburgos/Documents/Projects/2019/test Project")
        );
    }

    #[test]
    fn test_get_repo_directory_for_clone() {
        let git_stderr = Vec::from("Cloning into \'testdir/test\'...\n");
        let repo_directory = get_repo_directory_for_clone(git_stderr);
        assert_eq!(repo_directory, String::from("testdir/test"));
    }
}
