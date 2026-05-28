use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Result, Watcher};
use std::path::Path;
use std::sync::mpsc::channel;

fn main() {
    if let Err(e) = watch_directory("./") {
        print!("Error: {}", e);
    }
}

fn watch_directory(path: &str) -> Result<()> {
    let (tx, rx) = channel::<Result<Event>>();

    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

    watcher.watch(Path::new(path), RecursiveMode::Recursive)?;

    println!("Watching: {path}");

    for res in rx {
        println!("change detected")
    }

    Ok(())
}
