pub mod args;
pub mod package;
pub mod config;
pub mod prompt;

fn main() {
    args::parse_args();
    //package::Package::load_db_path("test1", &config::Config::load_config()).list_dependencies().iter().for_each(|p| println!("{}", p));
}
