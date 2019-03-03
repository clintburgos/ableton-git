extern crate regex;

use regex::Regex;
use std::env;
use std::process;
use std::process::Command;
use std::string::String;

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

    let repo_directory = get_repo_directory(args);

    println!("{}", repo_directory);

}

fn get_repo_directory(args: Vec<String>) -> String {
    if &args[1] == "init" {
        return String::from(".");
    }

    match args.get(3) {
        None => derive_project_name_from_repository_path(&args[2]),
        _ => args[3].clone()
    }
}

fn derive_project_name_from_repository_path(repository_path: &String) -> String {
    let project_name_regex = Regex::new("(?:.*/)(?P<project_name>.*)(?:.git)").unwrap();
    return match project_name_regex.captures(repository_path) {
        Some(captures) => String::from(captures.name("project_name").unwrap().as_str()),
        None => {
            println!("Invalid repository path.");
            process::exit(1);
        }
    };
}
