use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Result, Watcher};
use std::path::Path;
use std::sync::mpsc::channel;

use crate::on_change::on_change::on_change;
use crate::on_change::read_config::read_config;
use crate::startup::key_events::key_press;
use crate::startup::startup_ui::startup_ui;

pub fn watch_directory(path: &str) -> Result<()> {
    let (tx, rx) = channel::<Result<Event>>();

    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

    watcher.watch(Path::new(path), RecursiveMode::Recursive)?;

    let config = match read_config(Path::new("./rsrun.toml").to_path_buf()) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Config error: {e}");
            std::process::exit(1);
        }
    };

    startup_ui(path);

    std::thread::spawn(|| {
        loop {
            key_press();
        }
    });

    for res in rx {
        if let Ok(event) = res {
            if event.paths.iter().any(|p| {
                config
                    .ignore
                    .iter()
                    .any(|ignore| p.to_string_lossy().contains(ignore))
            }) {
                continue;
            }

            if let Err(e) = on_change() {
                eprintln!("Error: {e}");
            }
        }
    }

    Ok(())
}
