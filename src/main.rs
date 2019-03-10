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
    static ref GIT_CONFIG: String = String::from(include_str!("repo_files/config"));
    static ref GIT_ATTRIBUTES: String = String::from(include_str!("repo_files/.gitattributes"));
    static ref GIT_IGNORE: String = String::from(include_str!("repo_files/.gitignore"));
    static ref GIT_README: String = String::from(include_str!("repo_files/README.md"));
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Pass args to git first.
    // For the most part, Ableton projects can be version controlled with unwrapped git.
    Command::new("git")
        .args(&args[1..])
        .spawn()
        .expect("Error occurred calling git. Is git installed?");

    // We're only wrapping init and clone to ensure the repo is set up correctly.
    let command = match args.get(1) {
        Some(command) => command,
        None => return
    };

    if !(command == "init" || command == "clone") {
        return;
    }

    let repo_directory = get_repo_directory(command, args.get(2), args.get(3));

    if command == "init" {
        copy_repo_files(&repo_directory);
    }

    append_config(&repo_directory);
}

fn get_repo_directory(command: &String, repository_resource_path: Option<&String>, project_name: Option<&String>) -> String {
    if command == "init" {
        return String::from(".");
    }

    match project_name {
        None => derive_project_name_from_repository_resource_path(repository_resource_path.unwrap()),
        Some(repository_path) => repository_path.clone(),
    }
}

fn derive_project_name_from_repository_resource_path(repository_path: &String) -> String {
    let project_name_regex = Regex::new("(?:.*/)(?P<project_name>.*)(?:.git)")
        .expect("Could not construct project name regex");

    return match project_name_regex.captures(repository_path) {
        Some(captures) => String::from(
            captures
                .name("project_name")
                .expect("Could not capture project name in repository path")
                .as_str(),
        ),
        None => {
            println!("Invalid repository path.");
            process::exit(1);
        }
    };
}

fn copy_repo_files(repo_directory: &String) {
    let mut git_attributes_file = File::create(format!("{}/.gitattributes", repo_directory))
        .expect("Could not create .gitattributes");
    git_attributes_file.write_all(GIT_ATTRIBUTES.as_bytes())
        .expect("Could not write to .gitattributes");

    let mut git_ignore_file = File::open(format!("{}/.gitignore", repo_directory))
        .expect("Could not create .gitignore");
    git_ignore_file.write_all(GIT_IGNORE.as_bytes())
        .expect("Could not write to .gitignore");

    let mut git_readme_file = File::open(format!("{}/README.md", repo_directory))
        .expect("Could not create README.md");
    git_readme_file.write_all(GIT_README.as_bytes())
        .expect("Could not write to README.md");
}

fn append_config(repo_directory: &String) {
    let mut repository_config_file = OpenOptions::new()
        .append(true)
        .open(format!("{}/.git/config", repo_directory))
        .expect("Could not open .git/config for appending");

    repository_config_file.write_all(GIT_CONFIG.as_bytes())
        .expect("Could not write to .git/config");
}

#[cfg(test)]
mod tests {
    use crate::get_repo_directory;

    #[test]
    fn get_repo_directory_for_init_is_pwd() {
        let command = String::from("init");
        let repository_resource_path = Option::None;
        let project_name = Option::None;
        let repo_directory = get_repo_directory(&command, repository_resource_path, project_name);
        assert_eq!(repo_directory, String::from("."));
    }
}
