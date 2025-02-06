use crate::{
    custom_error::{CustomError, CustomResult},
    logger::{Logger, LoggerTrait},
};
use futures::stream::{FuturesUnordered, StreamExt};
use std::path::Path;
use std::{
    collections::{HashMap, HashSet},
    process::{Command, Output},
};
use tokio::task;

use super::structs::ExecutionResult;

pub struct ParallelExecuter<'config> {
    root: &'config String,
    command: &'config String,
    repos: &'config Vec<String>,
}

impl<'config> LoggerTrait for ParallelExecuter<'config> {}

impl<'config> ParallelExecuter<'config> {
    pub fn new(
        root: &'config String,
        repos: &'config Vec<String>,
        command: &'config String,
    ) -> Self {
        ParallelExecuter {
            root,
            repos,
            command,
        }
    }

    async fn execute_for_repo<'repo>(
        path: &'repo String,
        command: &'repo String,
    ) -> CustomResult<Output> {
        let logger = Logger::new();
        logger.info(format!("Executing command: {}", command).as_str());

        let output_result = Command::new("zsh")
            .arg("-c")
            .arg(command)
            .current_dir(path)
            .output();

        let output = match output_result {
            Ok(output) => output,
            Err(e) => {
                logger.error(format!("Failed to execute command: {}", e).as_str());
                return Err(CustomError::CommandExecution(e.to_string()));
            }
        };

        if !output.status.success() {
            logger.error(format!("Failed to execute command for repo: {}", path).as_str());
            logger.error(format!("Error: {}", String::from_utf8_lossy(&output.stderr)).as_str());

            return Err(CustomError::CommandExecution(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }

        logger.info(format!("Executed command: {}", command).as_str());
        logger.info(format!("Output: {}", String::from_utf8_lossy(&output.stdout)).as_str());

        Ok(output)
    }
}

impl<'repo> ParallelExecuter<'repo> {
    pub async fn execute(&self) -> CustomResult<ExecutionResult> {
        let logger = self.get_logger();
        let mut failed_repos_hash: HashMap<String, String> = HashMap::new();
        let mut success_repos_set: HashSet<String> = HashSet::new();
        let mut tech_errors_set: HashSet<String> = HashSet::new();
        let mut tasks = FuturesUnordered::new();

        for repo in self.repos.iter() {
            let repo_path = Path::new(self.root)
                .join(repo)
                .to_str()
                .ok_or(CustomError::PathBuildException)?
                .to_string();

            logger.debug(format!("Executing command: {}", repo_path).as_str());
            let command = self.command.clone();
            let repo = repo.clone();

            tasks.push(task::spawn(async move {
                (
                    repo,
                    ParallelExecuter::execute_for_repo(&repo_path, &command).await,
                )
            }));
        }

        while let Some(result) = tasks.next().await {
            match result {
                Err(e) => {
                    tech_errors_set.insert(e.to_string());
                }
                Ok((repo, execution_result)) => match execution_result {
                    Err(execution_error) => {
                        failed_repos_hash.insert(repo, execution_error.to_string());
                    }
                    Ok(_) => {
                        success_repos_set.insert(repo);
                    }
                },
            }
        }

        Ok(ExecutionResult {
            failed: failed_repos_hash,
            succeed: success_repos_set,
            technical: tech_errors_set,
        })
    }
}
