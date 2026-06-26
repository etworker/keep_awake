#[macro_use]
mod lang;
mod config;
mod inhibit;
mod mouse;
mod tray;

use config::{Config, Mode};
use inhibit::InhibitGuard;
use mouse::MouseJiggler;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tray_icon::menu::MenuEvent;
use tray_icon::{MouseButton, TrayIconEvent};

fn main() {
    lang::detect();

    let config = Arc::new(Mutex::new(Config::load()));
    let enabled = Arc::new(AtomicBool::new(config.lock().unwrap().enabled));
    let mouse_mode = Arc::new(AtomicBool::new(
        matches!(config.lock().unwrap().mode, Mode::Mouse),
    ));

    let guard: Arc<Mutex<Option<InhibitGuard>>> = Arc::new(Mutex::new(None));
    let jiggler: Arc<Mutex<Option<MouseJiggler>>> = Arc::new(Mutex::new(None));

    let (handle, toggle_id, mode_id, autostart_id, quit_id) = {
        let cfg = config.lock().unwrap();
        tray::setup(&cfg)
    };

    {
        let cfg = config.lock().unwrap();
        if cfg.enabled {
            activate(&cfg, &guard, &jiggler, &enabled, &mouse_mode, &handle);
        }
    }

    let menu_rx = MenuEvent::receiver();
    let tray_rx = TrayIconEvent::receiver();

    loop {
        pump_messages();

        while let Ok(event) = menu_rx.try_recv() {
            let id = event.id();
            if *id == quit_id {
                guard.lock().unwrap().take();
                jiggler.lock().unwrap().take();
                return;
            } else if *id == toggle_id {
                toggle_enabled(&config, &guard, &jiggler, &enabled, &mouse_mode, &handle);
            } else if *id == mode_id {
                let mut cfg = config.lock().unwrap();
                cfg.mode = match cfg.mode {
                    Mode::Api => Mode::Mouse,
                    Mode::Mouse => Mode::Api,
                };
                cfg.save().ok();
                mouse_mode.store(matches!(cfg.mode, Mode::Mouse), Ordering::Relaxed);
                handle.set_mode_label(&cfg.mode);
                if cfg.enabled {
                    guard.lock().unwrap().take();
                    jiggler.lock().unwrap().take();
                    activate(&cfg, &guard, &jiggler, &enabled, &mouse_mode, &handle);
                }
            } else if *id == autostart_id {
                let mut cfg = config.lock().unwrap();
                cfg.autostart = !cfg.autostart;
                cfg.save().ok();
                handle.set_autostart_checked(cfg.autostart);
                apply_autostart(cfg.autostart);
            }
        }

        while let Ok(event) = tray_rx.try_recv() {
            if matches!(event, TrayIconEvent::Click { button: MouseButton::Left, .. }) {
                toggle_enabled(&config, &guard, &jiggler, &enabled, &mouse_mode, &handle);
            }
        }

        std::thread::sleep(Duration::from_millis(10));
    }
}

fn toggle_enabled(
    config: &Arc<Mutex<Config>>,
    guard: &Arc<Mutex<Option<InhibitGuard>>>,
    jiggler: &Arc<Mutex<Option<MouseJiggler>>>,
    enabled: &Arc<AtomicBool>,
    mouse_mode: &Arc<AtomicBool>,
    handle: &tray::TrayHandle,
) {
    let mut cfg = config.lock().unwrap();
    let new = !cfg.enabled;
    cfg.enabled = new;
    cfg.save().ok();
    enabled.store(new, Ordering::Relaxed);

    if new {
        let ok = activate(&cfg, guard, jiggler, enabled, mouse_mode, handle);
        if !ok {
            cfg.enabled = false;
            cfg.save().ok();
            enabled.store(false, Ordering::Relaxed);
            handle.set_toggle_checked(false);
            return;
        }
    } else {
        guard.lock().unwrap().take();
        jiggler.lock().unwrap().take();
    }
    handle.set_toggle_checked(new);
}

