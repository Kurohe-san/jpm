use colored::Colorize;
use serde::Deserialize;
use std::{fs, process::Command, io::{self, Write}};
use crate::config::Config;

#[derive(Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub dependencies: Vec<String>,
    pub upstream: String,
    pub system_files: Vec<String>,
    pub user_files: Vec<String>,
    pub install: Vec<Vec<String>>,
    pub remove: Vec<Vec<String>>,
    pub description: String,
}

impl Package {
    
    // Load a package from the current directory
    pub fn load(filename: &str) -> Package {
        let content = fs::read_to_string(filename).expect(format!("Error loading file: {}", filename).as_str());
        let ret: Package = serde_json::from_str(&content).unwrap();
        ret
    }

    // Load a package from the configured system package database path
    pub fn load_db_path(package_name: &str, conf: &Config) -> Package {
        Package::load(&format!("{}/{}.json", conf.package_db_path, package_name))
    }

    // List installed packages return a vector of package names
    pub fn list_installed_packages(conf: &Config) -> Vec<String> {
        fs::read_dir(&conf.package_build_path).unwrap().filter_map(|p| p.ok()).map(|p| p.file_name().into_string().unwrap()).collect::<Vec<String>>()
    }

    // List all dependencies of a package recursively and return a vector of package names
    pub fn list_dependencies(&self) -> Vec<String> {
        let mut ret: Vec<String> = Vec::new();
        ret.push(self.name.clone());
        self.dependencies.iter().for_each(|p| {
            let p = Package::load_db_path(p, &Config::load_config());
            ret.append(&mut p.list_dependencies());
        });
        ret
    }
    
    // Install a package
    pub fn sys_install(&self, conf: &Config) {
        let inst_dir = format!("{}/{}", conf.package_build_path, self.name);
        run_command("mkdir", &["-pv", &inst_dir], ".", conf);
        run_command("git", &["clone", &self.upstream, &inst_dir], ".", conf);
        for c in self.install.iter() {
            run_command(&c[0], &c[1..].iter().map(|s| s.as_str()).collect::<Vec<&str>>(), &inst_dir, conf);
        }
    }

    // Remove an installed package
    pub fn sys_remove(&self, conf: &Config) {
        let inst_dir = format!("{}/{}", conf.package_build_path, self.name);
        for c in self.remove.iter() {
            run_command(&c[0], &c[1..].iter().map(|s| s.as_str()).collect::<Vec<&str>>(), &inst_dir, conf);
        }
        run_command("rm", &["-rf", &inst_dir], ".", conf);
    }

    // Search for packages in the database path
    pub fn search(query: &str, config: &Config) {
        let entries = fs::read_dir(&config.package_db_path).unwrap();
        let results = entries.map(|entry| entry.unwrap().file_name().into_string().unwrap()).filter(|entry| entry.contains(query));
        for mut result in results {
            let p = Package::load(&format!("{}/{}",config.package_db_path,result));
            result.truncate(result.len() - 5);
            println!("{}\n   » {}", format!("- {}",result).red(), format!("{}",p.description).yellow());
        }
    }

}

// Run a command with the given arguments and installation directory of the package
fn run_command(command: &str, args: &[&str], run_dir: &str, conf: &Config) {
    let mut args = args.to_vec();
    args.insert(0, command);
    let comm = &conf.elevated_privileges;
    let output = Command::new(comm).args(args).current_dir(run_dir).output().expect("Error running command");
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}