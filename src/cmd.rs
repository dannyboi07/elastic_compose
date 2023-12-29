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
    // Restart,
    // Status,
    // Logs,
    // Config,
    // Help,
}

impl Command {
    pub fn to_str(&self) -> &str {
        match self {
            Command::Start => "start",
            Command::Stop => "stop",
            // Command::Restart => "restart",
            // Command::Status => "status",
            // Command::Logs => "logs",
            // Command::Config => "config",
            // Command::Help => "help",
        }
    }
}

// impl fmt::Display for Command {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             Command::Start => write!(f, "start"),
//             Command::Stop => write!(f, "stop"),
//             // Command::Restart => write!(f, "restart"),
//             // Command::Status => write!(f, "status"),
//             // Command::Logs => write!(f, "logs"),
//             // Command::Config => write!(f, "config"),
//             // Command::Help => write!(f, "help"),
//         }
//     }
// }

#[derive(Parser)]
// #[command(author)]
struct ClapAppArgs {
    /// Command to run
    #[arg(short, value_enum)] // , possible_values = &["start", "stop"]
    command: ClapCommand,
    /// Port for the service's server to listen on
    #[arg(long)]
    port: u16,
    /// Projects to watch
    #[arg(long, num_args = 0.., value_delimiter = ' ')]
    projects: Vec<String>,
}

#[derive(Debug, Parser)]
struct ClapServiceArgs {
    /// Port for the service's server to listen on
    #[arg(long)]
    pub port: u16,
    /// Projects to watch
    #[arg(long, num_args = 0.., value_delimiter = ' ')]
    pub projects: Vec<String>,
}

pub struct Args {
    // pub mode: super::config::Mode,
    pub command: Option<Command>,
    pub port: u16,
    pub projects: Vec<String>,
}

impl Args {
    pub fn get(mode: super::config::Mode) -> Args {
        let (command_opt, port, projects) = match mode {
            super::config::Mode::App => {
                let args = ClapAppArgs::parse();
                (Some(args.command), args.port, args.projects)
            }
            super::config::Mode::Service => {
                let args = ClapServiceArgs::parse();
                (None, args.port, args.projects)
            }
        };
        Args {
            command: if let Some(command) = command_opt {
                Some(match command {
                    ClapCommand::Start => Command::Start,
                    ClapCommand::Stop => Command::Stop,
                    // ClapCommand::Restart => Command::Restart,
                    // ClapCommand::Status => Command::Status,
                    // ClapCommand::Logs => Command::Logs,
                    // ClapCommand::Config => Command::Config,
                    // ClapCommand::Help => Command::Help,
                })
            } else {
                None
            },
            port,
            projects,
        }
    }
}