fn activate(
    cfg: &Config,
    guard: &Arc<Mutex<Option<InhibitGuard>>>,
    jiggler: &Arc<Mutex<Option<MouseJiggler>>>,
    enabled: &Arc<AtomicBool>,
    _mouse_mode: &Arc<AtomicBool>,
    _handle: &tray::TrayHandle,
) -> bool {
    match cfg.mode {
        Mode::Api => {
            let g = InhibitGuard::acquire();
            if g.is_none() {
                return false;
            }
            *guard.lock().unwrap() = g;
            true
        }
        Mode::Mouse => {
            let interval = cfg.interval_secs;
            *jiggler.lock().unwrap() = Some(MouseJiggler::start(interval, enabled.clone()));
            true
        }
    }
}

fn apply_autostart(enabled: bool) {
    if enabled {
        if let Ok(exe) = std::env::current_exe() {
            auto_start::enable(&exe);
        }
    } else {
        auto_start::disable();
    }
}

#[cfg(windows)]
fn pump_messages() {
    use std::ffi::c_void;
    #[repr(C)]
    struct MSG {
        hwnd: *mut c_void,
        message: u32,
        w_param: usize,
        l_param: isize,
        time: u32,
        pt: POINT,
    }
    #[repr(C)]
    struct POINT { x: i32, y: i32 }
    const PM_REMOVE: u32 = 1;
    #[link(name = "user32")]
    unsafe extern "system" {
        fn PeekMessageW(msg: *mut MSG, hwnd: *mut c_void, msg_filter_min: u32, msg_filter_max: u32, remove_msg: u32) -> i32;
        fn TranslateMessage(msg: *const MSG) -> i32;
        fn DispatchMessageW(msg: *const MSG) -> i32;
    }
    unsafe {
        let mut msg = std::mem::zeroed::<MSG>();
        while PeekMessageW(&mut msg, std::ptr::null_mut(), 0, 0, PM_REMOVE) != 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}

#[cfg(not(windows))]
fn pump_messages() {}

mod auto_start {
    pub fn enable(exe: &std::path::Path) {
        #[cfg(windows)]
        {
            let _ = std::process::Command::new("reg")
                .args([
                    "add",
                    "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Run",
                    "/v",
                    "keep-awake",
                    "/t",
                    "REG_SZ",
                    "/d",
                    &exe.display().to_string(),
                    "/f",
                ])
                .output();
        }
        #[cfg(target_os = "macos")]
        {
            let plist = format!(
                r#"<?xml version="1.0"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0"><dict>
<key>Label</key><string>com.keep-awake</string>
<key>ProgramArguments</key><array><string>{}</string></array>
<key>RunAtLoad</key><true/>
</dict></plist>"#,
                exe.display()
            );
            if let Some(path) = dirs::home_dir()
                .map(|p| p.join("Library/LaunchAgents/com.keep-awake.plist"))
            {
                let _ = std::fs::write(&path, plist);
            }
        }
        #[cfg(target_os = "linux")]
        {
            let desktop = format!(
                "[Desktop Entry]\nType=Application\nName=Keep Awake\nExec={}\nX-GNOME-Autostart-enabled=true\n",
                exe.display()
            );
            if let Some(path) = dirs::config_dir()
                .map(|p| p.join("autostart/keep-awake.desktop"))
            {
                let _ = std::fs::write(&path, desktop);
            }
        }
    }

    pub fn disable() {
        #[cfg(windows)]
        {
            let _ = std::process::Command::new("reg")
                .args([
                    "delete",
                    "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Run",
                    "/v",
                    "keep-awake",
                    "/f",
                ])
                .output();
        }
        #[cfg(target_os = "macos")]
        {
            if let Some(path) = dirs::home_dir()
                .map(|p| p.join("Library/LaunchAgents/com.keep-awake.plist"))
            {
                let _ = std::fs::remove_file(&path);
            }
        }
        #[cfg(target_os = "linux")]
        {
            if let Some(path) = dirs::config_dir()
                .map(|p| p.join("autostart/keep-awake.desktop"))
            {
                let _ = std::fs::remove_file(&path);
            }
        }
    }
}
