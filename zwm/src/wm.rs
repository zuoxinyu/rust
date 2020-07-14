// A very simple reparenting window manager.
// This WM does NOT follow ICCCM!

extern crate x11rb;

use std::collections::HashSet;
use std::process::exit;

use x11rb::COPY_FROM_PARENT;
use x11rb::NONE;
use x11rb::protocol::randr;
use x11rb::connection::Connection;
use x11rb::errors::{ReplyError, ReplyOrIdError};
use x11rb::protocol::xproto::*;
use x11rb::protocol::{Error, Event};
use x11rb::{COPY_DEPTH_FROM_PARENT, CURRENT_TIME};
use x11rb::wrapper::ConnectionExt;

use crate::window::{ManagedWindow, ButtonPos};

/// The state of the full WM
#[derive(Debug)]
pub struct WindowManager<'a, C: Connection + ConnectionExt> {
    conn: &'a C,
    screen_num: usize,
    black_gc: Gcontext,
    windows: Vec<ManagedWindow>,
    pending_expose: HashSet<Window>,
    wm_protocols: Atom,
    wm_delete_window: Atom,
    exit: bool,
    root: Window,
    screen_size: (u16, u16),
}

impl<'a, C: Connection + ConnectionExt + randr::ConnectionExt> WindowManager<'a, C> {
    pub fn new(conn: &'a C, screen_num: usize) -> Result<WindowManager<'a, C>, ReplyOrIdError> {
        let screen = &conn.setup().roots[screen_num];
        let black_gc = conn.generate_id()?;
        let font = conn.generate_id()?;

        let monitors = conn.randr_get_monitors(screen.root, true)?.reply()?;
        println!("monitors: {:?}", monitors);

        conn.open_font(font, b"9x15")?;

        let gc_aux = CreateGCAux::new()
            .graphics_exposures(0)
            .background(screen.white_pixel)
            .foreground(screen.black_pixel)
            .font(font);

        conn.create_gc(black_gc, screen.root, &gc_aux)?;
        conn.close_font(font)?;

        let wm_protocols = conn.intern_atom(false, b"WM_PROTOCOLS")?;
        let wm_delete_window = conn.intern_atom(false, b"WM_DELETE_WINDOW")?;

        Ok(WindowManager {
            conn,
            screen_num,
            black_gc,
            windows: Vec::default(),
            pending_expose: HashSet::default(),
            wm_protocols: wm_protocols.reply()?.atom,
            wm_delete_window: wm_delete_window.reply()?.atom,
            exit: false,
            root: screen.root,
            screen_size: (screen.width_in_pixels as _, screen.height_in_pixels as _),
        })
    }


    /// Should kill myself
    pub fn should_exit(&self) -> bool { self.exit }

    /// Try to become the window manager. This causes an error if there is already another WM.
    pub fn become_wm(&mut self) -> Result<(), ReplyError> {
        let change = ChangeWindowAttributesAux::default().event_mask(
            EventMask::SubstructureRedirect
                | EventMask::EnterWindow
                | EventMask::ButtonPress
                | EventMask::ButtonRelease
                | EventMask::KeyPress
                | EventMask::KeyRelease,
        );
        let res = self.conn.change_window_attributes(self.root, &change)?.check();
        if let Err(ReplyError::X11Error(Error::Access(_))) = res {
            eprintln!("Another WM is already running.");
            exit(1);
        } else {
            res
        }
    }

    /// Scan for already existing windows and manage them
    pub fn scan_windows(&mut self) -> Result<(), ReplyOrIdError> {
        // Get the already existing top-level windows.
        let tree = self.conn.query_tree(self.root)?.reply()?;

        // For each window, request its attributes and geometry *now*
        let mut cookies = Vec::with_capacity(tree.children.len());
        println!("number of exists win: {}", tree.children.len());
        for win in tree.children {
            let attr = self.conn.get_window_attributes(win)?;
            let geom = self.conn.get_geometry(win)?;
            let name = self.get_window_name(win)?;

            cookies.push((win, attr, geom, name));
        }
        // Get the replies and manage windows
        for (win, attr, geom, name) in cookies {
            let (attr, geom) = (attr.reply(), geom.reply());
            if attr.is_err() || geom.is_err() {
                continue;
            }

            let (attr, geom) = (attr.unwrap(), geom.unwrap());

            println!(
                "WINID: {:?}, NAME: {:?}, OVERRIDE_REDIRECT: {:?}, MAP_STATE: {:?}, CLASS: {:?}",
                win, name, attr.override_redirect, attr.map_state, attr.class,
            );

            if !attr.override_redirect && attr.map_state == MapState::Viewable {
                self.manage_window(win, &geom)?;
            }
        }

        Ok(())
    }

    /// Do all pending work that was queued while handling some events
    pub fn refresh(&mut self) -> Result<(), ReplyError> {
        while let Some(&win) = self.pending_expose.iter().next() {
            self.pending_expose.remove(&win);
            if let Some(state) = self.find_window_by_id(win) {
                if let Err(err) = self.redraw_titlebar(state) {
                    eprintln!(
                        "Error while redrawing window {:x?}: {:?}",
                        state.window, err
                    );
                }
            }
        }
        Ok(())
    }

    /// Handle the given event
    pub fn handle_event(&mut self, event: Event) -> Result<(), ReplyOrIdError> {
        match event {
            Event::UnmapNotify(event) => self.handle_unmap_notify(event)?,
            Event::ConfigureRequest(event) => self.handle_configure_request(event)?,
            Event::MapRequest(event) => self.handle_map_request(event)?,
            Event::Expose(event) => self.handle_expose(event)?,
            Event::EnterNotify(event) => self.handle_enter(event)?,
            Event::ButtonPress(event) => self.handle_button_press(event)?,
            Event::ButtonRelease(event) => self.handle_button_release(event)?,
            Event::MotionNotify(event) => self.handle_mouse_move(event)?,
            Event::KeyPress(event) => self.handle_key_press(event)?,
            _ => {}
        }
        Ok(())
    }

    /// Add a new window that should be managed by the WM
    fn manage_window(&mut self, win: Window, geom: &GetGeometryReply) -> Result<(), ReplyOrIdError> {
        println!("Managing window {:?}", win);
        let screen = &self.conn.setup().roots[self.screen_num];
        assert!(self.find_window_by_id(win).is_none());
        // Clear the event mask of the window to avoid the unmapped event during re-parenting.
        let change_aux = ChangeWindowAttributesAux::default().event_mask(NONE);
        self.conn.change_window_attributes(win, &change_aux)?;

        let frame_win = self.conn.generate_id()?;
        let win_aux = CreateWindowAux::new()
            .event_mask(
                EventMask::Exposure
                    | EventMask::ButtonPress
                    | EventMask::ButtonRelease
                    | EventMask::PointerMotion
                    | EventMask::EnterWindow
                    | EventMask::LeaveWindow
                    | EventMask::KeyPress
                    | EventMask::KeyRelease
                    | EventMask::KeymapState
            )
            .background_pixel(screen.white_pixel);

        self.conn.create_window(
            COPY_DEPTH_FROM_PARENT,
            frame_win,
            screen.root,
            geom.x,
            geom.y,
            geom.width,
            geom.height + ManagedWindow::TITLEBAR_HEIGHT,
            1,
            WindowClass::InputOutput,
            COPY_FROM_PARENT,
            &win_aux,
        )?;

        println!("frame_win: {}", frame_win);
        self.conn.reparent_window(win, frame_win, 0, ManagedWindow::TITLEBAR_HEIGHT as _)?;

        // Restore the event mask. (from i3 source CHILD_EVENT_MASK)
        let change_aux = change_aux.event_mask((EventMask::PropertyChange|EventMask::StructureNotify|EventMask::FocusChange) & (!u32::from(EventMask::EnterWindow)));
        self.conn.change_window_attributes(win, &change_aux)?;

        self.conn.map_window(win)?;
        self.conn.map_window(frame_win)?;

        self.conn.change_save_set(SetMode::Insert, win)?; // may be repeated with drop?

        self.windows.push(ManagedWindow::new(win, frame_win, geom));

        Ok(())
    }

    /// Draw the titlebar of a window
    fn redraw_titlebar(&self, state: &ManagedWindow) -> Result<(), ReplyError> {
        let close_x = state.close_x_position();
        let maximum_x = state.maximum_x_position();
        let minimum_x = state.minimum_x_position();
        self.conn.poly_line(
            CoordMode::Origin,
            state.frame_window,
            self.black_gc,
            &[
                Point { x: close_x, y: 0 },
                Point {
                    x: state.width as _,
                    y: ManagedWindow::TITLEBAR_HEIGHT as _,
                },
            ],
        )?;
        self.conn.poly_line(
            CoordMode::Origin,
            state.frame_window,
            self.black_gc,
            &[
                Point {
                    x: close_x,
                    y: ManagedWindow::TITLEBAR_HEIGHT as _,
                },
                Point {
                    x: state.width as _,
                    y: 0,
                },
            ],
        )?;
        self.conn.poly_arc(state.frame_window, self.black_gc, &[
            Arc {
                x: maximum_x,
                y: 0,
                width: ManagedWindow::TITLEBAR_HEIGHT,
                height: ManagedWindow::TITLEBAR_HEIGHT,
                angle1: 0,
                angle2: 360 << 6,
            },
        ])?;
        self.conn.poly_line(
            CoordMode::Origin,
            state.frame_window,
            self.black_gc,
            &[
                Point {
                    x: minimum_x,
                    y: (ManagedWindow::TITLEBAR_HEIGHT / 2) as _,
                },
                Point {
                    x: minimum_x + (ManagedWindow::TITLEBAR_HEIGHT as i16),
                    y: (ManagedWindow::TITLEBAR_HEIGHT / 2) as _,
                },
            ],
        )?;
        let reply = self
            .conn
            .get_property(
                false,
                state.window,
                AtomEnum::WM_NAME,
                AtomEnum::STRING,
                0,
                std::u32::MAX,
            )?
            .reply()?;
        self.conn
            .image_text8(state.frame_window, self.black_gc, 10, 10, &reply.value)?;
        Ok(())
    }

    fn find_window_by_id(&self, win: Window) -> Option<&ManagedWindow> {
        self.windows
            .iter()
            .find(|state| state.window == win || state.frame_window == win)
    }

    fn find_window_by_id_mut(&mut self, win: Window) -> Option<&mut ManagedWindow> {
        self.windows
            .iter_mut()
            .find(|state| state.window == win || state.frame_window == win)
    }

    fn get_window_name(&self, win: Window) -> Result<String, ReplyError> {
        let prop = self.conn.get_property(
            false,
            win,
            AtomEnum::WM_NAME,
            AtomEnum::STRING,
            0,
            std::u32::MAX,
        )?;
        let value = prop.reply()?.value;
        let name = String::from_utf8(value).unwrap();
        Ok(name)
    }

    fn handle_unmap_notify(&mut self, event: UnmapNotifyEvent) -> Result<(), ReplyError> {
        let conn = self.conn;
        self.windows.retain(|state| {
            if state.window != event.window {
                return true;
            }
            conn.destroy_window(state.frame_window).unwrap();
            false
        });
        Ok(())
    }

    fn handle_configure_request(&mut self, event: ConfigureRequestEvent) -> Result<(), ReplyError> {
        let mut aux = ConfigureWindowAux::default();
        if event.value_mask & u16::from(ConfigWindow::X) != 0 {
            aux = aux.x(i32::from(event.x));
        }
        if event.value_mask & u16::from(ConfigWindow::Y) != 0 {
            aux = aux.y(i32::from(event.y));
        }
        if event.value_mask & u16::from(ConfigWindow::Width) != 0 {
            aux = aux.width(u32::from(event.width));
        }
        if event.value_mask & u16::from(ConfigWindow::Height) != 0 {
            aux = aux.height(u32::from(event.height));
        }
        println!("Configure: {:?}", aux);
        self.conn.configure_window(event.window, &aux)?;
        Ok(())
    }

    fn handle_map_request(&mut self, event: MapRequestEvent) -> Result<(), ReplyOrIdError> {
        self.manage_window(
            event.window,
            &self.conn.get_geometry(event.window)?.reply()?,
        )
    }

    fn handle_expose(&mut self, event: ExposeEvent) -> Result<(), ReplyError> {
        self.pending_expose.insert(event.window);
        Ok(())
    }

    fn handle_enter(&mut self, event: EnterNotifyEvent) -> Result<(), ReplyError> {
        let window = if let Some(state) = self.find_window_by_id(event.child) {
            state.window
        } else {
            event.event
        };
        self.conn
            .set_input_focus(InputFocus::Parent, window, CURRENT_TIME)?;

        // put above
        let aux = ConfigureWindowAux::default().stack_mode(StackMode::Above);
        self.conn.configure_window(event.event, &aux)?;
        Ok(())
    }

    fn handle_button_press(&mut self, event: ButtonPressEvent) -> Result<(), ReplyError> {
        if let Some(state) = self.find_window_by_id_mut(event.event) {
            state.pressing = true;
            state.pressing_x = event.root_x;
            state.pressing_y = event.root_y;
        }
        Ok(())
    }

    fn handle_button_release(&mut self, event: ButtonReleaseEvent) -> Result<(), ReplyError> {
        let state: &mut ManagedWindow;
        let conn = self.conn;
        let screen_size = self.screen_size;
        let (delete_atom, protocoal_atom) = (self.wm_delete_window, self.wm_protocols);

        if let Some(s) = self.find_window_by_id_mut(event.event) {
            state = s;
            state.pressing = false;
        } else {
            return Ok(());
        }

        match state.on_button(event.event_x, event.event_y) {
            ButtonPos::Close => {
                let data = [delete_atom, 0, 0, 0, 0];
                let event = ClientMessageEvent {
                    response_type: CLIENT_MESSAGE_EVENT,
                    format: 32,
                    sequence: 0,
                    window: state.window,
                    type_: protocoal_atom,
                    data: data.into(),
                };
                conn.send_event(false, state.window, EventMask::NoEvent, &event)?;
            }
            ButtonPos::Maximum => {
                let aux = ConfigureWindowAux::default()
                    .width(screen_size.0 as u32)
                    .height(screen_size.1 as u32 - ManagedWindow::TITLEBAR_HEIGHT as u32);
                conn.configure_window(state.window, &aux)?;
                let aux = aux
                    .x(0)
                    .y(0)
                    .height(screen_size.1 as u32);
                conn.configure_window(state.frame_window, &aux)?;
            }
            ButtonPos::Minimum => {
                conn.unmap_window(state.frame_window)?;
            }
            _ => (),
        }

        Ok(())
    }

    fn handle_mouse_move(&mut self, event: MotionNotifyEvent) -> Result<(), ReplyError> {
        if let Some(state) = self.find_window_by_id(event.event) {
            if state.pressing {
                let x = state.x + event.root_x - state.pressing_x;
                let y = state.y + event.root_y - state.pressing_y;
                let mut aux = ConfigureWindowAux::new();
                aux.x = Some(x as i32);
                aux.y = Some(y as i32);
                self.conn.configure_window(state.frame_window, &aux)?;

                if let Some(state) = self.find_window_by_id_mut(event.event) {
                    state.x = x;
                    state.y = y;
                    state.pressing_x = event.root_x;
                    state.pressing_y = event.root_y;
                    println!("current window: {:?}", state);
                }
            }
        }
        Ok(())
    }

    fn handle_key_press(&mut self, event: KeyPressEvent) -> Result<(), ReplyError> {
        if event.detail == 24 {
            self.exit = true;
        }
        Ok(())
    }
}

impl<'a, C: Connection + ConnectionExt> Drop for WindowManager<'a, C> {
    fn drop(&mut self) {
        for it in self.windows.iter() {
            self.conn
                .reparent_window(it.window, self.root, it.x, it.y)
                .unwrap();
            self.conn.destroy_window(it.frame_window).unwrap();
        }

        self.conn.flush().unwrap();
        println!("dropped wm");
    }
}
