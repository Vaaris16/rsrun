use serde::Deserialize;

mod on_change;
mod startup;
mod watch_dir;

mod errors;

use watch_dir::watch_dir::watch_directory;

fn main() {
    if let Err(e) = watch_directory("./") {
        print!("Errror: {}", e);
    }
}

#[derive(Debug, Deserialize)]
struct RsrunConfig {
    commands: Commands,
    ignore: Ignore,
}

#[derive(Debug, Deserialize)]
struct Commands {
    command: String,
    args: String,
}

#[derive(Debug, Deserialize)]
struct Ignore {
    item: String,
}
