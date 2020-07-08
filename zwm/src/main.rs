extern crate x11rb;
extern crate zwm;
// extern crate ctrlc;

use x11rb::connection::Connection;

use zwm::wm::*;

fn main() {
    let (conn, screen_num) = x11rb::connect(None).unwrap();
    let conn = &conn;

    let screen = &conn.setup().roots[screen_num];
    println!("Root window: {}", screen.root);

    let mut wm_state = WindowManager::new(conn, screen_num).unwrap();
    wm_state.become_wm().unwrap();
    wm_state.scan_windows().unwrap();

    // ctrlc::set_handler(move || { wm_state.destroy(); });

    loop {
        wm_state.refresh().unwrap();
        conn.flush().unwrap();
        let event = conn.wait_for_event();
        println!("Got event: {:?}", event);
        match event {
            Ok(event) => {
                let res = wm_state.handle_event(event);
                if res. is_err() {
                    println!("Error: {:?}", res.unwrap_err());
                }
            },
            Err(err) => {
                println!("Error: {:?}", err);
            },
        }

        if wm_state.should_exit() {
            wm_state.destroy();
            return;
        }
    }
}
