mod dispatcher;
mod headless;
mod keyboard;
mod platform;
mod system_notifications;
mod text_system;
mod wayland;
mod xdg_desktop_portal;

pub use dispatcher::*;
pub(crate) use headless::*;
pub(crate) use keyboard::*;
pub(crate) use platform::*;
pub(crate) use text_system::*;
pub(crate) use wayland::*;

use std::rc::Rc;

/// Returns the default platform implementation for the current OS.
pub fn current_platform(headless: bool) -> Rc<dyn gpui::Platform> {
    if headless {
        return Rc::new(LinuxPlatform {
            inner: HeadlessClient::new(),
        });
    }

    match gpui::guess_compositor() {
        "Headless" => Rc::new(LinuxPlatform {
            inner: HeadlessClient::new(),
        }),
        _ => Rc::new(LinuxPlatform {
            inner: WaylandClient::new(),
        }),
    }
}
