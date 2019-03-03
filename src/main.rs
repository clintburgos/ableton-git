extern crate fs_extra;
extern crate regex;

use regex::Regex;
use std::env;
use std::process;
use std::process::Command;
use std::string::String;
use fs_extra::dir;
use std::fs::OpenOptions;
use std::fs;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Pass args to git first.
    // For the most part, Ableton projects can be version controlled with unwrapped git.
//    Command::new("git")
//            .args(&args[1..])
//            .spawn()
//            .expect("Error occurred calling git. Is git installed?");

    // We're only wrapping init and clone to ensure the repo is set up correctly.
    if !(&args[1] == "init" || &args[1] == "clone") {
        return;
    }

    let repo_directory = get_repo_directory(&args);

    if &args[1] == "init" {
        copy_repo_files(&repo_directory);
    }

    append_config(&repo_directory);
}

fn get_repo_directory(args: &Vec<String>) -> String {
    if &args[1] == "init" {
        return String::from(".");
    }

    match args.get(3) {
        None => derive_project_name_from_repository_path(&args[2]),
        _ => args[3].clone()
    }
}

fn derive_project_name_from_repository_path(repository_path: &String) -> String {
    let project_name_regex = Regex::new("(?:.*/)(?P<project_name>.*)(?:.git)")
        .expect("Could not construct project name regex");

    return match project_name_regex.captures(repository_path) {
        Some(captures) => String::from(captures.name("project_name")
            .expect("Could not capture project name in repository path").as_str()),
        None => {
            println!("Invalid repository path.");
            process::exit(1);
        }
    };
}

fn copy_repo_files(repo_directory: &String) {
    dir::copy("repo_files", repo_directory, &dir::CopyOptions::new());
}

fn append_config(repo_directory: &String) {
    let mut repository_config_file = OpenOptions::new()
        .append(true)
        .open(format!("{}/.git/config", repo_directory))
        .expect("Could not open .git/config for appending");

    let config_to_append = fs::read_to_string("src/templates/config.txt")
        .expect("Could not read config template");

    repository_config_file.write_all(config_to_append.as_bytes());
}
