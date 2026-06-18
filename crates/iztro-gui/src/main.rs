//! Entry point for the local iztro static chart GUI prototype.

fn main() -> iced::Result {
    prefer_xwayland_on_wsl();
    iztro_gui::run()
}

/// WSLg's Wayland connection is unstable with the Iced 0.13 software renderer,
/// while its XWayland endpoint is stable. Winit 0.30 chooses Wayland whenever
/// `WAYLAND_DISPLAY` or `WAYLAND_SOCKET` is present, so hide those selectors on
/// WSL when an X11 display is available.
#[cfg(target_os = "linux")]
fn prefer_xwayland_on_wsl() {
    if std::env::var_os("WSL_DISTRO_NAME").is_some() && std::env::var_os("DISPLAY").is_some() {
        // SAFETY: This binary calls the function at the start of `main`, before
        // Iced initializes or this program spawns any threads.
        unsafe {
            std::env::remove_var("WAYLAND_DISPLAY");
            std::env::remove_var("WAYLAND_SOCKET");
        }
    }
}

#[cfg(not(target_os = "linux"))]
fn prefer_xwayland_on_wsl() {}
