extern crate x11rb;

use std::collections::HashMap;
use std::collections::HashSet;
use std::process::exit;

use x11rb::connection::Connection;
use x11rb::errors::{ReplyError, ReplyOrIdError};
use x11rb::protocol::randr;
use x11rb::protocol::xproto::*;
use x11rb::protocol::{Error, Event};
use x11rb::COPY_DEPTH_FROM_PARENT;
use x11rb::COPY_FROM_PARENT;
use x11rb::NONE;

use crate::action::Action;
use crate::container::Container;
use crate::window::ManagedWindow;

/// The state of the full WM
#[derive(Debug)]
pub struct WindowManager<T>
where
    T: Connection + ConnectionExt+randr::ConnectionExt
{
    conn: Box<T>,
    screen_num: usize,
    black_gc: Gcontext,
    windows: Vec<ManagedWindow>,
    managed_windows: HashMap<Window, Box<ManagedWindow>>,
    pending_expose: HashSet<Window>,
    wm_protocols: Atom,
    wm_delete_window: Atom,
    exit: bool,
    root: Window,
    screen_size: (u16, u16),
}

impl<T> WindowManager<T>
where
    T: Connection + ConnectionExt,
{
    pub fn new(
        address: &str,
    ) -> Result<WindowManager<impl Connection + ConnectionExt + randr::ConnectionExt>, ReplyOrIdError> {
        let (conn, screen_num) = x11rb::connect(Some(address)).unwrap();
        let conn = Box::new(conn);

        let black_gc = conn.generate_id()?;
        let font = conn.generate_id()?;
        let screen = conn.setup().roots[screen_num].clone();

        //let monitors = &conn.randr_get_monitors(screen.root, true)?.reply()?;
        let monitors = randr::get_screen_info(conn.as_ref(), screen.root)?.reply();
        println!("monitors: {:?}", monitors);

        conn.open_font(font, b"9x15")?;

        let gc_aux = CreateGCAux::new()
            .graphics_exposures(0)
            .background(screen.white_pixel)
            .foreground(screen.black_pixel)
            .font(font);

        conn.create_gc(black_gc, screen.root, &gc_aux)?;
        conn.close_font(font)?;

        let wm_protocols = conn
            .as_ref()
            .intern_atom(false, b"WM_PROTOCOLS")?
            .reply()?
            .atom;
        let wm_delete_window = conn
            .as_ref()
            .intern_atom(false, b"WM_DELETE_WINDOW")?
            .reply()?
            .atom;

        let mut wm = WindowManager {
            conn,
            screen_num,
            black_gc,
            windows: Vec::default(),
            managed_windows: HashMap::new(),
            pending_expose: HashSet::default(),
            wm_protocols,
            wm_delete_window,
            exit: false,
            root: screen.root,
            screen_size: (screen.width_in_pixels as _, screen.height_in_pixels as _),
        };

        wm.become_wm().unwrap();
        wm.scan_windows().unwrap();
        // conn.flush().unwrap();

        Ok(wm)
    }

    /// Should kill myself
    pub fn set_exit(&mut self) {
        self.exit = true
    }

    /// Scan for already existing windows and manage them
    pub fn scan_windows(&mut self) -> Result<(), ReplyOrIdError> {
        // Get the already existing top-level windows.
        let tree = self.conn.query_tree(self.root)?.reply()?;
        // For each window, request its attributes and geometry *now*
        println!("number of exists win: {}", tree.children.len());
        let cookies: Vec<_> = tree
            .children
            .iter()
            .map(|&w| {
                (
                    w,
                    self.conn.get_window_attributes(w).unwrap().reply(),
                    self.conn.get_geometry(w).unwrap().reply(),
                    self.get_window_name(w).unwrap(),
                )
            })
            .filter(|t| t.1.is_ok() && t.2.is_ok())
            .collect();
        // Get the replies and manage windows
        for (win, attr, geom, name) in cookies {
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
        self.conn.flush().map_err(|e| e.into())
    }

    pub fn run(&mut self) -> () {
        loop {
            if self.exit {
                break;
            }
            self.refresh().unwrap();
            let event = self.conn.poll_for_event();
            // let event = self.conn.wait_for_event();
            if let Ok(event) = event {
                if let Some(event) = event {
                    println!("Got event: {:?}", event);
                    self.handle_event(event).unwrap();
                }
            } else {
                eprintln!("Error: {:?}", event.unwrap_err());
            }
        }
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

    /// Try to become the window manager. This causes an error if there is already another WM.
    fn become_wm(&mut self) -> Result<(), ReplyError> {
        let change = ChangeWindowAttributesAux::default().event_mask(
            EventMask::SubstructureRedirect
                | EventMask::EnterWindow
                | EventMask::KeyPress
                | EventMask::KeyRelease
                | EventMask::ButtonPress
                | EventMask::ButtonRelease,
        );
        let res = self
            .conn
            .change_window_attributes(self.root, &change)?
            .check();

        self.conn.grab_key(
            false,
            self.root,
            KeyButMask::Mod1,
            24,
            GrabMode::Async,
            GrabMode::Async,
        )?;
        if let Err(ReplyError::X11Error(Error::Access(_))) = res {
            eprintln!("Another WM is already running.");
            exit(1);
        } else {
            res
        }
    }

    /// Add a new window that should be managed by the WM
    fn manage_window(
        &mut self,
        win: Window,
        geom: &GetGeometryReply,
    ) -> Result<(), ReplyOrIdError> {
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
                    | EventMask::KeymapState,
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
        self.conn
            .reparent_window(win, frame_win, 0, ManagedWindow::TITLEBAR_HEIGHT as _)?;

        // Restore the event mask. (from i3 source CHILD_EVENT_MASK)
        let change_aux = change_aux.event_mask(
            (EventMask::PropertyChange | EventMask::StructureNotify | EventMask::FocusChange)
                & (!u32::from(EventMask::EnterWindow)),
        );
        self.conn.change_window_attributes(win, &change_aux)?;

        self.conn.map_window(win)?;
        self.conn.map_window(frame_win)?;

        // self.conn.grab_pointer(true, frame_win, EventMask::PointerMotion as u16, GrabMode::Async, GrabMode::Async, frame_win, NONE, CURRENT_TIME)?;

        self.conn.change_save_set(SetMode::Insert, win)?; // may be repeated with drop?

        self.windows.push(ManagedWindow::new(win, frame_win, geom));
        self.managed_windows
            .insert(win, Box::new(ManagedWindow::new(win, frame_win, geom)));

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
        self.conn.poly_arc(
            state.frame_window,
            self.black_gc,
            &[Arc {
                x: maximum_x,
                y: 0,
                width: ManagedWindow::TITLEBAR_HEIGHT,
                height: ManagedWindow::TITLEBAR_HEIGHT,
                angle1: 0,
                angle2: 360 << 6,
            }],
        )?;
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

    #[allow(dead_code)]
    fn find_window_by_event(&mut self, event: Event) -> Option<&Box<ManagedWindow>> {
        let w = match event {
            Event::ButtonPress(e) => Some(e.event),
            Event::ButtonRelease(e) => Some(e.event),
            Event::CirculateNotify(e) => Some(e.window),
            Event::CirculateRequest(e) => Some(e.window),
            Event::ClientMessage(e) => Some(e.window),
            Event::ColormapNotify(e) => Some(e.window),
            Event::ConfigureNotify(e) => Some(e.window),
            Event::ConfigureRequest(e) => Some(e.window),
            Event::CreateNotify(e) => Some(e.window),
            Event::DestroyNotify(e) => Some(e.window),
            Event::EnterNotify(e) => Some(e.event),
            Event::Expose(e) => Some(e.window),
            Event::FocusIn(e) => Some(e.event),
            Event::FocusOut(e) => Some(e.event),
            Event::GraphicsExposure(e) => Some(e.drawable),
            Event::GravityNotify(e) => Some(e.window),
            Event::KeyPress(e) => Some(e.event),
            Event::KeyRelease(e) => Some(e.event),
            Event::LeaveNotify(e) => Some(e.event),
            Event::MapNotify(e) => Some(e.window),
            Event::MapRequest(e) => Some(e.window),
            Event::MotionNotify(e) => Some(e.event),
            Event::NoExposure(e) => Some(e.drawable),
            Event::PropertyNotify(e) => Some(e.window),
            Event::ReparentNotify(e) => Some(e.window),
            Event::ResizeRequest(e) => Some(e.window),
            Event::SelectionClear(e) => Some(e.owner),
            Event::SelectionNotify(e) => Some(e.target),
            Event::SelectionRequest(e) => Some(e.target),
            Event::UnmapNotify(e) => Some(e.window),
            Event::VisibilityNotify(e) => Some(e.window),
            Event::DamageNotify(e) => Some(e.drawable),
            _ => None,
        };
        w.map_or_else(|| None, move |x| self.managed_windows.get(&x))
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
        if let Some(win) = self.find_window_by_id_mut(event.window) {
            win.render(Action::Destroy).unwrap();
        }
        Ok(())
    }

    fn handle_configure_request(&mut self, event: ConfigureRequestEvent) -> Result<(), ReplyError> {
        if let Some(win) = self.find_window_by_id_mut(event.window) {
            if event.value_mask & (u16::from(ConfigWindow::X) | u16::from(ConfigWindow::Y)) != 0 {
                win.render(Action::SetPosition(event.x, event.y)).unwrap();
            }
            if event.value_mask & (u16::from(ConfigWindow::Width) | u16::from(ConfigWindow::Height))
                != 0
            {
                win.render(Action::SetSize(event.width, event.height))
                    .unwrap();
            }
        }
        Ok(())
    }

    fn handle_map_request(&mut self, event: MapRequestEvent) -> Result<(), ReplyOrIdError> {
            let geo = &self.conn.get_geometry(event.window)?.reply()?;
        self.manage_window( event.window, geo)
    }

    fn handle_expose(&mut self, event: ExposeEvent) -> Result<(), ReplyError> {
        self.pending_expose.insert(event.window);
        Ok(())
    }

    fn handle_enter(&mut self, event: EnterNotifyEvent) -> Result<(), ReplyError> {
        if let Some(win) = self.find_window_by_id_mut(event.child) {
            win.render(Action::Focus).unwrap();
        }
        Ok(())
    }

    fn handle_button_press(&mut self, event: ButtonPressEvent) -> Result<(), ReplyError> {
        if let Some(win) = self.find_window_by_id_mut(event.event) {
            win.render(Action::SetPressing(event.event_x, event.event_y))
                .unwrap();
        }
        Ok(())
    }

    fn handle_button_release(&mut self, event: ButtonReleaseEvent) -> Result<(), ReplyError> {
        if let Some(win) = self.find_window_by_id_mut(event.event) {
            win.render(Action::MouseRelease(event.event_x, event.event_y))
                .unwrap();
        }

        Ok(())
    }

    fn handle_mouse_move(&mut self, event: MotionNotifyEvent) -> Result<(), ReplyError> {
        if let Some(state) = self.find_window_by_id_mut(event.event) {
            state
                .render(Action::Move(event.root_x, event.root_y))
                .unwrap();
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

impl<T> Drop for WindowManager<T>
where
    T: Connection,
{
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
