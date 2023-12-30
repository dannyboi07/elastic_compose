use clap::{Parser, ValueEnum};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum ClapCommand {
    /// Start the service
    Start,
    /// Stop the service
    Stop,
    // Restart,
    // Status,
    // Logs,
    // Config,
    // Help,
}

pub enum Command {
    Start,
    Stop,
}

#[derive(Parser)]
struct ClapArgs {
    /// Command to run
    #[arg(short, value_enum)] // , possible_values = &["start", "stop"]
    command: ClapCommand,
    /// Port for the service's server to listen on (defaults to 8080)
    #[arg(long)]
    port: Option<u16>,
    // Path to the config file
    // #[arg(short, default_value = "config.yaml")]
    // config: String,
    // #[arg(long)]
    // replace: Option<bool>,
    #[arg(long, num_args = 0.., value_delimiter = ' ')]
    projects: Option<Vec<String>>,
}

pub struct Args {
    pub command: Command,
    pub port: Option<u16>,
    // pub replace: bool,
    pub projects: Option<Vec<String>>,
}

pub enum ValidationError {
    // NoCommand,
    NoPort,
    NoProjects,
}

impl Args {
    pub fn get() -> Result<Args, ValidationError> {
        let args = ClapArgs::parse();
        let command = match args.command {
            ClapCommand::Start => {
                if args.port == None {
                    return Err(ValidationError::NoPort);
                } else if args.projects == None {
                    return Err(ValidationError::NoProjects);
                }
                Command::Start
            }
            ClapCommand::Stop => Command::Stop,
        };

        Ok(Args {
            command,
            port: args.port,
            // replace: match args.replace {
            //     Some(bool) => bool,
            //     None => false,
            // },
            projects: args.projects,
        })
    }
}
