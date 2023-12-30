// #[path = "cli.rs"]
// mod cli;

use core::fmt;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::fs;

pub enum Mode {
    App,
    Service,
}

const DEFAULT_SERVICE_PORT: u16 = 8080;
const SERVICE_CONF_PATH: &str = "/var/lib/elastic_compose/config.yml";
const PROJ_CONF_FILENAME: &str = "ec_config.yml";

#[derive(Debug, Deserialize, Serialize)]
pub struct VCSConfig {
    pub url: String,
    pub branch: String,
    pub token: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ECRConfig {
    // prefix: String,
    // bucket: String,
    pub access_key: String,
    pub secret_key: String,
    // region: String,
    // secure: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum SourceEnum {
    ECR,
    VCS,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectConfig {
    pub source: SourceEnum,
    pub vcs: Option<VCSConfig>,
    pub ecr: Option<ECRConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Project {
    pub path: String,
    pub config: ProjectConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub port: u16,
    pub projects: Vec<Project>,
}

// pub enum ArgType<'a> {
//     App(&'a super::cmd::AppArgs),
//     Service(&'a super::cmd::ServiceArgs),
// }

pub enum ConfigError {
    // MissingPort,
    FailedToReadConfig(String),
    InvalidConfig(String),
    FailedToReadProjectConfig(String, String),
    InvalidProjectConfig(String, String),
    FailedToSaveConfig(String),
    FailedToDeleteConfig(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // ConfigError::MissingPort => {
            //     writeln!(
            //         f,
            //         "Missing 'port' arg, defaulting to port: {}",
            //         DEFAULT_SERVICE_PORT
            //     )
            // },
            ConfigError::FailedToReadConfig(err) => {
                writeln!(
                    f,
                    "Failed to read service config at: {}, err: {}",
                    SERVICE_CONF_PATH, err
                )
            }
            ConfigError::InvalidConfig(err) => {
                writeln!(
                    f,
                    "Invalid service config at: {}, err: {}",
                    SERVICE_CONF_PATH, err
                )
            }
            ConfigError::FailedToReadProjectConfig(path, err) => {
                writeln!(
                    f,
                    "Failed to read config at project: {}, err: {}",
                    path, err
                )
            }
            ConfigError::InvalidProjectConfig(path, err) => {
                writeln!(f, "Invalid config for project path: {}, err: {}", path, err)
            }
            ConfigError::FailedToSaveConfig(err) => {
                writeln!(
                    f,
                    "Failed to save config to: {}, err: {}",
                    SERVICE_CONF_PATH, err
                )
            }
            ConfigError::FailedToDeleteConfig(err) => {
                writeln!(
                    f,
                    "Failed to delete config from: {}, err: {}",
                    SERVICE_CONF_PATH, err
                )
            }
        }
    }
}

impl Config {
    fn read_service_config() -> Result<Config, ConfigError> {
        match fs::File::open(SERVICE_CONF_PATH) {
            Err(err) => return Err(ConfigError::FailedToReadConfig(err.to_string())),
            Ok(file) => match serde_yaml::from_reader(file) {
                Err(err) => return Err(ConfigError::InvalidConfig(err.to_string())),
                Ok(config) => Ok(config),
            },
        }
    }

    fn read_project_config(path: &String) -> Result<ProjectConfig, ConfigError> {
        match fs::File::open(format!("{}/{}", path, PROJ_CONF_FILENAME)) {
            Err(err) => {
                return Err(ConfigError::FailedToReadProjectConfig(
                    path.clone(),
                    err.to_string(),
                ))
            }
            Ok(file) => match serde_yaml::from_reader(file) {
                Err(err) => {
                    return Err(ConfigError::InvalidProjectConfig(
                        path.clone(),
                        err.to_string(),
                    ))
                }
                Ok(config) => Ok(config),
            },
        }
    }

    pub fn new(args: &super::cli::Args) -> Result<Config, ConfigError> {
        let mut projects: Vec<Project> = Vec::new();

        // Unwrap here is sure not to fail according to the app's flow
        for project_path in args.projects.as_ref().unwrap() {
            let project_config = match Config::read_project_config(project_path) {
                Err(err) => return Err(err),
                Ok(config) => config,
            };

            projects.push(Project {
                path: project_path.clone(),
                config: project_config,
            });
        }

        Ok(Config {
            port: match args.port {
                Some(port) => port,
                None => {
                    println!(
                        "Missing 'port' arg, defaulting to port: {}",
                        DEFAULT_SERVICE_PORT
                    );
                    DEFAULT_SERVICE_PORT
                }
            },
            projects,
        })
    }

    pub fn flush_to_disk(&self) -> Result<(), ConfigError> {
        match fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(SERVICE_CONF_PATH)
        {
            Err(err) => return Err(ConfigError::FailedToSaveConfig(err.to_string())),
            Ok(file) => match serde_yaml::to_writer(file, &self) {
                Err(err) => return Err(ConfigError::FailedToSaveConfig(err.to_string())),
                Ok(()) => Ok(()),
            },
        }
    }

    pub fn read_from_disk() -> Result<Config, ConfigError> {
        return Config::read_service_config();
    }

    pub fn delete_from_disk() -> Result<(), ConfigError> {
        match std::fs::remove_file(SERVICE_CONF_PATH) {
            Err(err) => return Err(ConfigError::FailedToDeleteConfig(err.to_string())),
            Ok(()) => Ok(()),
        }
    }
}
