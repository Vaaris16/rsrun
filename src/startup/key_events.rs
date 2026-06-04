use crate::on_change::on_change::on_change;
use std::io::{self};

pub fn key_press() {
    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
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
