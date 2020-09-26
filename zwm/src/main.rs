extern crate ctrlc;
extern crate x11rb;
extern crate zwm;

use std::process;

use zwm::wm::*;

fn main() {
    process::Command::new("feh")
        .arg("--bg-fill")
        .arg("/home/doubleleft/.config/i3/Ryan.jpg")
        .env("DISPLAY", ":0")
        .spawn()
        .unwrap();

    let mut wm = WindowManager::<x11rb::rust_connection::RustConnection>::new(":0").unwrap();
    wm.run();
}
