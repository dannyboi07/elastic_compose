// mod cmd;
mod cli;
mod config;
mod server;

// use axum::{http::status, Error};
// use cli::{Args, ValidationError, Command};
use config::Config;
// use server::{Server, ServerError};
use std::{error, process};

// #[tokio::main]
fn main() {
    let args = match cli::Args::get() {
        Err(err) => match err {
            cli::ValidationError::NoPort => {
                println!("Missing 'port' argument");
                return;
            }
            cli::ValidationError::NoProjects => {
                println!("Missing 'projects' argument");
                return;
            }
        },
        Ok(args) => args,
    };

    match args.command {
        cli::Command::Start => {
            let config = match Config::new(&args) {
                Err(err) => {
                    println!("{}", err);
                    return;
                }
                Ok(config) => config,
            };
            match config.flush() {
                Err(err) => {
                    println!("{}", err);
                    return;
                }
                Ok(()) => {
                    match process::Command::new("systemd")
                        .args(["start", "elastic-compose-service"])
                        .status()
                    {
                        Err(err) => {
                            println!("Failed to start service, err: {}", err);
                            return;
                        }
                        Ok(status) => {
                            if !status.success() {
                                println!("Failed to start service");
                                return;
                            }
                            println!("Initiated service")
                        }
                    };
                }
            };
        }
        cli::Command::Stop => {}
    }

    // let config = match Config::new_from_cmd_args(&args) {
    //     Err(config_error) => {
    //         println!("{}", config_error);
    //         // return ExitCode::FAILURE;
    //     }
    //     Ok(config) => config,
    // };

    // if config.projects.len() == 0 {
    //     println!("No projects to watch, exiting...");
    // return ExitCode::SUCCESS;
    // }

    // match Server::new(config.port).start().await {
    //     Err(err) => match err {
    //         ServerError::ListenError(err) | ServerError::ServeError(err) => {
    //             println!("{}", err);
    //             return ExitCode::FAILURE;
    //         }
    //     },
    //     Ok(_) => {}
    // };

    // To remove exit
    // ExitCode::SUCCESS
}

// fn start_service() -> Result<(), error::Error> {
//     match process::Command::new("systemd")
//         .args(["start", "elastic-compose-service"])
//         .status()
//     {
//         Err(err) => return Err(error::Error::new(err.to_string())),
//         Ok(status) => {
//             if !status.success() {
//                 return Err(error::Error::new("Failed to start service"));
//             }
//             return Ok(());
//         }
//     };
// }
