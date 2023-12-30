use elastic_compose::config::Config;
use elastic_compose::server::{Server, ServerError};
use std::process::ExitCode;

fn main() -> ExitCode {
    let config = match Config::read_from_disk() {
        Err(config_error) => {
            println!("{}", config_error);
            return ExitCode::FAILURE;
        }
        Ok(config) => config,
    };

    let shutdown_handler = || Config::delete_from_disk().unwrap_or_else(|err| println!("{}", err));
    match Server::new(config.port)
        .with_shutdown_handler(shutdown_handler)
        .start()
    {
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
