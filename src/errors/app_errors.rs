use core::fmt;
use std::{fmt::write, path::PathBuf};

#[derive(Debug)]
pub enum AppErrors {
    NotFound { path: PathBuf },
    FailedRead { path: PathBuf },
    FailedParsing { path: PathBuf },
    NotFoundCommand,
    SpawnError,
}

impl fmt::Display for AppErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppErrors::NotFound { path } => write!(f, "Config file Not Found: {:?}", path),
            AppErrors::FailedRead { path } => write!(f, "Failed to read config: {:?}", path),
            AppErrors::FailedParsing { path } => write!(f, "Failed to parse config: {:?}", path),
            AppErrors::NotFoundCommand => write!(f, "Command not found"),
            AppErrors::SpawnError => write!(f, "failed to spawn command"),
        }
    }
}
