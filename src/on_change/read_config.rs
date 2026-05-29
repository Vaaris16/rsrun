use std::{fs, path::PathBuf};

use crate::{RsrunConfig, errors::app_errors::AppErrors};

pub fn read_config(path: PathBuf) -> std::result::Result<RsrunConfig, AppErrors> {
    let config_str =
        fs::read_to_string(&path).map_err(|_| AppErrors::FailedRead { path: path.clone() })?;

    let config: RsrunConfig =
        toml::from_str(&config_str).map_err(|_| AppErrors::FailedParsing { path: path.clone() })?;

    Ok(config)
}
