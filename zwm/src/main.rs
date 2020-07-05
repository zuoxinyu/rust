extern crate x11rb;
extern crate zwm;

use x11rb::connection::Connection;

use zwm::wm::*;

fn main() {
    let (conn, screen_num) = x11rb::connect(None).unwrap();

    let screen = &conn.setup().roots[screen_num];

    become_wm(&conn, screen).unwrap();

    let mut wm_state = WMState::new(&conn, screen_num).unwrap();
    wm_state.scan_windows().unwrap();

    loop {
        wm_state.refresh().unwrap();
        conn.flush().unwrap();

        let event = conn.wait_for_event().unwrap();
        println!("Got event: {:?}", event);
        let mut event_option = Some(event);
        while let Some(event) = event_option {
            // TODO: exit
            wm_state.handle_event(event).unwrap();
            event_option = conn.poll_for_event().unwrap();
        }
    }
}
