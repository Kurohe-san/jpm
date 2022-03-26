use std::env;
use colored::Colorize;

use crate::{package::Package, config::Config, prompt};

mod commands {
    pub const INSTALL: &str = "install";
    pub const REMOVE: &str = "remove";
    pub const SEARCH: &str = "search";
    pub const FLAG_RECURSE: &str = "-r";
}

pub fn parse_args() {
    let a: Vec<_> = env::args().collect();
    if a.len() == 1 {
        println!("Usage: jpm <command> <package> [<additional flags>]");
    }
    else {
        match a[1].as_str() {
            commands::INSTALL => install(a[2].as_str()) ,
            commands::REMOVE => remove(a[2].as_str(), a.iter().filter(|s| s.as_str() == commands::FLAG_RECURSE).collect::<Vec<&String>>().len() > 0),
            commands::SEARCH => search(a[2].as_str()),
            _ => println!("Error: unknown command"),
        }
    }
}

fn install(package_name: &str) {
    let conf = Config::load_config();
    let p = Package::load_db_path(package_name, &conf);
    if Package::list_installed_packages(&conf).contains(&p.name) {
        println!("{}", format!("Warning: `{}` {} is already installed, reinstalling...", p.name, p.version).yellow());
        if prompt::ask_continue(Some("Do you want to reinstall it?")){
            remove(&p.name, false);
        }
        else {
            return;
        }
    }
    else {
        println!("{}",format!("Installing `{}` {}...", p.name, p.version).green());
    }
    if p.dependencies.len() > 0 {
        println!("{}",format!("The following dependencies will be installed:").yellow());
        let installed_packages = Package::list_installed_packages(&conf); 
        let all_deps = p.list_dependencies();
        let install_packages = all_deps.iter().filter(|s| !installed_packages.contains(s)).collect::<Vec<&String>>();
        install_packages.iter().skip(1).for_each(|f| println!("{}  {}", format!("   |").yellow(), format!("{}",f).bold()));
        if prompt::ask_continue(None) {
            install_packages.iter().for_each(|p| Package::load_db_path(p, &conf).sys_install(&conf));
        }
    }
    else if prompt::ask_continue(None) {
        p.sys_install(&conf);
    }
}

fn remove(package_name: &str, recurse: bool) {
    let conf = Config::load_config();
    let p = Package::load_db_path(package_name, &conf);
    if !Package::list_installed_packages(&conf).contains(&p.name) {
        println!("{}",format!("Error: Package `{}` is not installed", p.name).red());
        return;
    }
    println!("Removing `{}`...", p.name);
    if recurse && p.dependencies.len() > 0 {
        println!("{}",format!("The following packages will be removed:").yellow());
        p.dependencies.iter().for_each(|f| println!("{}  {}", format!("   |").yellow(), format!("{}",f).bold()));
        println!("{}  {}", format!("   |").yellow(), format!("{}", p.name).bold());
        if prompt::ask_continue(None) {
            p.dependencies.iter().for_each(|p| Package::load_db_path(p, &conf).sys_remove(&conf));
            p.sys_remove(&conf);
        }
    }
    else if prompt::ask_continue(None) {
        p.sys_remove(&conf);
    }
}

fn search(query: &str) {
    println!("Searching for {}...", query);
    Package::search(query, &Config::load_config());
}