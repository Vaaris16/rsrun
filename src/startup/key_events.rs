use crate::on_change::on_change::on_change;
use std::io::{self, BufRead};

pub fn key_press() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line.unwrap().trim().to_string().as_str() {
            "r" => {
                println!("Manual Reload...");
                if let Err(e) = on_change() {
                    eprintln!("Error: {e}");
                }
            }
            "q" => std::process::exit(0),
            "h" => {
                println!("  r + enter: manual reload");
                println!("  q + enter: quit");
                println!("  h + enter: help");
            }
            _ => {}
        }
    }
}
