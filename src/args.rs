use std::env;
use crate::{package::Package, config::Config};

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
    println!("Installing {}...", package_name);
    let p = Package::load(format!("{}.json", package_name).as_str());
    p.sys_install();
}

fn remove(package_name: &str) {
    println!("Removing {}...", package_name);
    let p = Package::load(format!("{}.json", package_name).as_str());
    p.sys_remove();    
}

fn search(query: &str) {
    println!("Searching for {}...", query);
    Package::search(query, Config::load_config());
}