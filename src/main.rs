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
use workers::loginer::login;

fn main() -> CustomResult<()> {
    println!("Reading cli args...");
    let cli_args = CLi::parse();
    println!("CLI args: {:#?}", cli_args);

    let config = config::read_config(&cli_args.path);

    crate::logger::Logger::init(config.logger.log_level);
    let logger = Logger::new();
    logger.info("Command executer started!");
    let repos = config.repos.get_repos_list();
    logger.info(format!("Repos to execute: {:#?}", repos).as_str());

    let mut result_string = String::new();
    result_string.push('\n');

    if config.aws.login_required {
        logger.debug("Logging in to AWS...");
        login(
            &config.git.branch,
            &config.aws.role_script_path,
            &config.aws.role,
        );
        logger.debug("Logged in to AWS");
    }

    let executer = Executer::new(&config.root, &repos, &config.command);

    let result = executer.execute()?;

    logger.warn(format!("Failed repos: {:#?}", result.failed).as_str());
    logger.warn(format!("Successes repos: {:#?}", result.succeed).as_str());
    logger.info("Command executer finished!");

    Ok(())
}
