use crate::{
    custom_error::{CustomError, CustomResult},
    logger::{Logger, LoggerTrait},
};
use std::path::Path;
use std::{
    collections::{HashMap, HashSet},
    process::{Command, Output},
};

pub struct ExecutionResult {
    pub failed: HashMap<String, String>,
    pub succeed: HashSet<String>,
}

pub struct Executer<'config> {
    root: &'config String,
    command: &'config String,
    repos: &'config Vec<String>,
}

impl<'config> LoggerTrait for Executer<'config> {}

impl<'config> Executer<'config> {
    pub fn new(
        root: &'config String,
        repos: &'config Vec<String>,
        command: &'config String,
    ) -> Self {
        Executer {
            root,
            repos,
            command,
        }
    }
}

impl<'repo> Executer<'repo> {
    pub fn execute(&self) -> CustomResult<ExecutionResult> {
        let logger = Logger::new();
        let mut failed_repos_hash: HashMap<String, String> = HashMap::new();
        let mut success_repos_set: HashSet<String> = HashSet::new();

        for repo in self.repos.iter() {
            let repo_path = Path::new(self.root)
                .join(repo)
                .to_str()
                .ok_or(CustomError::PathBuildException)?
                .to_string();

            logger.debug(format!("Executing command: {}", repo_path).as_str());

            let output = self.execute_for_repo(&repo_path);

            match output {
                Ok(_) => {
                    logger.debug(
                        format!("Command executed successfully in repo: {}", repo_path).as_str(),
                    );
                    success_repos_set.insert(repo.to_string());
                }
                Err(e) => {
                    logger.error(format!("Error: {}", e).as_str());
                    failed_repos_hash.insert(repo.clone(), e.to_string());
                }
            }

            logger.debug(format!("Executed command in repo: {}", repo_path).as_str());
        }

        Ok(ExecutionResult {
            failed: failed_repos_hash,
            succeed: success_repos_set,
        })
    }

    fn execute_for_repo(&self, path: &'repo String) -> CustomResult<Output> {
        let logger = Logger::new();
        logger.info(format!("Executing command: {}", self.command).as_str());

        let output_result = Command::new("zsh")
            .arg("-c")
            .arg(self.command)
            .current_dir(path)
            .output();

        let output = match output_result {
            Ok(output) => output,
            Err(e) => {
                logger.error(format!("Failed to execute command: {}", e).as_str());
                return Err(CustomError::CommandExecution);
            }
        };

        if !output.status.success() {
            logger.error(format!("Failed to execute command for repo: {}", path).as_str());
            logger.error(format!("Error: {}", String::from_utf8_lossy(&output.stderr)).as_str());

            return Err(CustomError::CommandExecution);
        }

        logger.info(format!("Executed command: {}", self.command).as_str());
        logger.info(format!("Output: {}", String::from_utf8_lossy(&output.stdout)).as_str());

        Ok(output)
    }
}
