pub mod args;
pub mod package;
pub mod config;
pub mod prompt;

fn main() {
    args::parse_args();
}
