#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::env;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::process;
use std::process::Command;
use std::string::String;

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

    let repo_directory =
        if command == "init" {
            get_repo_directory_for_init(git_output.stdout)
        } else {
            get_repo_directory_for_clone(git_output.stderr)
        };

    if command == "init" {
        let mut git_attributes_file =
            File::create(format!("{}/.gitattributes", repo_directory))
                .expect("Could not create .gitattributes");
        git_attributes_file
            .write_all(GIT_ATTRIBUTES.as_bytes())
            .expect("Could not write to .gitattributes");

        let mut git_ignore_file =
            File::create(format!("{}/.gitignore", repo_directory))
                .expect("Could not create .gitignore");
        git_ignore_file
            .write_all(GIT_IGNORE.as_bytes())
            .expect("Could not write to .gitignore");

        let mut git_readme_file =
            File::create(format!("{}/README.md", repo_directory))
                .expect("Could not create README.md");
        git_readme_file
            .write_all(GIT_README.as_bytes())
            .expect("Could not write to README.md");
    }

    let mut repository_config_file = OpenOptions::new()
        .append(true)
        .open(format!("{}/.git/config", repo_directory))
        .expect("Could not open .git/config for appending");

    repository_config_file
        .write_all(GIT_CONFIG.as_bytes())
        .expect("Could not write to .git/config");
}

fn get_repo_directory_for_init(git_stdout: Vec<u8>) -> String {
    let git_stdout = String::from_utf8(git_stdout)
        .expect("Failed to convert stdout output from git into a string.");

    String::from(".")
}

fn get_repo_directory_for_clone(git_stderr: Vec<u8>) -> String {
    let git_stderr = String::from_utf8(git_stderr)
        .expect("Failed to convert stderr output from git into a string.");

    let repo_directory_regex =
        Regex::new("(Cloning into \')(?P<repo_directory>.*)(\'...\n)")
            .expect("Could not construct repo directory regex");

    match repo_directory_regex.captures(&git_stderr) {
        Some(captures) => String::from(
            captures
                .name("repo_directory")
                .expect("Could not capture repo directory from stderr output.")
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
    use crate::get_repo_directory_for_init;
    use crate::get_repo_directory_for_clone;

    #[test]
    fn get_repo_directory_for_init_is_pwd() {
        let git_stdout = Vec::new();
        let repo_directory = get_repo_directory_for_init(git_stdout);
        assert_eq!(repo_directory, String::from("."));
    }

//    #[test]
//    fn get_repo_directory_for_clone_is_() {
//        let command = String::from("init");
//        let git_stderr = String::from("");
//        let repo_directory = get_repo_directory(&command, &git_stderr);
//        assert_eq!(repo_directory, String::from("."));
//    }
}
