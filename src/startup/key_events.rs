use crate::on_change::on_change::on_change;
use std::io::{self, BufRead};

pub fn key_press() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line.unwrap().trim().to_string().as_str() {
            "r" => {
                println!("\x1b[1;36mManual Reload...\x1b[0m");
                if let Err(e) = on_change() {
                    eprintln!("Error: {e}");
                }
            }
            "q" => std::process::exit(0),
            "h" => {
                println!("\x1b[1;33mKeybinds:\x1b[0m\r");
                println!("  \x1b[1mr + enter:\x1b[0m manual reload");
                println!("  \x1b[1mq + enter:\x1b[0m quit");
                println!("  \x1b[1mh + enter:\x1b[0m help");
            }
            _ => {}
        }
    }
}
