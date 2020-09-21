extern crate ctrlc;
extern crate x11rb;
extern crate zwm;

use std::process;

use x11rb::connection::Connection;

use zwm::wm::*;

static mut EXIT_WM: bool = false;

fn main() -> std::io::Result<()> {

    process::Command::new("feh")
        .arg("--bg-fill")
        .arg("/home/doubleleft/.config/i3/Ryan.jpg")
        .env("DISPLAY", ":0")
        .spawn()?;

    let (conn, screen_num) = x11rb::connect(Some(":0")).unwrap();
    let conn = &conn;

    let screen = &conn.setup().roots[screen_num];
    println!("Root window: {}, screen_num {}", screen.root, screen_num);

    let mut wm = WindowManager::new(conn, screen_num).unwrap();

    ctrlc::set_handler(|| unsafe { EXIT_WM = true }).unwrap();

    loop {
        unsafe {
            if EXIT_WM || wm.should_exit() {
                return Ok(());
            }
        }

        wm.refresh().unwrap();
        let event = conn.wait_for_event();
        if let Ok(event) = event {
            println!("Got event: {:?}", event);
            wm.handle_event(event).unwrap();
        } else {
            eprintln!("Error: {:?}", event.unwrap_err());
        }
    }
}
