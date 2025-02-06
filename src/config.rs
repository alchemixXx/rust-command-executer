use serde_derive::Deserialize;

use std::fs;

use crate::logger::LogLevel;

#[derive(Debug, Deserialize)]
pub struct WorkersConfig {
    workers: Box<[String]>,
}

#[derive(Debug, Deserialize)]
pub struct LoggerConfig {
    pub log_level: LogLevel,
}
// Top level struct to hold the TOML data.
#[derive(Debug, Deserialize)]
pub struct Data {
    pub root: String,
    pub command: String,
    pub repos: WorkersConfig,
    pub logger: LoggerConfig,
    pub parallel: bool,
}

impl WorkersConfig {
    pub fn get_repos_list(&self) -> Vec<String> {
        let mut repos: Vec<String> = Vec::with_capacity(self.workers.len());
        for repo in self.workers.iter() {
            repos.push(repo.to_string());
        }

        repos
    }
}

pub fn read_config(path: &str) -> Data {
    println!("Reading config file: {}", path);
    let contents =
        fs::read_to_string(path).unwrap_or_else(|_| panic!("Could not read file `{path}`"));

    let data: Data =
        toml::from_str(&contents).unwrap_or_else(|_| panic!("Unable to load data from `{path}`"));
    println!("Read config file: {}. {:#?}", path, data);

    data
}
