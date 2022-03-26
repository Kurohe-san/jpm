use colored::Colorize;
use serde::Deserialize;
use std::{fs, process::Command, env, io::{self, Write}};
use crate::{config::Config, prompt};

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
    
    pub fn load(filename: &str) -> Package {
        let content = fs::read_to_string(filename).expect(format!("Error loading file: {}", filename).as_str());
        let ret: Package = serde_json::from_str(&content).unwrap();
        ret
    }

    pub fn load_db_path(package_name: &str, conf: &Config) -> Package {
        Package::load(&format!("{}/{}.json", conf.package_db_path, package_name))
    }

    pub fn sys_install(&self) {
        let inst_dir = format!("{}/.cache/jpm/{}", env::var("HOME").unwrap(), self.name);
        run_command("mkdir", &["-pv", &inst_dir], ".", false);
        run_command("git", &["clone", &self.upstream, &inst_dir], ".", false);
        println!("{}", inst_dir);
        for c in self.install.iter() {
            run_command(&c[0], &c[1..].iter().map(|s| s.as_str()).collect::<Vec<&str>>(), &inst_dir, true);
        }
    }
    pub fn sys_remove(&self) {
        let inst_dir = format!("{}/.cache/jpm/{}", env::var("HOME").unwrap(), self.name);
        for c in self.remove.iter() {
            run_command(&c[0], &c[1..].iter().map(|s| s.as_str()).collect::<Vec<&str>>(), &inst_dir, true);
        }
        run_command("rm", &["-rf", &inst_dir], ".", false);
    }

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
fn run_command(command: &str, args: &[&str], run_dir: &str, needs_root: bool) {
    let mut comm = command;
    let mut args = args.to_vec();
    if needs_root {
        args.insert(0, command);
        comm = "sudo";
    }
    let output = Command::new(comm).args(args).current_dir(run_dir).output().expect("Error running command");
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}