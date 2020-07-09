extern crate x11rb;
extern crate zwm;
extern crate ctrlc;

use x11rb::connection::Connection;

use zwm::wm::*;
use std::process::exit;

static mut EXIT_WM: bool = false;

fn main() {
    let (conn, screen_num) = x11rb::connect(None).unwrap();
    let conn = &conn;

    let screen = &conn.setup().roots[screen_num];
    println!("Root window: {}", screen.root);

    let mut wm = WindowManager::new(conn, screen_num).unwrap();
    wm.become_wm().unwrap();
    wm.scan_windows().unwrap();
    conn.flush().unwrap();

    ctrlc::set_handler(|| unsafe { EXIT_WM = true }).unwrap();

    loop {
        unsafe {
            if EXIT_WM || wm.should_exit() {
                println!("exiting zwm");
                wm.destroy();
                exit(0);
            }
        }

        wm.refresh().unwrap();
        conn.flush().unwrap();
        let event = conn.wait_for_event();
        if let Ok(event) = event {
            println!("Got event: {:?}", event);
            wm.handle_event(event).unwrap();
        } else {
            eprintln!("Error: {:?}", event.unwrap_err());
        }
    }
}
