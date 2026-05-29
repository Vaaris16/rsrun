use figlet_rs::FIGlet;

pub fn startup_ui(path: &str) {
    let slant_font = FIGlet::slant().unwrap();

    println!("Watching: {}\r", path);
    println!("{}\r", slant_font.convert("RSRUN").unwrap());
    println!("Keybinds:\r");
    println!("  r + enter: manual reload");
    println!("  q + enter: quit");
    println!("  h + enter: help");
}
