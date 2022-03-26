use std::io;
use colored::Colorize;

pub fn ask_continue() -> bool {
    let mut answer = String::new();
    println!("{}",format!("Do you wish to continue? [Y/n]").red());
    io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read line from standard input");

    &answer.to_lowercase() == "y"
}