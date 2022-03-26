use std::io;
use colored::Colorize;

pub fn ask_continue(msg: Option<&str>) -> bool {
    let mut answer = String::new();
    match msg {
        Some(m) => println!("{} [Y/n]", m.yellow()),
        None => println!("{} [Y/n]",format!("Do you wish to continue?").yellow()),
    }
    io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read line from standard input");

    answer.trim().to_lowercase().as_str() == "y"
}