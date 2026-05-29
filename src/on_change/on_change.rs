use std::{
    path::Path,
    process::{Child, Command},
    sync::Mutex,
};

use crate::errors::app_errors::AppErrors;
use crate::on_change::read_config::read_config;

pub fn on_change() -> std::result::Result<(), AppErrors> {
    static CHILD: Mutex<Option<Child>> = Mutex::new(None);

    let config = Path::new("./rsrun.toml");

    if !config.exists() {
        AppErrors::NotFound {
            path: config.to_path_buf(),
        };
    }

    let parsed_config = read_config(config.to_path_buf())?;

    let mut child = CHILD.lock().unwrap();

    if let Some(ref mut prev) = *child {
        let _ = prev.kill();
    }
    let mut new_child = Command::new(parsed_config.commands.command)
        .arg(parsed_config.commands.args)
        .spawn()
        .expect("command failed");

    new_child.wait().ok();

    *child = Some(new_child);

    Ok(())
}
