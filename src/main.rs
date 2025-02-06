use clap::Parser;
use custom_error::CustomResult;
mod cli;
mod config;
mod custom_error;
mod logger;
mod workers;

use crate::logger::Logger;
use cli::CLi;
use workers::executer::Executer;
use workers::parallel_executer::ParallelExecuter;

#[tokio::main]
async fn main() -> CustomResult<()> {
    println!("Reading cli args...");
    let cli_args = CLi::parse();
    println!("CLI args: {:#?}", cli_args);

    let config = config::read_config(&cli_args.path);

    crate::logger::Logger::init(config.logger.log_level);
    let logger = Logger::new();
    logger.info("Command executer started!");
    let repos = config.repos.get_repos_list();
    logger.info(format!("Repos to execute: {:#?}", repos).as_str());

    let start = std::time::Instant::now();

    println!("Executer started at: {:?}", std::time::Instant::now());

    let result: workers::structs::ExecutionResult = if config.parallel {
        let executer = ParallelExecuter::new(&config.root, &repos, &config.command);

        executer.execute().await?
    } else {
        let executer = Executer::new(&config.root, &repos, &config.command);

        executer.execute().await?
    };

    let end = std::time::Instant::now();

    println!(
        "Executer finished. Total time: {:?}",
        end.duration_since(start)
    );

    logger.warn(format!("Failed repos: {:#?}", result.failed).as_str());
    logger.warn(format!("Technical errors: {:#?}", result.technical).as_str());
    logger.warn(format!("Successes repos: {:#?}", result.succeed).as_str());
    logger.info("Command executer finished!");

    Ok(())
}
