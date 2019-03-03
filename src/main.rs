use std::env;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Pass args to git first.
    Command::new("git")
            .args(&args[1..])
            .spawn()
            .expect("Error occurred calling git. Is git installed?");

}
