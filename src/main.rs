use elastic_compose::cli;
use elastic_compose::config::Config;
use std::process;

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
            match config.flush_to_disk() {
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
                            println!("Failed to start the service, err: {}", err);
                            return;
                        }
                        Ok(status) => {
                            if !status.success() {
                                println!("Failed to start the service");
                                return;
                            }
                            println!("Initiated service")
                        }
                    };
                }
            };
        }
        cli::Command::Stop => {
            match process::Command::new("systemd")
                .args(["stop", "elastic-compose-service"])
                .status()
            {
                Err(err) => {
                    println!("Failed to stop the service, err: {}", err);
                    return;
                }
                Ok(status) => {
                    if !status.success() {
                        println!("Failed to stop the service");
                        return;
                    }
                    println!("Service stopped")
                }
            }
        }
    }
}
