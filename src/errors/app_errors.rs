use core::fmt;
use std::path::PathBuf;

#[derive(Debug)]
pub enum AppErrors {
    NotFound { path: PathBuf },
    FailedRead { path: PathBuf },
    FailedParsing { path: PathBuf },
}

impl fmt::Display for AppErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppErrors::NotFound { path } => write!(f, "Config file Not Found: {:?}", path),
            AppErrors::FailedRead { path } => write!(f, "Failed to read config: {:?}", path),
            AppErrors::FailedParsing { path } => write!(f, "Failed to parse config: {:?}", path),
        }
    }
}
