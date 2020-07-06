extern crate x11rb;
extern crate zwm;
// extern crate ctrlc;

use x11rb::connection::Connection;

use zwm::wm::*;

fn main() {
    let (conn, screen_num) = x11rb::connect(None).unwrap();
    let conn = &conn;

    let screen = &conn.setup().roots[screen_num];

    become_wm(conn, screen).unwrap();

    let mut wm_state = WMState::new(conn, screen_num).unwrap();
    let res = wm_state.scan_windows();
    if res.is_err() {
        println!("Error: {:?}", res.unwrap_err());
    }

    // ctrlc::set_handler(move || { wm_state.destroy(); });

    loop {
        wm_state.refresh().unwrap();
        conn.flush().unwrap();
        let event = conn.wait_for_event().unwrap();
        println!("Got event: {:?}", event);
        let mut option_event = Some(event);
        while let Some(event) = option_event {
            let res = wm_state.handle_event(event);
            if res.is_err() {
                println!("Error: {:?}", res.unwrap_err());
            }
            if wm_state.should_exit() {
                wm_state.destroy();
                return;
            }
            option_event = conn.poll_for_event().unwrap();
        }
    }
}
