// mod cmd;
mod cli;
mod config;
mod server;

use std::process::ExitCode;

// use cmd::Args;
use config::Config;
use server::{Server, ServerError};

fn main() -> ExitCode {
    // let args = Args::get(config::Mode::Service);
    let config = match Config::read_from_disk() {
        Err(config_error) => {
            println!("{}", config_error);
            return ExitCode::FAILURE;
        }
        Ok(config) => config,
    };

    match Server::new(config.port).start() {
        Err(err) => match err {
            ServerError::ListenError(err) | ServerError::ServeError(err) => {
                println!("{}", err);
                return ExitCode::FAILURE;
            }
        },
        Ok(_) => {}
    };

    // To be removed
    ExitCode::SUCCESS
}
