use figlet_rs::FIGlet;

pub fn startup_ui(path: &str) {
    let slant_font = FIGlet::slant().unwrap();

    println!("\x1b[1;36mWatching:\x1b[0m {}\r", path);
    println!(
        "\x1b[1;32m{}\x1b[0m\r",
        slant_font.convert("RSRUN").unwrap().to_string()
    );
    println!("\x1b[1;33mKeybinds:\x1b[0m\r");
    println!("  \x1b[1mr + enter:\x1b[0m manual reload");
    println!("  \x1b[1mq + enter:\x1b[0m quit");
    println!("  \x1b[1mh + enter:\x1b[0m help");
}
