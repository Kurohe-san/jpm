use std::env;
use colored::Colorize;

use crate::{package::Package, config::Config, prompt};

mod commands {
    pub const INSTALL: &str = "install";
    pub const REMOVE: &str = "remove";
    pub const SEARCH: &str = "search";
}

pub fn parse_args() {
    let a: Vec<_> = env::args().map(|s| s.clone()).collect();
    if a.len() == 1 {
        println!("Usage: jpm <command> <package>");
    }
    else {
        match a[1].as_str() {
            commands::INSTALL => install(a[2].as_str()),
            commands::REMOVE => remove(a[2].as_str()),
            commands::SEARCH => search(a[2].as_str()),
            _ => println!("Error: unknown command"),
        }
    }
}

fn install(package_name: &str) {
    println!("{}",format!("Installing {}...", package_name).green());
    let conf = Config::load_config();
    let p = Package::load_db_path(package_name, &conf);
    if p.dependencies.len() > 0 {
        println!("{}",format!("The following dependencies will need to be installed:").yellow());
        p.dependencies.iter().for_each(|f| println!("{}", format!("   | {}", f).yellow()));
        if prompt::ask_continue() {
            p.dependencies.iter().for_each(|p| Package::load_db_path(p, &conf).sys_install());
            p.sys_install();
        }
    }
    else if prompt::ask_continue() {
        p.sys_install();
    }
}

fn remove(package_name: &str) {
    println!("Removing {}...", package_name);
    let p = Package::load(format!("{}.json", package_name).as_str());
    p.sys_remove();    
}

fn search(query: &str) {
    println!("Searching for {}...", query);
    Package::search(query, &Config::load_config());
}